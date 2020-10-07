// To conserve gas, efficient serialization is achieved through Borsh (http://borsh.io/)
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::wee_alloc;
use near_sdk::{env, ext_contract, near_bindgen};
use std::collections::HashMap;
use near_sdk::collections::UnorderedMap;
use near_sdk::Gas;


const BASE: Gas = 25_000_000_000_000;
pub const CALLBACK: Gas = BASE * 2;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

type PoolId = String;
type FieldName = String;
type FieldValue = String;
type FieldsStorageByPoolId = UnorderedMap<PoolId, HashMap<FieldName, FieldValue>>;

#[ext_contract(staking_pool)]
pub trait StakingPool {
    fn get_owner_id(&self) -> String;
}

#[ext_contract(ext_self_owner)]
pub trait ExtPoolDetails {
    fn on_get_owner_id(
        &mut self,
        #[callback] get_owner_id: String,
        staking_pool_account_id: String,
        pool_id: String,
        name: String,
        value: String,
    ) -> bool;
}

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct PoolDetails {
    fields_by_pool: FieldsStorageByPoolId
}

#[near_bindgen]
impl PoolDetails {
    pub fn update_field(&mut self, pool_id: String, name: String, value: String) -> bool {
        assert!(
            pool_id != "",
            "Abort. Pool ID is empty"
        );

        assert!(
            name != "",
            "Abort. Name is empty"
        );

        assert!(
            value != "",
            "Abort. Value is empty"
        );

        let _owner_account_id_to_compare: String = env::signer_account_id().clone();

        staking_pool::get_owner_id(
            &pool_id, 0, BASE)
            .then(ext_self_owner::on_get_owner_id(
                _owner_account_id_to_compare,
                pool_id,
                name,
                value,
                &env::current_account_id(),
                0,
                CALLBACK,
            ));

        true
    }


    pub fn get_all_fields(&self) -> HashMap<PoolId, HashMap<FieldName, FieldValue>> {
        self.fields_by_pool.iter().collect()
    }


    pub fn get_fields_by_pool(&self, pool_id: String) -> Option<HashMap<FieldName, FieldValue>> {
        self.fields_by_pool.get(&pool_id)
    }

    pub fn on_get_owner_id(
        &mut self,
        #[callback] owner_id: String,
        staking_pool_account_id: String,
        pool_id: String,
        name: String,
        value: String,
    ) -> bool {
        assert_self();

        assert!(
            owner_id == staking_pool_account_id,
            "You are not the owner of pool. Login as {} in order to update {}",
            owner_id,
            pool_id
        );

        env::log(format!("Field {} added for pool {}", name, pool_id).as_bytes());

        let mut fields = self.fields_by_pool.get(&pool_id).unwrap_or_default();
        fields.insert(name, value);

        self.fields_by_pool.insert(
            &pool_id,
            &fields);

        true
    }

    pub fn assert_self() {
        assert_eq!(env::predecessor_account_id(), env::current_account_id());
    }
}