use candid :: { Decode , Encode } ; use chainsight_cdk_macros :: { init_in , manage_single_state , setup_func , prepare_stable_structure , stable_memory_for_vec , StableMemoryStorable , timer_task_func , chainsight_common , did_export , snapshot_icp_source } ; use chainsight_cdk :: rpc :: { CallProvider , Caller , Message } ; init_in ! () ; chainsight_common ! (3600) ; manage_single_state ! ("target_canister" , String , false) ; setup_func ! ({ target_canister : String }) ; prepare_stable_structure ! () ; stable_memory_for_vec ! ("snapshot" , Snapshot , 0 , true) ; timer_task_func ! ("set_task" , "execute_task" , true) ; # [derive (Clone , Debug , candid :: CandidType , candid :: Deserialize , serde :: Serialize , StableMemoryStorable)] # [stable_mem_storable_opts (max_size = 10000 , is_fixed_size = false)] pub struct Snapshot { pub value : SnapshotValue , pub timestamp : u64 , } type SnapshotValue = CustomResponseStruct ; fn _get_last_snapshot_value () -> SnapshotValue { get_last_snapshot () . value } fn _get_top_snapshot_values (n : u64) -> Vec < SnapshotValue > { get_top_snapshots (n) . iter () . map (| s | s . value . clone ()) . collect () } fn _get_snapshot_value (idx : u64) -> SnapshotValue { get_snapshot (idx) . value } # [ic_cdk :: query] # [candid :: candid_method (query)] pub fn get_last_snapshot_value () -> SnapshotValue { _get_last_snapshot_value () } # [ic_cdk :: query] # [candid :: candid_method (query)] pub fn get_top_snapshot_values (n : u64) -> Vec < SnapshotValue > { _get_top_snapshot_values (n) } # [ic_cdk :: query] # [candid :: candid_method (query)] pub fn get_snapshot_value (idx : u64) -> SnapshotValue { _get_snapshot_value (idx) } # [ic_cdk :: update] # [candid :: candid_method (update)] pub async fn proxy_get_last_snapshot_value (input : std :: vec :: Vec < u8 >) -> std :: vec :: Vec < u8 > { use chainsight_cdk :: rpc :: Receiver ; chainsight_cdk :: rpc :: ReceiverProviderWithoutArgs :: < SnapshotValue > :: new (proxy () , _get_last_snapshot_value ,) . reply (input) . await } # [ic_cdk :: update] # [candid :: candid_method (update)] pub async fn proxy_get_top_snapshot_values (input : std :: vec :: Vec < u8 >) -> std :: vec :: Vec < u8 > { use chainsight_cdk :: rpc :: Receiver ; chainsight_cdk :: rpc :: ReceiverProvider :: < u64 , Vec < SnapshotValue >> :: new (proxy () , _get_top_snapshot_values ,) . reply (input) . await } # [ic_cdk :: update] # [candid :: candid_method (update)] pub async fn proxy_get_snapshot_value (input : std :: vec :: Vec < u8 >) -> std :: vec :: Vec < u8 > { use chainsight_cdk :: rpc :: Receiver ; chainsight_cdk :: rpc :: ReceiverProvider :: < u64 , SnapshotValue > :: new (proxy () , _get_snapshot_value ,) . reply (input) . await } # [derive (Clone , Debug , candid :: CandidType , serde :: Serialize , candid :: Deserialize)] pub struct CustomResponseStruct { pub value : String , pub timestamp : u64 } type CallCanisterArgs = sample_snapshot_indexer_icp :: CallCanisterArgs ; pub fn call_args () -> CallCanisterArgs { sample_snapshot_indexer_icp :: call_args () } snapshot_icp_source ! ("proxy_get_last_snapshot") ; type CallCanisterResponse = SnapshotValue ; async fn execute_task () { let current_ts_sec = ic_cdk :: api :: time () / 1000000 ; let target_canister = candid :: Principal :: from_text (get_target_canister ()) . unwrap () ; let px = _get_target_proxy (target_canister) . await ; let call_result = CallProvider :: new () . call (Message :: new :: < CallCanisterArgs > (call_args () , px . clone () , "proxy_get_last_snapshot") . unwrap ()) . await ; if let Err (err) = call_result { ic_cdk :: println ! ("error: {:?}" , err) ; return ; } let res = call_result . unwrap () . reply :: < CallCanisterResponse > () ; if let Err (err) = res { ic_cdk :: println ! ("error: {:?}" , err) ; return ; } let datum = Snapshot { value : res . unwrap () . clone () , timestamp : current_ts_sec , } ; add_snapshot (datum . clone ()) ; ic_cdk :: println ! ("ts={}, value={:?}" , datum . timestamp , datum . value) ; } did_export ! ("sample_snapshot_indexer_icp") ;