use crate::error::{BotError, Result};
use config::{Config as ConfigLoader, File};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Deserialize, Serialize)]
pub struct Config {
    pub server: ServerConfig,
    pub crater: CraterConfig,
    pub platforms: PlatformsConfig,
    pub bot: BotConfig,
}

impl fmt::Debug for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Config")
            .field("server", &self.server)
            .field("crater", &self.crater)
            .field("platforms", &"[REDACTED]")
            .field("bot", &self.bot)
            .finish()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct CraterConfig {
    pub api_url: String,
    pub api_token: String,
    pub callback_base_url: String,
    #[serde(default)]
    pub callback_secret: String,
}

impl fmt::Debug for CraterConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CraterConfig")
            .field("api_url", &self.api_url)
            .field("api_token", &"[REDACTED]")
            .field("callback_base_url", &self.callback_base_url)
            .field("callback_secret", &"[REDACTED]")
            .finish()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PlatformsConfig {
    pub gitcode: Option<PlatformConfig>,
    pub github: Option<PlatformConfig>,
    pub gitee: Option<PlatformConfig>,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct PlatformConfig {
    pub enabled: bool,
    pub api_url: String,
    pub access_token: String,
    pub webhook_secret: String,
}

impl fmt::Debug for PlatformConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PlatformConfig")
            .field("enabled", &self.enabled)
            .field("api_url", &self.api_url)
            .field("access_token", &"[REDACTED]")
            .field("webhook_secret", &"[REDACTED]")
            .finish()
    }
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
