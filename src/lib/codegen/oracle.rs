/// Get Oracle Address
pub fn get_oracle_address(network_id: u32) -> String {
    let address = match network_id {
        80001 => "0539a0EF8e5E60891fFf0958A059E049e43020d9",
        _ => "",
    };
    address.to_string()
}
