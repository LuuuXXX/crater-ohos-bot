use crate::bot::{BotCommand, CommandProcessor};
use crate::config::Config;
use crate::error::{BotError, Result};
use crate::platforms::{gitcode::GitCodeAdapter, PlatformAdapter};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::{error, info};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitCodeWebhook {
    pub object_kind: String,
    pub project: Option<GitCodeProject>,
    pub issue: Option<GitCodeIssue>,
    pub object_attributes: Option<GitCodeNote>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitCodeProject {
    pub path_with_namespace: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitCodeIssue {
    pub iid: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitCodeNote {
    pub note: String,
}

pub struct WebhookReceiver {
    processor: Arc<CommandProcessor>,
    gitcode_adapter: Arc<GitCodeAdapter>,
    config: Config,
}

impl WebhookReceiver {
    pub fn new(
        processor: Arc<CommandProcessor>,
        gitcode_adapter: Arc<GitCodeAdapter>,
        config: Config,
    ) -> Self {
        Self {
            processor,
            gitcode_adapter,
            config,
        }
    }

    pub async fn handle_gitcode_webhook(
        &self,
        webhook: GitCodeWebhook,
        signature: &str,
    ) -> Result<()> {
        info!("Received GitCode webhook: {:?}", webhook.object_kind);

        // Verify webhook signature
        let payload = serde_json::to_vec(&webhook)?;
        if !PlatformAdapter::verify_webhook(self.gitcode_adapter.as_ref(), &payload, signature)? {
            info!("Webhook signature verification failed");
            return Err(BotError::WebhookVerification(
                "Invalid webhook signature".to_string(),
            ));
        }

        // Only process note (comment) events
        if webhook.object_kind != "note" {
            info!("Ignoring non-note webhook event");
            return Ok(());
        }

        let project = webhook
            .project
            .ok_or_else(|| BotError::Platform("Missing project information".to_string()))?;

        let issue = webhook
            .issue
            .ok_or_else(|| BotError::Platform("Missing issue information".to_string()))?;

        let note = webhook
            .object_attributes
            .ok_or_else(|| BotError::Platform("Missing note information".to_string()))?;

        info!(
            "Processing comment on {}/issue#{}",
            project.path_with_namespace, issue.iid
        );

        // Parse the command
        let command = match BotCommand::parse(&note.note, &self.config.bot.trigger_prefix)? {
            Some(cmd) => cmd,
            None => {
                info!("Comment does not contain a bot command");
                return Ok(());
            }
        };

        info!("Parsed command: {:?}", command);

        // Process the command
        let response = self
            .processor
            .process(
                command,
                self.gitcode_adapter.as_ref(),
                &project.path_with_namespace,
                issue.iid,
            )
            .await;

        match response {
            Ok(message) => {
                PlatformAdapter::post_comment(
                    self.gitcode_adapter.as_ref(),
                    &project.path_with_namespace,
                    issue.iid,
                    &message,
                )
                .await?;
                info!("Command processed successfully");
            }
            Err(e) => {
                error!("Error processing command: {}", e);
                let error_message = format!("❌ 错误: {}", e);
                if let Err(comment_err) = PlatformAdapter::post_comment(
                    self.gitcode_adapter.as_ref(),
                    &project.path_with_namespace,
                    issue.iid,
                    &error_message,
                )
                .await
                {
                    error!("Failed to post error comment: {}", comment_err);
                }
                return Err(e);
            }
        }

        Ok(())
    }
}
