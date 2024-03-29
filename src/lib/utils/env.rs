use std::collections::HashMap;

pub fn cache_envfile(env_file_path: Option<&str>) -> anyhow::Result<()> {
    if let Some(env_file_path) = env_file_path {
        dotenvy::from_filename(env_file_path)?;
    } else {
        dotenvy::dotenv()?;
    }
    Ok(())
}

/// replace ${ENV_VAR} with actual value
pub fn load_env(contents: &str) -> anyhow::Result<String> {
    let mut envs = HashMap::new();
    for (k, v) in dotenvy::vars() {
        envs.insert(k, v);
    }
    _load_env(contents, envs)
}

fn _load_env(contents: &str, env: HashMap<String, String>) -> anyhow::Result<String> {
    let mut contents = contents.to_string();
    for (k, v) in env {
        contents = contents.replace(&format!("${{{}}}", k), &v);
    }
    Ok(contents)
}

#[cfg(test)]
mod tests {
    use super::*;

    const DOTENV_FILE: &str = r#"
        TEST_ENV=TEST
        TEST_ENV2=TEST2
        "#;

    #[test]
    fn test_cache_envfile() {
        let test_dotenv = ".env__test_cache_envfile";
        std::fs::write(test_dotenv, DOTENV_FILE).unwrap();

        cache_envfile(Some(test_dotenv)).unwrap();
        assert_eq!(dotenvy::var("TEST_ENV").unwrap(), "TEST");
        assert_eq!(dotenvy::var("TEST_ENV2").unwrap(), "TEST2");
        assert!(dotenvy::var("TEST_ENV3").is_err());

        // teardown
        std::fs::remove_file(test_dotenv).unwrap();
        let _ = dotenvy::dotenv();
    }

    #[test]
    fn test_load_env() {
        let contents = r#"
        TEST_ENV: ${TEST_ENV}
        TEST_ENV2: ${TEST_ENV2}
        test3: ${TEST_ENV}
        raw: raw
        "#;
        let expected = r#"
        TEST_ENV: TEST
        TEST_ENV2: TEST2
        test3: TEST
        raw: raw
        "#;
        // setup
        let mut envs = HashMap::new();
        [("TEST_ENV", "TEST"), ("TEST_ENV2", "TEST2")]
            .iter()
            .for_each(|(k, v)| {
                envs.insert(k.to_string(), v.to_string());
            });
        // test
        let actual = _load_env(contents, envs).unwrap();
        assert_eq!(actual, expected);
    }
    #[test]
    fn test_load_env_without_env_file() {
        let contents = r#"
        TEST_ENV: ${TEST_ENV}
        TEST_ENV2: ${TEST_ENV2}
        test3: ${TEST_ENV}
        raw: raw
        "#;
        let expected = contents;
        let actual = _load_env(contents, HashMap::new()).unwrap();
        assert_eq!(actual, expected);
        dotenvy::dotenv().ok();
    }
}
