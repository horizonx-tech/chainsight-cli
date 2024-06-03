use candid::Principal;
use ic_utils::interfaces::management_canister::builders::CanisterSettings;

#[derive(candid::CandidType)]
pub struct UpdateSettingsArgs {
    pub canister_id: Principal,
    pub settings: CanisterSettings,
}
