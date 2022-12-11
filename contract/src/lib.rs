use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::{env, ext_contract, near_bindgen, AccountId, Gas, Balance, log, BorshStorageKey, PanicOnDefault, Promise};
use std::collections::HashMap;
use near_sdk::json_types::U128;

mod social;
mod migrate;

const BASE: Gas = Gas(Gas::ONE_TERA.0 * 25);
pub const CALLBACK: Gas = Gas(Gas::ONE_TERA.0 * 50);

//  lockup-whitelist.near for Mainnet, whitelist.f863973.m0 for Testnet
pub const WHITELIST_ACCOUNT_ID: &str = "lockup-whitelist.near";


type PoolId = AccountId;
type FieldName = String;
type FieldValue = String;
type FieldsStorageByPoolId = UnorderedMap<PoolId, HashMap<FieldName, FieldValue>>;
type WrappedBalance = U128;

#[ext_contract(staking_pool)]
pub trait ExtStakingPool {
    fn get_owner_id(&self) -> String;
}

#[ext_contract(lockup_whitelist)]
pub trait ExtWhitelist {
    fn is_whitelisted(&self, staking_pool_account_id: AccountId) -> bool;
}

#[ext_contract(ext_self_owner)]
pub trait ExtPoolDetails {
    fn on_get_owner_id(
        &mut self,
        #[callback] get_owner_id: AccountId,
        current_user_account_id: AccountId,
        pool_id: AccountId,
        name: String,
        value: String,
    );

    fn on_get_owner_id_for_social_badge(
        &mut self,
        #[callback] get_owner_id: AccountId,
        pool_id: AccountId,
    );

    fn on_get_fields_by_pool(
        &mut self,
        #[callback] fields_by_pool: Option<HashMap<FieldName, FieldValue>>,
        #[callback] owner_id: AccountId,
        deposit: WrappedBalance,
        pool_id: AccountId,
    );

    fn on_get_owner_id_for_export_to_near_social(
        &mut self,
        #[callback] owner_id: AccountId,
        deposit: WrappedBalance,
        pool_id: AccountId,
    );
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct PoolDetails {
    fields_by_pool: FieldsStorageByPoolId,
}

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    FieldsByPool
}

#[near_bindgen]
impl PoolDetails {
    #[init]
    pub fn new() -> Self {
        Self {
            fields_by_pool: UnorderedMap::new(StorageKey::FieldsByPool)
        }
    }

    pub fn update_field(&mut self, pool_id: AccountId, name: String, value: String) -> Promise {
        assert_ne!(pool_id.to_string(), "", "Abort. Pool ID is empty");
        assert_ne!(name, "", "Abort. Name is empty");
        assert_ne!(value, "", "Abort. Value is empty");
        assert!(name.len() <= 100, "Abort. Name is longer then 100 characters");
        assert!(value.len() <= 4000, "Abort. Value is longer then 4000 characters");

        lockup_whitelist::ext(AccountId::new_unchecked(WHITELIST_ACCOUNT_ID.to_string()))
            .with_static_gas(BASE)
            .is_whitelisted(
                pool_id.clone()
            )
            .and(
                staking_pool::ext(pool_id.clone())
                    .with_static_gas(BASE)
                    .get_owner_id()
            )
            .then(
                ext_self_owner::ext(env::current_account_id())
                    .with_static_gas(CALLBACK)
                    .on_get_owner_id(
                        env::predecessor_account_id(),
                        pool_id,
                        name,
                        value,
                    )
            )
    }


    #[private]
    pub fn on_get_owner_id(
        &mut self,
        #[callback] is_whitelisted: bool,
        #[callback] owner_id: AccountId,
        current_user_account_id: AccountId,
        pool_id: AccountId,
        name: String,
        value: String,
    ) {
        assert!(
            is_whitelisted,
            "Abort. Pool {} was not whitelisted.",
            pool_id
        );

        assert_eq!(owner_id, current_user_account_id, "You are not the owner of pool. Login as {} in order to update {}. Your current account is {}", owner_id, pool_id, current_user_account_id);

        log!("Field {} added for pool {}", name, pool_id);

        let mut fields = self.fields_by_pool.get(&pool_id).unwrap_or_default();
        fields.insert(name, value);

        self.fields_by_pool.insert(&pool_id, &fields);
    }

    pub fn get_all_fields(&self, from_index: u64, limit: u64) -> HashMap<PoolId, HashMap<FieldName, FieldValue>> {
        let keys = self.fields_by_pool.keys_as_vector();
        let values = self.fields_by_pool.values_as_vector();

        (from_index..std::cmp::min(from_index + limit, self.fields_by_pool.len()))
            .map(|index| {
                let key = keys.get(index).unwrap();
                let value = values.get(index).unwrap();
                (key, value)
            })
            .collect()
    }

    pub fn get_num_pools(&self) -> u64 {
        self.fields_by_pool.len()
    }

    pub fn get_fields_by_pool(&self, pool_id: AccountId) -> Option<HashMap<FieldName, FieldValue>> {
        self.fields_by_pool.get(&pool_id)
    }

    pub fn get_fields_by_pools(&self, pool_ids: Vec<AccountId>) -> Vec<(AccountId, Option<HashMap<FieldName, FieldValue>>)> {
        pool_ids
            .into_iter()
            .map(|pool_id| {
                let data = self.fields_by_pool.get(&pool_id);
                (pool_id, data)
            })
            .collect()
    }
}