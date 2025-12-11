use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateExperimentRequest {
    pub name: String,
    pub toolchains: Vec<String>,
    pub mode: String,
    pub crate_select: String,
    pub priority: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Experiment {
    pub name: String,
    pub toolchains: Vec<String>,
    pub mode: String,
    pub crate_select: String,
    pub priority: i32,
    pub status: ExperimentStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ExperimentStatus {
    Queued,
    Running,
    Completed,
    Failed,
    Aborted,
}

impl std::fmt::Display for ExperimentStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExperimentStatus::Queued => write!(f, "排队中"),
            ExperimentStatus::Running => write!(f, "运行中"),
            ExperimentStatus::Completed => write!(f, "已完成"),
            ExperimentStatus::Failed => write!(f, "失败"),
            ExperimentStatus::Aborted => write!(f, "已中止"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookCallback {
    pub experiment: String,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperimentList {
    pub experiments: Vec<Experiment>,
}
