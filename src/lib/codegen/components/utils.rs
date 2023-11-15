use std::path::Path;

use chainsight_cdk::{
    config::components::LENS_FUNCTION_ARGS_TYPE,
    convert::candid::{read_did_to_string_without_service, CanisterMethodIdentifier},
};

use crate::lib::utils::paths::canister_did_path_str;

// Generate types.rs code using the type information in bindings
pub fn generate_types_from_bindings(id: &str, identifier: &str) -> anyhow::Result<String> {
    let identifier = CanisterMethodIdentifier::new(identifier)?;
    let (args_ty, _) = identifier.get_types();

    let mut codes = format!(
        r#"use {}_bindings as bindings;
pub type {} = bindings::{};
"#,
        id,
        CanisterMethodIdentifier::RESPONSE_TYPE_NAME,
        CanisterMethodIdentifier::RESPONSE_TYPE_NAME
    );
    if args_ty.is_some() {
        codes += &format!(
            r#"pub type {} = bindings::{};"#,
            CanisterMethodIdentifier::REQUEST_ARGS_TYPE_NAME,
            CanisterMethodIdentifier::REQUEST_ARGS_TYPE_NAME
        );
    }

    Ok(codes)
}

// Get .did path of target component in the same project.
pub fn get_did_by_component_id(component_id: &str) -> Option<String> {
    let component_did_path = canister_did_path_str("src", component_id);
    if Path::new(&component_did_path).is_file() {
        Some(component_did_path)
    } else {
        None
    }
}

pub fn generate_method_identifier(
    identifier: &str,
    interface: &Option<String>,
) -> anyhow::Result<CanisterMethodIdentifier> {
    if let Some(path) = interface {
        let did_str = read_did_to_string_without_service(path)?;
        CanisterMethodIdentifier::new_with_did(identifier, did_str)
    } else {
        CanisterMethodIdentifier::new(identifier)
    }
}

// determine if the caller is a lens with arguments by CanisterMethodIdentifier
// NOTE: only for snapshot_indexer_icp, relayer
pub fn is_lens_with_args(identifier: CanisterMethodIdentifier) -> bool {
    let (req_ty, _) = identifier.get_types();
    if let Some(req_ty) = req_ty {
        // NOTE: check this simply by upgraded candid
        if req_ty.to_string() == LENS_FUNCTION_ARGS_TYPE {
            let lens_args_ty = identifier
                .get_type(LENS_FUNCTION_ARGS_TYPE)
                .unwrap_or_else(|| panic!("not found {} from identifier", LENS_FUNCTION_ARGS_TYPE));
            lens_args_ty.to_string().starts_with("record")
        } else {
            false
        }
    } else {
        false
    }
}
