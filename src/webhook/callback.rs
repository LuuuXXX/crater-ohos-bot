use crate::crater::WebhookCallback;
use crate::error::Result;
use crate::platforms::{gitcode::GitCodeAdapter, PlatformAdapter};
use std::sync::Arc;
use tracing::info;

pub struct CallbackHandler {
    gitcode_adapter: Arc<GitCodeAdapter>,
}

impl CallbackHandler {
    pub fn new(gitcode_adapter: Arc<GitCodeAdapter>) -> Self {
        Self { gitcode_adapter }
    }

    pub async fn handle_crater_callback(&self, callback: WebhookCallback) -> Result<()> {
        info!("Received crater callback: {:?}", callback);

        // Parse the experiment name to extract project and issue_id
        // Format: {project}-{issue_id}
        let parts: Vec<&str> = callback.experiment.rsplitn(2, '-').collect();
        if parts.len() != 2 {
            info!("Cannot parse experiment name to extract issue info");
            return Ok(());
        }

        let issue_id = parts[0].parse::<u64>().unwrap_or(0);
        let project = parts[1].replace('-', "/");

        if issue_id == 0 {
            info!("Invalid issue ID in experiment name");
            return Ok(());
        }

        let message = match callback.status.as_str() {
            "completed" => {
                if let Some(report_url) = callback.report_url {
                    format!(
                        "🎉 实验 `{}` 已完成！\n\n📊 查看完整报告：[点击查看]({})",
                        callback.experiment, report_url
                    )
                } else {
                    format!("🎉 实验 `{}` 已完成！", callback.experiment)
                }
            }
            "failed" => {
                format!("❌ 实验 `{}` 失败。", callback.experiment)
            }
            "aborted" => {
                format!("⏹️ 实验 `{}` 已中止。", callback.experiment)
            }
            status => {
                format!("📊 实验 `{}` 状态更新：{}", callback.experiment, status)
            }
        };

        PlatformAdapter::post_comment(
            self.gitcode_adapter.as_ref(),
            &project,
            issue_id,
            &message,
        )
        .await?;

        Ok(())
    }
}
