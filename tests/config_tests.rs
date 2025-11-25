use rust_tui_coder::config::Config;
use std::fs;

#[test]
fn test_config_from_file() {
    let test_config_path = "tmp_rovodev_test_config.toml";
    let config_content = r#"
[llm]
provider = "openai"
api_key = "test_key"
api_base_url = "http://localhost:11434/v1"
model_name = "test-model"
"#;

    fs::write(test_config_path, config_content).unwrap();

    let result = Config::from_file(test_config_path);
    assert!(result.is_ok());

    let config = result.unwrap();
    assert_eq!(config.llm.provider, Some("openai".to_string()));
    assert_eq!(config.llm.api_key, "test_key");
    assert_eq!(config.llm.api_base_url, "http://localhost:11434/v1");
    assert_eq!(config.llm.model_name, "test-model");

    fs::remove_file(test_config_path).ok();
}

#[test]
fn test_config_from_file_missing() {
    let result = Config::from_file("tmp_rovodev_nonexistent_config.toml");
    assert!(result.is_err());
}

#[test]
fn test_config_from_file_invalid_toml() {
    let test_config_path = "tmp_rovodev_test_config_invalid.toml";
    let invalid_content = "this is not valid toml [[[";

    fs::write(test_config_path, invalid_content).unwrap();

    let result = Config::from_file(test_config_path);
    assert!(result.is_err());

    fs::remove_file(test_config_path).ok();
}

#[test]
fn test_config_clone() {
    let test_config_path = "tmp_rovodev_test_config_clone.toml";
    let config_content = r#"
[llm]
api_key = "test_key"
api_base_url = "http://localhost:11434/v1"
model_name = "test-model"
"#;

    fs::write(test_config_path, config_content).unwrap();

    let config = Config::from_file(test_config_path).unwrap();
    let cloned_config = config.clone();

    assert_eq!(config.llm.api_key, cloned_config.llm.api_key);
    assert_eq!(config.llm.api_base_url, cloned_config.llm.api_base_url);
    assert_eq!(config.llm.model_name, cloned_config.llm.model_name);

    fs::remove_file(test_config_path).ok();
}
