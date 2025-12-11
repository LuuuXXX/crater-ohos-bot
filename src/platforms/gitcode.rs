use crate::config::PlatformConfig;
use crate::error::{BotError, Result};
use crate::platforms::PlatformAdapter;
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CreateCommentRequest {
    body: String,
}

pub struct GitCodeAdapter {
    client: Client,
    config: PlatformConfig,
    // Simple in-memory storage for experiment mappings
    // WARNING: This will lose all mappings on restart!
    // TODO: Replace with persistent storage (database or file-based) for production use
    experiment_mappings: Arc<RwLock<HashMap<String, String>>>,
}

impl GitCodeAdapter {
    pub fn new(config: PlatformConfig) -> Result<Self> {
        let client = Client::builder()
            .build()
            .map_err(|e| BotError::Internal(format!("Failed to create HTTP client: {}", e)))?;

        Ok(Self {
            client,
            config,
            experiment_mappings: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    fn make_mapping_key(&self, project: &str, issue_id: u64) -> String {
        format!("{}#{}", project, issue_id)
    }
}

#[async_trait]
impl PlatformAdapter for GitCodeAdapter {
    async fn post_comment(&self, project: &str, issue_id: u64, content: &str) -> Result<()> {
        // GitCode API: POST /api/v5/repos/{owner}/{repo}/issues/{number}/comments
        let url = format!(
            "{}/repos/{}/issues/{}/comments",
            self.config.api_url, project, issue_id
        );

        info!("Posting comment to GitCode issue: {}/{}", project, issue_id);
        debug!("Comment content: {}", content);

        let request = CreateCommentRequest {
            body: content.to_string(),
        };

        let response = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("token {}", &self.config.access_token))
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(BotError::Platform(format!(
                "Failed to post comment to GitCode: {} - {}",
                status, body
            )));
        }

        info!("Comment posted successfully");
        Ok(())
    }

    fn verify_webhook(&self, _payload: &[u8], signature: &str) -> Result<bool> {
        // GitCode uses X-GitCode-Token header for webhook verification
        // Use constant-time comparison to prevent timing attacks
        use subtle::ConstantTimeEq;
        
        let expected = self.config.webhook_secret.as_bytes();
        let provided = signature.as_bytes();
        
        if expected.len() != provided.len() {
            warn!("Webhook signature length mismatch");
            return Ok(false);
        }
        
        Ok(expected.ct_eq(provided).into())
    }

    async fn store_experiment_mapping(
        &self,
        project: &str,
        issue_id: u64,
        experiment_name: &str,
    ) -> Result<()> {
        let key = self.make_mapping_key(project, issue_id);
        let mut mappings = self.experiment_mappings.write().await;
        mappings.insert(key, experiment_name.to_string());
        Ok(())
    }

    async fn get_experiment_mapping(&self, project: &str, issue_id: u64) -> Result<Option<String>> {
        let key = self.make_mapping_key(project, issue_id);
        let mappings = self.experiment_mappings.read().await;
        Ok(mappings.get(&key).cloned())
    }
}

