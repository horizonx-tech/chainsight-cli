use anyhow::Ok;
use candid::{bindings::rust::Target, check_prog, types::Type, IDLProg, TypeEnv};

// NOTE/TODO: Duplicates struct in common.rs, Use this
pub struct CanisterMethodIdentifier {
    pub identifier: String,
    type_env: TypeEnv,
}
impl CanisterMethodIdentifier {
    pub const REQUEST_ARGS_TYPE_NAME: &'static str = "RequestArgsType";
    pub const RESPONSE_TYPE_NAME: &'static str = "ResponseType";

    pub fn new(s: &str) -> anyhow::Result<Self> {
        let (identifier, args_ty, response_ty) = extract_elements(s)?;
        let did: String = Self::generate_did(&args_ty, &response_ty);

        let ast: IDLProg = did.to_string().parse().unwrap();
        let mut type_env = TypeEnv::new();
        let _ = check_prog(&mut type_env, &ast);

        Ok(Self {
            identifier,
            type_env,
        })
    }

    pub fn compile(&self) -> String {
        let mut config = candid::bindings::rust::Config::new();
        // Update the structure derive to chainsight's own settings
        config.type_attributes = "#[derive(Clone, Debug, candid :: CandidType, candid :: Deserialize, serde :: Serialize, chainsight_cdk_macros :: StableMemoryStorable)]".to_string();
        config.target = Target::CanisterStub;
        let contents = candid::bindings::rust::compile(&config, &self.type_env, &None);

        let lines = contents
            .lines()
            // Delete comment lines and blank lines
            .filter(|line| !(line.starts_with("//") || line.is_empty()))
            .collect::<Vec<_>>();
        lines.join("\n")
    }

    pub fn get_types(&self) -> (Option<&Type>, Option<&Type>) {
        (
            self.find_type(Self::REQUEST_ARGS_TYPE_NAME),
            self.find_type(Self::RESPONSE_TYPE_NAME),
        )
    }

    fn generate_did(args_ty: &str, response_ty: &str) -> String {
        let args_ty_did = if args_ty.is_empty() {
            "".to_string()
        } else {
            generate_did_type(Self::REQUEST_ARGS_TYPE_NAME, args_ty)
        };
        let response_ty_did = if response_ty.is_empty() {
            "".to_string()
        } else {
            generate_did_type(Self::RESPONSE_TYPE_NAME, response_ty)
        };
        format!("{}\n{}", args_ty_did, response_ty_did)
    }

    fn find_type(&self, key: &str) -> Option<&Type> {
        let ty = self.type_env.find_type(key);
        ty.ok()
    }
}

fn generate_did_type(key: &str, value: &str) -> String {
    format!("type {} = {};", key, value)
}

fn extract_elements(s: &str) -> anyhow::Result<(String, String, String)> {
    let (identifier, remains) = s
        .split_once(':')
        .expect("Invalid canister method identifier");
    let (args_ty, response_ty) = remains
        .split_once("->")
        .expect("Invalid canister method identifier");

    let trim_type_str = |s: &str| {
        let trimed = s.trim();
        let removed_brackets = trimed.trim_matches(|c| c == '(' || c == ')');
        removed_brackets.trim().to_string()
    };

    Ok((
        identifier.trim().to_string(),
        trim_type_str(args_ty),
        trim_type_str(response_ty),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    use insta::assert_snapshot;

    const TEST_IDENTS: &'static [(&'static str, &'static str); 4] = &[
        (&"normal", &"update_value : (nat64) -> (text)"),
        (&"no_args", &"get_value : () -> (text)"),
        (
            &"record",
            &"get_snapshot : (nat64) -> (record { value : text; timestamp : nat64 })",
        ),
        (
            &"nested_record",
            &"get_last_snapshot : () -> (record { dai : record { usd : float64 } })",
        ),
    ];

    #[test]
    fn test_compile() {
        for (label, s) in TEST_IDENTS {
            let ident = CanisterMethodIdentifier::new(s).expect("Failed to parse");
            let compiled = ident.compile();
            assert_snapshot!(format!("compile__{}", label.to_string()), compiled);
        }
    }

    #[test]
    fn test_generate_did() {
        let key = "Account";
        let value = "record { owner : principal; subaccount : opt blob }";
        let did = generate_did_type(key, value);
        assert_snapshot!(did);
    }

    #[test]
    fn test_extract_elements() {
        for (label, s) in TEST_IDENTS {
            let result = extract_elements(s).expect("Failed to parse");
            assert_snapshot!(
                format!("extract_elements__{}", label.to_string()),
                format!("{:#?}", result)
            );
        }
    }

    #[test]
    fn test_extract_elements_multi_nested_record() {
        let s =
            "icrc2_allowance : (record { account : record { owner : principal; subaccount : opt blob }; spender : record { owner : principal; subaccount : opt blob } }) -> (record { allowance : nat; expires_at : opt nat64 })";
        let (identifier, args_ty, response_ty) = extract_elements(s).expect("Failed to parse");
        assert_eq!(identifier, "icrc2_allowance");
        assert_eq!(args_ty, "record { account : record { owner : principal; subaccount : opt blob }; spender : record { owner : principal; subaccount : opt blob } }");
        assert_eq!(
            response_ty,
            "record { allowance : nat; expires_at : opt nat64 }"
        );
    }
}
