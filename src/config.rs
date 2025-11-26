use serde::Deserialize;
use std::fs;
use std::io;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub llm: LlmConfig,
    #[serde(default)]
    pub web: WebConfig,
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct WebConfig {
    #[serde(default = "default_provider")]
    pub provider: String,
    #[allow(dead_code)]
    pub api_key: Option<String>,
}

fn default_provider() -> String {
    "duckduckgo".to_string()
}

#[derive(Deserialize, Debug, Clone)]
pub struct LlmConfig {
    pub provider: Option<String>,
    pub api_key: String,
    pub api_base_url: String,
    pub model_name: String,
}

impl Config {
    pub fn from_file(path: &str) -> Result<Self, io::Error> {
        let contents = fs::read_to_string(path)?;
        let config: Config =
            toml::from_str(&contents).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        Ok(config)
    }
}
