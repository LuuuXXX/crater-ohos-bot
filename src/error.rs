use thiserror::Error;

#[derive(Error, Debug)]
pub enum BotError {
    #[error("Configuration error: {0}")]
    Config(String),

    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    #[error("JSON parsing error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Invalid command: {0}")]
    InvalidCommand(String),

    #[error("Platform error: {0}")]
    Platform(String),

    #[error("Crater API error: {0}")]
    CraterApi(String),

    #[error("Webhook verification failed: {0}")]
    WebhookVerification(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Internal error: {0}")]
    Internal(String),
}

pub type Result<T> = std::result::Result<T, BotError>;
