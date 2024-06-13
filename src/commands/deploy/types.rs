use candid::Principal;
use ic_utils::interfaces::management_canister::builders::CanisterSettings;

#[derive(Clone)]
pub enum ComponentsToDeploy {
    Single(String),
    Multiple(Vec<String>),
}

#[derive(candid::CandidType)]
pub struct UpdateSettingsArgs {
    pub canister_id: Principal,
    pub settings: CanisterSettings,
}
