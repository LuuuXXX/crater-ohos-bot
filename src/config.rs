use crate::error::{BotError, Result};
use config::{Config as ConfigLoader, File};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub server: ServerConfig,
    pub crater: CraterConfig,
    pub platforms: PlatformsConfig,
    pub bot: BotConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CraterConfig {
    pub api_url: String,
    pub api_token: String,
    pub callback_base_url: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PlatformsConfig {
    pub gitcode: Option<PlatformConfig>,
    pub github: Option<PlatformConfig>,
    pub gitee: Option<PlatformConfig>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PlatformConfig {
    pub enabled: bool,
    pub api_url: String,
    pub access_token: String,
    pub webhook_secret: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BotConfig {
    pub name: String,
    pub trigger_prefix: String,
    pub default_mode: String,
    pub default_crate_select: String,
}

impl Config {
    pub fn from_file(path: &str) -> Result<Self> {
        let config = ConfigLoader::builder()
            .add_source(File::with_name(path))
            .build()
            .map_err(|e| BotError::Config(e.to_string()))?;

        config
            .try_deserialize()
            .map_err(|e| BotError::Config(e.to_string()))
    }

    pub fn from_env() -> Result<Self> {
        Self::from_file("config")
    }
}
