use super::components::common::DestinactionType;

/// Get Oracle Attributes
pub fn get_oracle_attributes(type_: &DestinactionType) -> (String, String, &'static str) {
    match type_ {
        DestinactionType::Uint256Oracle => {
            let oracle_name = "Uint256Oracle";
            (
                oracle_name.to_string(),
                format!("{}.json", oracle_name),
                include_str!("../../../resources/Uint256Oracle.json")
            )
        },
        DestinactionType::StringOracle => {
            let oracle_name = "StringOracle";
            (
                oracle_name.to_string(),
                format!("{}.json", oracle_name),
                include_str!("../../../resources/StringOracle.json")
            )
        },
    }
}

/// Get Oracle Address
pub fn get_oracle_address(network_id: u32, type_: DestinactionType) -> String {
    let address = match network_id {
        80001 => {
            match type_ {
                DestinactionType::Uint256Oracle => "0539a0EF8e5E60891fFf0958A059E049e43020d9",
                DestinactionType::StringOracle => "2b26d3a003a65Cd7cEb958cda68262Ba1D631C18",
            }
        }
        _ => ""
    };
    address.to_string()
}