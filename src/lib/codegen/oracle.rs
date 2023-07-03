use super::components::common::DestinactionType;

/// Get Oracle Attributes
pub fn get_oracle_attributes(type_: &DestinactionType) -> (String, String, &'static str) {
    match type_ {
        DestinactionType::Uint256Oracle => {
            let oracle_name = "Uint256Oracle";
            (
                oracle_name.to_string(),
                format!("{}.json", oracle_name),
                include_str!("../../../resources/Uint256Oracle.json"),
            )
        }
        DestinactionType::Uint128Oracle => {
            let oracle_name = "Uint128Oracle";
            (
                oracle_name.to_string(),
                format!("{}.json", oracle_name),
                include_str!("../../../resources/Uint128Oracle.json"),
            )
        }
        DestinactionType::Uint64Oracle => {
            let oracle_name = "Uint64Oracle";
            (
                oracle_name.to_string(),
                format!("{}.json", oracle_name),
                include_str!("../../../resources/Uint64Oracle.json"),
            )
        }
        DestinactionType::StringOracle => {
            let oracle_name = "StringOracle";
            (
                oracle_name.to_string(),
                format!("{}.json", oracle_name),
                include_str!("../../../resources/StringOracle.json"),
            )
        }
    }
}

/// Get Oracle Address
pub fn get_oracle_address(network_id: u32, type_: DestinactionType) -> String {
    let address = match network_id {
        80001 => match type_ {
            DestinactionType::Uint256Oracle => "0539a0EF8e5E60891fFf0958A059E049e43020d9",
            DestinactionType::Uint128Oracle => "7ecbe4fe2ea7631b948f95b76ecdaa70cf9782f4",
            DestinactionType::Uint64Oracle => "69a37Ba9b2DFbEA4bF658949c966f4EE324469d3",
            DestinactionType::StringOracle => "2b26d3a003a65Cd7cEb958cda68262Ba1D631C18",
        },
        _ => "",
    };
    address.to_string()
}
