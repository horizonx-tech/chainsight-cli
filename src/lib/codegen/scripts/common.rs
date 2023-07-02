pub fn generate_command_to_set_task(label: &str, interval: u32, delay: u32) -> String {
    format!(r#"dfx canister call {} set_task '({}, {})'"#, label, interval, delay)
}