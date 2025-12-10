use crate::config::PlatformConfig;
use crate::error::{BotError, Result};
use crate::platforms::PlatformAdapter;
use async_trait::async_trait;

pub struct GiteeAdapter {
    _config: PlatformConfig,
}

impl GiteeAdapter {
    pub fn new(config: PlatformConfig) -> Result<Self> {
        Ok(Self { _config: config })
    }
}

#[async_trait]
impl PlatformAdapter for GiteeAdapter {
    async fn post_comment(&self, _project: &str, _issue_id: u64, _content: &str) -> Result<()> {
        Err(BotError::Platform(
            "Gitee adapter not implemented yet".to_string(),
        ))
    }

    fn verify_webhook(&self, _payload: &[u8], _signature: &str) -> Result<bool> {
        Err(BotError::Platform(
            "Gitee adapter not implemented yet".to_string(),
        ))
    }

    async fn store_experiment_mapping(
        &self,
        _project: &str,
        _issue_id: u64,
        _experiment_name: &str,
    ) -> Result<()> {
        Err(BotError::Platform(
            "Gitee adapter not implemented yet".to_string(),
        ))
    }

    async fn get_experiment_mapping(
        &self,
        _project: &str,
        _issue_id: u64,
    ) -> Result<Option<String>> {
        Err(BotError::Platform(
            "Gitee adapter not implemented yet".to_string(),
        ))
    }
}
