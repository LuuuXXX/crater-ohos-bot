pub mod gitcode;
pub mod github;
pub mod gitee;

use crate::error::Result;
use async_trait::async_trait;

#[async_trait]
pub trait PlatformAdapter: Send + Sync {
    /// Post a comment to an issue
    async fn post_comment(&self, project: &str, issue_id: u64, content: &str) -> Result<()>;

    /// Verify webhook signature
    fn verify_webhook(&self, payload: &[u8], signature: &str) -> Result<bool>;

    /// Store experiment mapping for an issue
    async fn store_experiment_mapping(
        &self,
        project: &str,
        issue_id: u64,
        experiment_name: &str,
    ) -> Result<()>;

    /// Get experiment mapping for an issue
    async fn get_experiment_mapping(&self, project: &str, issue_id: u64) -> Result<Option<String>>;
}
