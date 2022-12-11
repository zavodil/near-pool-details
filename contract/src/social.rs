use crate::*;

use near_sdk::serde_json::{Map, Value};

const GAS_FOR_SOCIAL_SET: Gas = Gas(Gas::ONE_TERA.0 * 15);
const NEAR_SOCIAL_ACCOUNT_ID: &str = "social.near";
const NEAR_SOCIAL_BADGE: &str = "staking-pool-owner";
const DEPOSIT_FOR_SOCIAL_SET: Balance = 50_000_000_000_000_000_000_000;

#[ext_contract(ext_social)]
pub trait ExtSocial {
    fn set(&mut self, data: Value);
}

#[near_bindgen]
impl PoolDetails {
    #[payable]
    pub fn set_near_social_badge(&mut self, pool_id: AccountId) -> Promise{
        assert!(env::attached_deposit() >= DEPOSIT_FOR_SOCIAL_SET, "Deposit {} required", DEPOSIT_FOR_SOCIAL_SET);
        lockup_whitelist::ext(AccountId::new_unchecked(WHITELIST_ACCOUNT_ID.to_string()))
            .with_static_gas(BASE)
            .is_whitelisted(
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
                    .on_get_owner_id_for_social_badge(
                        pool_id,
                    )
            )
    }

    #[private]
    pub fn on_get_owner_id_for_social_badge(
        &mut self,
        #[callback] is_whitelisted: bool,
        #[callback] owner_id: AccountId,
        pool_id: AccountId,
    ) {
        assert!(is_whitelisted, "Abort. Pool {} was not whitelisted.", pool_id);

        self.internal_social_set_badge(NEAR_SOCIAL_BADGE.to_string(), owner_id, pool_id);
    }

    #[payable]
    pub fn export_to_near_social(&mut self, pool_id: AccountId) -> Promise{
        staking_pool::ext(pool_id.clone())
            .with_static_gas(BASE)
            .get_owner_id()

            .then(
                ext_self_owner::ext(env::current_account_id())
                    .with_static_gas(CALLBACK)
                    .on_get_owner_id_for_export_to_near_social(
                        U128::from(env::attached_deposit()),
                        pool_id,
                    )
            )
    }

    #[private]
    pub fn on_get_owner_id_for_export_to_near_social(
        &mut self,
        #[callback] owner_id: AccountId,
        deposit: WrappedBalance,
        pool_id: AccountId,
    ) {
        let data = self.fields_by_pool.get(&pool_id).expect("Missing Pool Details");
        self.internal_social_set_profile_data(pool_id, data, owner_id, deposit.0);
    }
}

impl PoolDetails {
    pub fn internal_social_set_badge(&mut self, badge: String, account_id: AccountId, pool_id: AccountId) {
        let mut account_data: Map<String, Value> = Map::new();
        account_data.insert(account_id.to_string(), Value::String(pool_id.to_string()));

        let mut holder_data: Map<String, Value> = Map::new();
        holder_data.insert("holder".to_string(), Value::Object(account_data));

        let mut badge_data: Map<String, Value> = Map::new();
        badge_data.insert(badge, Value::Object(holder_data));

        let mut app_data: Map<String, Value> = Map::new();
        app_data.insert("badge".to_string(), Value::Object(badge_data));

        let mut data: Map<String, Value> = Map::new();
        data.insert(env::current_account_id().to_string(), Value::Object(app_data));

        ext_social::ext(AccountId::new_unchecked(NEAR_SOCIAL_ACCOUNT_ID.to_string()))
            .with_static_gas(GAS_FOR_SOCIAL_SET)
            .with_attached_deposit(DEPOSIT_FOR_SOCIAL_SET)
            .set(
                Value::Object(data)
            );
    }

    pub fn internal_social_set_profile_data(&mut self, pool_id: AccountId, data: HashMap<FieldName, FieldValue>, pool_owner_id: AccountId, deposit: Balance) {
        let mut profile: Map<String, Value> = Map::new();

        let linktree_fields: Vec<String> = vec!["twitter".to_string(), "github".to_string(), "telegram".to_string(), "website".to_string(), "email".to_string(), "discord".to_string()];
        let mut linktree: Map<String, Value> = Map::new();

        for field in linktree_fields {
            if let Some(item) = data.get(&field) {
                linktree.insert(field, Value::String(item.to_owned()));
            }
        }

        if let Some(url) = data.get(&"url".to_string()) {
            linktree.insert("website".to_string(), Value::String(url.to_owned()));
        }

        profile.insert("linktree".to_string(), Value::Object(linktree));

        let location_fields: Vec<String> = vec!["country_code".to_string(), "country".to_string(), "city".to_string()];
        let mut location: Map<String, Value> = Map::new();

        for field in location_fields {
            if let Some(item) = data.get(&field) {
                location.insert(field, Value::String(item.to_owned()));
            }
        }
        profile.insert("location".to_string(), Value::Object(location));


        if let Some(name) = data.get(&"name".to_string()) {
            profile.insert("name".to_string(), Value::String(name.to_owned()));
        }

        if let Some(description) = data.get(&"description".to_string()) {
            profile.insert("description".to_string(), Value::String(description.to_owned()));
        }

        if let Some(logo) = data.get(&"logo".to_string()) {
            let mut logo_url: Map<String, Value> = Map::new();
            logo_url.insert("url".to_string(), Value::String(logo.to_string()));

            profile.insert("image".to_string(), Value::Object(logo_url));
        }

        profile.insert("pool-owner".to_string(), Value::String(pool_owner_id.to_string()));

        let mut pool_data: Map<String, Value> = Map::new();
        pool_data.insert(pool_id.to_string(), Value::Object(profile));

        let mut data: Map<String, Value> = Map::new();
        data.insert(env::current_account_id().to_string(), Value::Object(pool_data));

        ext_social::ext(AccountId::new_unchecked(NEAR_SOCIAL_ACCOUNT_ID.to_string()))
            .with_static_gas(GAS_FOR_SOCIAL_SET)
            .with_attached_deposit(deposit)
            .set(
                Value::Object(data)
            );
    }
}