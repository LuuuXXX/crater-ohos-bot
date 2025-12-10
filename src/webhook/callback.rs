use crate::crater::WebhookCallback;
use crate::error::Result;
use crate::platforms::{gitcode::GitCodeAdapter, PlatformAdapter};
use crate::utils::parse_experiment_name;
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
        let (project, issue_id) = match parse_experiment_name(&callback.experiment) {
            Ok((p, i)) => (p, i),
            Err(e) => {
                info!("Cannot parse experiment name: {}", e);
                return Ok(());
            }
        };

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
