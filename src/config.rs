use std::{env, fs, path::Path};

use serde::Deserialize;

use crate::error::AppError;

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub llm: LlmConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    #[serde(default = "default_host")]
    pub host: String,
    #[serde(default = "default_port")]
    pub port: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LlmConfig {
    #[serde(default = "default_base_url")]
    pub base_url: String,
    pub model: String,
    #[serde(default = "default_system_prompt")]
    pub system_prompt: String,
    #[serde(default = "default_temperature")]
    pub temperature: f32,
    #[serde(default = "default_timeout_secs")]
    pub timeout_secs: u64,
    #[serde(default = "default_token_env")]
    pub token_env: String,
}

impl AppConfig {
    pub fn load_from_file(path: impl AsRef<Path>) -> Result<Self, AppError> {
        let raw = fs::read_to_string(path.as_ref()).map_err(|source| AppError::ConfigRead {
            path: path.as_ref().display().to_string(),
            source,
        })?;
        toml::from_str(&raw).map_err(AppError::ConfigParse)
    }
}

impl LlmConfig {
    pub fn token(&self) -> Result<String, AppError> {
        env::var(&self.token_env).map_err(|_| AppError::MissingTokenEnv {
            env_name: self.token_env.clone(),
        })
    }
}

fn default_host() -> String {
    "127.0.0.1".to_owned()
}

fn default_port() -> u16 {
    5800
}

fn default_base_url() -> String {
    "https://api.openai.com/v1".to_owned()
}

fn default_system_prompt() -> String {
    "You are a translation engine. Translate the user's text faithfully. Return only the translated text without commentary.".to_owned()
}

fn default_temperature() -> f32 {
    0.0
}

fn default_timeout_secs() -> u64 {
    60
}

fn default_token_env() -> String {
    "TOKEN".to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_toml_config() {
        let config = toml::from_str::<AppConfig>(
            r#"
[server]
host = "0.0.0.0"
port = 8080

[llm]
base_url = "https://example.com/v1"
model = "gpt-test"
system_prompt = "translate"
temperature = 0.1
timeout_secs = 30
token_env = "TOKEN"
"#,
        )
        .expect("config should parse");

        assert_eq!(config.server.host, "0.0.0.0");
        assert_eq!(config.server.port, 8080);
        assert_eq!(config.llm.model, "gpt-test");
        assert_eq!(config.llm.token_env, "TOKEN");
    }
}
