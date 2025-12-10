use crate::crater::WebhookCallback;
use crate::webhook::{CallbackHandler, GitCodeWebhook, WebhookReceiver};
use axum::{
    body::Bytes,
    extract::State,
    http::{HeaderMap, StatusCode},
    routing::{get, post},
    Json, Router,
};
use std::sync::Arc;
use tracing::error;

#[derive(Clone)]
pub struct AppState {
    pub webhook_receiver: Arc<WebhookReceiver>,
    pub callback_handler: Arc<CallbackHandler>,
}

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/webhook/gitcode", post(gitcode_webhook_handler))
        .route("/callback/crater", post(crater_callback_handler))
        .with_state(state)
}

async fn health_check() -> &'static str {
    "OK"
}

async fn gitcode_webhook_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    payload: Bytes,
) -> Result<StatusCode, (StatusCode, String)> {
    // Extract the X-GitCode-Token header
    let signature = headers
        .get("X-GitCode-Token")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("");

    let webhook: GitCodeWebhook = serde_json::from_slice(&payload).map_err(|e| {
        error!("Failed to parse webhook payload: {}", e);
        (
            StatusCode::BAD_REQUEST,
            format!("Failed to parse webhook payload: {}", e),
        )
    })?;

    // Handle the webhook
    state
        .webhook_receiver
        .handle_gitcode_webhook(webhook, signature)
        .await
        .map_err(|e| {
            error!("Failed to handle webhook: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to handle webhook: {}", e),
            )
        })?;

    Ok(StatusCode::OK)
}

async fn crater_callback_handler(
    State(state): State<AppState>,
    Json(callback): Json<WebhookCallback>,
) -> Result<StatusCode, (StatusCode, String)> {
    state
        .callback_handler
        .handle_crater_callback(callback)
        .await
        .map_err(|e| {
            error!("Failed to handle crater callback: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to handle crater callback: {}", e),
            )
        })?;

    Ok(StatusCode::OK)
}
