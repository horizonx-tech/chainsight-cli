use std::collections::HashMap;
#[derive(Clone, Debug, Default, candid :: CandidType, serde :: Serialize, serde :: Deserialize)]
pub struct Transfer {
    pub from: String,
    pub to: String,
    pub value: String,
}
pub fn persist(elem: HashMap<u64, Vec<Transfer>>) {
    todo!()
}
