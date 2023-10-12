use sample_algorithm_lens_accessors::*;
#[derive(Clone, Debug, Default, candid :: CandidType, serde :: Deserialize, serde :: Serialize)]
pub struct LensValue {
    pub dummy: u64,
}
pub async fn calculate(targets: Vec<String>) -> LensValue {
    let _result = get_sample_snapshot_indexer_icp(targets.get(0usize).unwrap().clone()).await;
    todo!()
}
