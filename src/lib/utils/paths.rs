pub fn src_path_str(root: &str) -> String {
    format!("{}/src", root)
}

pub fn logics_path_str(src: &str, component: &str) -> String {
    format!("{}/logics/{}", src, component)
}

pub fn canisters_path_str(src: &str, component: &str) -> String {
    format!("{}/canisters/{}", src, component)
}

pub fn canister_did_path_str(src: &str, component: &str) -> String {
    format!("{}/{}.did", canisters_path_str(src, component), component)
}

pub fn bindings_path_str(src: &str, component: &str) -> String {
    format!("{}/bindings/{}", src, bindings_name(component))
}

pub fn accessors_path_str(src: &str, component: &str) -> String {
    format!("{}/accessors/{}", src, accessors_name(component))
}

pub fn canister_name(component: &str) -> String {
    format!("{}_canister", component)
}

pub fn bindings_name(component: &str) -> String {
    format!("{}_bindings", component)
}
pub fn accessors_name(component: &str) -> String {
    format!("{}_accessors", component)
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

pub fn accessors_dependency(component: &str) -> String {
    format!(
        r#"{} = {{ path = "../../accessors/{}" }}"#,
        accessors_name(component),
        accessors_name(component)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_funcs_to_get_path_str() {
        assert_eq!(src_path_str("pj_root"), "pj_root/src");

        let cases: Vec<(fn(&str, &str) -> String, &str, &str)> = vec![
            (logics_path_str, "logics", ""),
            (canisters_path_str, "canisters", ""),
            (bindings_path_str, "bindings", "_bindings"),
            (accessors_path_str, "accessors", "_accessors"),
        ];
        for (func_to_test, dir, suffix) in cases {
            assert_eq!(
                func_to_test("src", "component"),
                format!("src/{}/component{}", dir, suffix)
            );
        }

        assert_eq!(
            canister_did_path_str("src", "component"),
            "src/canisters/component/component.did"
        );
    }

    #[test]
    fn test_funcs_for_name() {
        assert_eq!(canister_name("sample"), "sample_canister".to_owned());
        assert_eq!(bindings_name("sample"), "sample_bindings".to_owned());
        assert_eq!(accessors_name("sample"), "sample_accessors".to_owned());
    }

    #[test]
    fn test_funcs_for_dependency() {
        assert_eq!(
            logic_dependency("sample"),
            r#"sample = { path = "../../logics/sample" }"#.to_owned()
        );
        assert_eq!(
            bindings_dependency("sample"),
            r#"sample_bindings = { path = "../../bindings/sample_bindings" }"#.to_owned()
        );
        assert_eq!(
            accessors_dependency("sample"),
            r#"sample_accessors = { path = "../../accessors/sample_accessors" }"#.to_owned()
        );
    }
}
