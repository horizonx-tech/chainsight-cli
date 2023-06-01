use anyhow::ensure;
use quote::quote;
use proc_macro2::TokenStream;

use crate::{types::ComponentType, lib::codegen::components::RelayerComponentManifest};

// temp
fn common_codes() -> TokenStream {
    quote! {
        use std::str::FromStr;
        use chainsight_cdk_macros::{manage_single_state, setup_func, timer_task_func, cross_canister_call_func, define_web3_ctx, define_transform_for_web3, define_get_ethereum_address, monitoring_canister_metrics, did_export};
        use ic_web3::types::{Address, U256};

        monitoring_canister_metrics!(60);
        define_web3_ctx!();
        define_transform_for_web3!();
        define_get_ethereum_address!();

        manage_single_state!("target_canister", String, false);
        manage_single_state!("target_addr", String, false);

        setup_func!({
            target_canister: String,
            target_addr: String,
            web3_ctx_param: Web3CtxParam
        });
    }
}

// temp
fn custom_codes(manifest: &RelayerComponentManifest) -> TokenStream {
    let mut method_ident = manifest.datasource.method.identifier.clone();
    // method.identifier: remove `()`
    method_ident.pop();
    method_ident.pop();

    // let interval = manifest.destinations[0].interval; // temp: use in function arguments

    quote! {
        ic_solidity_bindgen::contract_abi!("./src/relayer/abi/Oracle.json");

        #[derive(Debug, Clone, candid::CandidType, candid::Deserialize)]
        pub struct VirtualPrice {
            pub value: String,
            pub timestamp: u64,
        }
        type CallCanisterArgs = ();
        type CallCanisterResponse = VirtualPrice;
        cross_canister_call_func!(#method_ident, CallCanisterArgs, CallCanisterResponse); // TODO

        timer_task_func!("set_task", "sync", true);
        async fn sync() {
            let target_canister = candid::Principal::from_text(get_target_canister()).unwrap();
            let price = call_get_last_price(
                target_canister,
                ()
            ).await;
            if let Err(err) = price {
                ic_cdk::println!("error: {:?}", err);
                return;
            }
            let datum = price.unwrap();

            // temp: set gas_price, nonce
            Oracle::new(
                Address::from_str(&get_target_addr()).unwrap(),
                &web3_ctx().unwrap()
            ).update_state(U256::from_str(&datum.value).unwrap()).await.unwrap();
            ic_cdk::println!("ts={}, value={:?}", datum.timestamp, datum.value);
        }

        did_export!("relayer");
    }
}

pub fn generate_codes(manifest: &RelayerComponentManifest) -> anyhow::Result<TokenStream> {
    ensure!(manifest.type_ == ComponentType::Relayer, "type is not Relayer");

    let common_code_token = common_codes();
    let custom_code_token = custom_codes(manifest);

    let code = quote! {
        #common_code_token
        #custom_code_token
    };

    Ok(code)
}
