pub fn src_path_str(root: &str) -> String {
    format!("{}/src", root)
}

pub fn logics_path_str(src: &str, component: &str) -> String {
    format!("{}/logics/{}", src, component)
}

pub fn canisters_path_str(src: &str, component: &str) -> String {
    format!("{}/canisters/{}", src, component)
}

pub fn bindings_path_str(src: &str, component: &str) -> String {
    format!("{}/bindings/{}", src, bindings_name(component))
}

pub fn canister_name(component: &str) -> String {
    format!("{}_canister", component)
}

pub fn bindings_name(component: &str) -> String {
    format!("{}_bindings", component)
}

pub fn logic_dependency(component: &str) -> String {
    format!(
        r#"{} = {{ path = "../../logics/{}" }}"#,
        component, component
    )
}

pub fn bindings_dependency(component: &str) -> String {
    format!(
        r#"{} = {{ path = "../../bindings/{}" }}"#,
        bindings_name(component),
        bindings_name(component)
    )
}
