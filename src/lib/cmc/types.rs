// This is an experimental feature to generate Rust binding from Candid.
// You may want to manually adjust some of the types.
#![allow(dead_code, unused_imports)]
use candid::{self, CandidType, Decode, Deserialize, Encode, Principal};
use ic_agent::AgentError;
use ic_utils::{interfaces::WalletCanister, Argument};

#[derive(CandidType, Deserialize)]
pub enum ExchangeRateCanister {
    Set(Principal),
    Unset,
}

pub type AccountIdentifier = String;
#[derive(CandidType, Deserialize)]
pub struct CyclesCanisterInitPayload {
    pub exchange_rate_canister: Option<ExchangeRateCanister>,
    pub cycles_ledger_canister_id: Option<Principal>,
    pub last_purged_notification: Option<u64>,
    pub governance_canister_id: Option<Principal>,
    pub minting_account_id: Option<AccountIdentifier>,
    pub ledger_canister_id: Option<Principal>,
}

#[derive(CandidType, Deserialize)]
pub struct SubnetFilter {
    pub subnet_type: Option<String>,
}

#[derive(CandidType, Deserialize)]
pub enum SubnetSelection {
    Filter(SubnetFilter),
    Subnet { subnet: Principal },
}

#[derive(CandidType, Deserialize)]
pub enum LogVisibility {
    #[serde(rename = "controllers")]
    Controllers,
    #[serde(rename = "public")]
    Public,
}

#[derive(CandidType, Deserialize)]
pub struct CanisterSettings {
    pub freezing_threshold: Option<candid::Nat>,
    pub wasm_memory_threshold: Option<candid::Nat>,
    pub controllers: Option<Vec<Principal>>,
    pub reserved_cycles_limit: Option<candid::Nat>,
    pub log_visibility: Option<LogVisibility>,
    pub wasm_memory_limit: Option<candid::Nat>,
    pub memory_allocation: Option<candid::Nat>,
    pub compute_allocation: Option<candid::Nat>,
}

#[derive(CandidType, Deserialize)]
pub struct CreateCanisterArg {
    pub subnet_selection: Option<SubnetSelection>,
    pub settings: Option<CanisterSettings>,
    pub subnet_type: Option<String>,
}

#[derive(CandidType, Deserialize)]
pub enum CreateCanisterError {
    Refunded {
        create_error: String,
        refund_amount: candid::Nat,
    },
}

#[derive(CandidType, Deserialize)]
pub enum CreateCanisterResult {
    Ok(Principal),
    Err(CreateCanisterError),
}

#[derive(CandidType, Deserialize)]
pub struct IcpXdrConversionRate {
    pub xdr_permyriad_per_icp: u64,
    pub timestamp_seconds: u64,
}

// #[derive(CandidType, Deserialize)]
// pub struct IcpXdrConversionRateResponse {
//     pub certificate: serde_bytes::ByteBuf,
//     pub data: IcpXdrConversionRate,
//     pub hash_tree: serde_bytes::ByteBuf,
// }

#[derive(CandidType, Deserialize)]
pub struct PrincipalsAuthorizedToCreateCanistersToSubnetsResponse {
    pub data: Vec<(Principal, Vec<Principal>)>,
}

#[derive(CandidType, Deserialize)]
pub struct SubnetTypesToSubnetsResponse {
    pub data: Vec<(String, Vec<Principal>)>,
}

pub type BlockIndex = u64;
#[derive(CandidType, Deserialize)]
pub struct NotifyCreateCanisterArg {
    pub controller: Principal,
    pub block_index: BlockIndex,
    pub subnet_selection: Option<SubnetSelection>,
    pub settings: Option<CanisterSettings>,
    pub subnet_type: Option<String>,
}

#[derive(CandidType, Deserialize)]
pub enum NotifyError {
    Refunded {
        block_index: Option<BlockIndex>,
        reason: String,
    },
    InvalidTransaction(String),
    Other {
        error_message: String,
        error_code: u64,
    },
    Processing,
    TransactionTooOld(BlockIndex),
}

#[derive(CandidType, Deserialize)]
pub enum NotifyCreateCanisterResult {
    Ok(Principal),
    Err(NotifyError),
}

// pub type Memo = Option<serde_bytes::ByteBuf>;
// pub type Subaccount = Option<serde_bytes::ByteBuf>;
// #[derive(CandidType, Deserialize)]
// pub struct NotifyMintCyclesArg {
//     pub block_index: BlockIndex,
//     pub deposit_memo: Memo,
//     pub to_subaccount: Subaccount,
// }

#[derive(CandidType, Deserialize)]
pub struct NotifyMintCyclesSuccess {
    pub balance: candid::Nat,
    pub block_index: candid::Nat,
    pub minted: candid::Nat,
}

#[derive(CandidType, Deserialize)]
pub enum NotifyMintCyclesResult {
    Ok(NotifyMintCyclesSuccess),
    Err(NotifyError),
}

#[derive(CandidType, Deserialize)]
pub struct NotifyTopUpArg {
    pub block_index: BlockIndex,
    pub canister_id: Principal,
}

pub type Cycles = candid::Nat;
#[derive(CandidType, Deserialize)]
pub enum NotifyTopUpResult {
    Ok(Cycles),
    Err(NotifyError),
}

pub struct Service(pub Principal);
impl Service {
    pub async fn create_canister(
        &self,
        wallet: &WalletCanister<'_>,
        arg0: CreateCanisterArg,
        cycles: u128,
    ) -> Result<(CreateCanisterResult,), AgentError> {
        let arg = Argument::from_raw(Encode!(&arg0).unwrap());
        wallet
            .call128(self.0, "create_canister", arg, cycles)
            .call_and_wait()
            .await
    }
    // pub async fn get_build_metadata(&self) -> Result<(String,)> {
    //     ic_cdk::call(self.0, "get_build_metadata", ()).await
    // }
    // pub async fn get_icp_xdr_conversion_rate(&self) -> Result<(IcpXdrConversionRateResponse,)> {
    //     ic_cdk::call(self.0, "get_icp_xdr_conversion_rate", ()).await
    // }
    // pub async fn get_principals_authorized_to_create_canisters_to_subnets(
    //     &self,
    // ) -> Result<(PrincipalsAuthorizedToCreateCanistersToSubnetsResponse,)> {
    //     ic_cdk::call(
    //         self.0,
    //         "get_principals_authorized_to_create_canisters_to_subnets",
    //         (),
    //     )
    //     .await
    // }
    // pub async fn get_subnet_types_to_subnets(&self) -> Result<(SubnetTypesToSubnetsResponse,)> {
    //     ic_cdk::call(self.0, "get_subnet_types_to_subnets", ()).await
    // }
    // pub async fn notify_create_canister(
    //     &self,
    //     arg0: NotifyCreateCanisterArg,
    // ) -> Result<(NotifyCreateCanisterResult,)> {
    //     ic_cdk::call(self.0, "notify_create_canister", (arg0,)).await
    // }
    // pub async fn notify_mint_cycles(
    //     &self,
    //     arg0: NotifyMintCyclesArg,
    // ) -> Result<(NotifyMintCyclesResult,)> {
    //     ic_cdk::call(self.0, "notify_mint_cycles", (arg0,)).await
    // }
    // pub async fn notify_top_up(&self, arg0: NotifyTopUpArg) -> Result<(NotifyTopUpResult,)> {
    //     ic_cdk::call(self.0, "notify_top_up", (arg0,)).await
    // }
}
