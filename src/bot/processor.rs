use crate::bot::commands::BotCommand;
use crate::config::{BotConfig, Config};
use crate::crater::{CraterClient, CreateExperimentRequest};
use crate::error::Result;
use crate::platforms::PlatformAdapter;
use std::sync::Arc;
use tracing::info;

pub struct CommandProcessor {
    crater_client: Arc<CraterClient>,
    config: BotConfig,
    callback_base_url: String,
}

impl CommandProcessor {
    pub fn new(crater_client: Arc<CraterClient>, config: &Config) -> Self {
        Self {
            crater_client,
            config: config.bot.clone(),
            callback_base_url: config.crater.callback_base_url.clone(),
        }
    }

    pub async fn process<P: PlatformAdapter>(
        &self,
        command: BotCommand,
        platform: &P,
        project: &str,
        issue_id: u64,
    ) -> Result<String> {
        match command {
            BotCommand::Run { toolchains } => {
                self.handle_run(platform, project, issue_id, toolchains)
                    .await
            }
            BotCommand::Status => self.handle_status(platform, project, issue_id).await,
            BotCommand::Abort => self.handle_abort(platform, project, issue_id).await,
            BotCommand::Help => Ok(self.handle_help()),
            BotCommand::List => self.handle_list().await,
        }
    }

    async fn handle_run<P: PlatformAdapter>(
        &self,
        platform: &P,
        project: &str,
        issue_id: u64,
        toolchains: Vec<String>,
    ) -> Result<String> {
        let experiment_name = format!("{}-{}", project.replace('/', "-"), issue_id);
        
        info!("Creating experiment: {}", experiment_name);

        let callback_url = format!("{}/callback/crater", self.callback_base_url);

        let request = CreateExperimentRequest {
            name: experiment_name.clone(),
            toolchains: toolchains.clone(),
            mode: self.config.default_mode.clone(),
            crate_select: self.config.default_crate_select.clone(),
            priority: 0,
            callback_url: Some(callback_url),
        };

        let experiment = self.crater_client.create_experiment(request).await?;
        self.crater_client.run_experiment(&experiment.name).await?;

        let message = format!(
            "✅ 实验 `{}` 已创建并开始执行。\n\n\
            工具链：{}\n\
            模式：{}\n\
            状态：{}\n\n\
            我会在实验完成后通知您。",
            experiment.name,
            toolchains.join(" vs "),
            experiment.mode,
            experiment.status
        );

        // Store the experiment mapping for this issue
        platform
            .store_experiment_mapping(project, issue_id, &experiment_name)
            .await?;

        Ok(message)
    }

    async fn handle_status<P: PlatformAdapter>(
        &self,
        platform: &P,
        project: &str,
        issue_id: u64,
    ) -> Result<String> {
        let experiment_name = platform
            .get_experiment_mapping(project, issue_id)
            .await?;

        if let Some(name) = experiment_name {
            let experiment = self.crater_client.get_experiment(&name).await?;
            let message = format!(
                "📊 实验状态\n\n\
                名称：`{}`\n\
                工具链：{}\n\
                状态：{}\n\
                模式：{}",
                experiment.name,
                experiment.toolchains.join(" vs "),
                experiment.status,
                experiment.mode
            );
            Ok(message)
        } else {
            Ok("当前没有与此 Issue 关联的实验。".to_string())
        }
    }

    async fn handle_abort<P: PlatformAdapter>(
        &self,
        platform: &P,
        project: &str,
        issue_id: u64,
    ) -> Result<String> {
        let experiment_name = platform
            .get_experiment_mapping(project, issue_id)
            .await?;

        if let Some(name) = experiment_name {
            self.crater_client.abort_experiment(&name).await?;
            Ok(format!("⏹️ 实验 `{}` 已中止。", name))
        } else {
            Ok("当前没有与此 Issue 关联的实验可以中止。".to_string())
        }
    }

    fn handle_help(&self) -> String {
        format!(
            "## {} 帮助\n\n\
            ### 可用命令\n\n\
            - `{} run <toolchain1> <toolchain2>` - 创建并运行实验\n\
            - `{} status` - 查看当前实验状态\n\
            - `{} abort` - 中止当前实验\n\
            - `{} list` - 列出所有实验\n\
            - `{} help` - 显示此帮助信息\n\n\
            ### 示例\n\n\
            ```\n\
            {} run stable beta\n\
            {} run nightly-2024-01-01 stable\n\
            ```",
            self.config.name,
            self.config.trigger_prefix,
            self.config.trigger_prefix,
            self.config.trigger_prefix,
            self.config.trigger_prefix,
            self.config.trigger_prefix,
            self.config.trigger_prefix,
            self.config.trigger_prefix
        )
    }

    async fn handle_list(&self) -> Result<String> {
        let experiments = self.crater_client.list_experiments().await?;
        
        if experiments.is_empty() {
            return Ok("当前没有实验。".to_string());
        }

        let mut message = "## 实验列表\n\n".to_string();
        for exp in experiments.iter().take(10) {
            message.push_str(&format!(
                "- `{}` - {} ({})\n",
                exp.name, exp.status, exp.toolchains.join(" vs ")
            ));
        }

        if experiments.len() > 10 {
            message.push_str(&format!("\n_...还有 {} 个实验_", experiments.len() - 10));
        }

        Ok(message)
    }
}
