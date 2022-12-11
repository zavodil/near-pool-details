use crate::*;

pub const ORIGINAL_ACCOUNT_ID: &str = "name.near";

#[ext_contract(original_contract)]
pub trait ExtOriginalContact {
    fn get_fields_by_pool(&self, pool_id: AccountId) -> String;
}

#[near_bindgen]
impl PoolDetails {
    #[private]
    #[payable]
    pub fn migrate_pool_details (&mut self, pool_id: AccountId) -> Promise {
        original_contract::ext(AccountId::new_unchecked(ORIGINAL_ACCOUNT_ID.to_string()))
            .with_static_gas(BASE)
            .get_fields_by_pool(
                pool_id.clone(),
            )
            .and(
                staking_pool::ext(pool_id.clone())
                    .with_static_gas(BASE)
                    .get_owner_id()
            )
            .then(
                ext_self_owner::ext(env::current_account_id())
                    .with_static_gas(CALLBACK)
                    .on_get_fields_by_pool(
                        U128::from(env::attached_deposit()),
                        pool_id,
                    )
            )
    }

    #[private]
    pub fn on_get_fields_by_pool(
        &mut self,
        #[callback] fields_by_pool: Option<HashMap<FieldName, FieldValue>>,
        #[callback] owner_id: AccountId,
        deposit: WrappedBalance,
        pool_id: AccountId,
    ){
        if let Some(data) = fields_by_pool {
            self.fields_by_pool.insert(&pool_id, &data);
            self.internal_social_set_profile_data(pool_id, data, owner_id, deposit.0);
        }
        else {
            panic!("Missing Pool Details");
        }
    }
}