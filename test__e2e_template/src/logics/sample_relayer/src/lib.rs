pub type CallCanisterResponse = String;
pub type CallCanisterArgs = ();
pub fn call_args() -> CallCanisterArgs {
    ()
}
pub fn filter(_: &CallCanisterResponse) -> bool {
    true
}
