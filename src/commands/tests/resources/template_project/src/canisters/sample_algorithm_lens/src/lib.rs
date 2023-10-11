use candid::CandidType;
use chainsight_cdk_macros::{chainsight_common, did_export, init_in, lens_method};
use ic_web3_rs::futures::{future::BoxFuture, FutureExt};
chainsight_common!(60);
init_in!();
use sample_algorithm_lens::*;
lens_method!(1usize);
did_export!("sample_algorithm_lens");
