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
    let mut contents = contents.to_string();
    for (k, v) in envs {
        contents = contents.replace(&format!("${{{}}}", k), &v);
    }
    Ok(contents)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_load_env() {
        let dotenv_file = r#"
        TEST_ENV=TEST
        TEST_ENV2=TEST2
        "#;
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
        let test_dotenv = "test_dotenv";
        std::fs::write(test_dotenv, dotenv_file).unwrap();
        dotenvy::from_filename(test_dotenv).ok();
        // test
        let actual = load_env(contents).unwrap();
        assert_eq!(actual, expected);
        // teardown
        std::fs::remove_file(test_dotenv).unwrap();
        dotenvy::dotenv().ok();
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
        let actual = load_env(contents).unwrap();
        assert_eq!(actual, expected);
        dotenvy::dotenv().ok();
    }
}
