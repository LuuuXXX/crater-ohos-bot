use crate::crater::WebhookCallback;
use crate::webhook::{CallbackHandler, WebhookReceiver};
use axum::{
    body::Bytes,
    extract::State,
    http::{HeaderMap, StatusCode},
    routing::{get, post},
    Json, Router,
};
use std::sync::Arc;
use tracing::{error, warn};

#[derive(Clone)]
pub struct AppState {
    pub webhook_receiver: Arc<WebhookReceiver>,
    pub callback_handler: Arc<CallbackHandler>,
    pub callback_secret: String,
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

    // Handle the webhook with raw payload for signature verification
    state
        .webhook_receiver
        .handle_gitcode_webhook(&payload, signature)
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
    headers: HeaderMap,
    Json(callback): Json<WebhookCallback>,
) -> Result<StatusCode, (StatusCode, String)> {
    // Verify callback authentication using a shared secret
    let auth_header = headers
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("");

    let expected_auth = format!("Bearer {}", state.callback_secret);
    
    // Use constant-time comparison to prevent timing attacks
    use subtle::ConstantTimeEq;
    if auth_header.as_bytes().ct_eq(expected_auth.as_bytes()).into() {
        // Authenticated, process the callback
        state
            .callback_handler
            .handle_crater_callback(callback)
            .await
            .map_err(|e| {
                error!("Failed to handle crater callback: {}", e);
                // Provide more specific error codes based on error type
                let status_code = if e.to_string().contains("parse") || e.to_string().contains("Invalid") {
                    StatusCode::BAD_REQUEST
                } else {
                    StatusCode::INTERNAL_SERVER_ERROR
                };
                (status_code, format!("Failed to handle crater callback: {}", e))
            })?;

        Ok(StatusCode::OK)
    } else {
        warn!("Crater callback authentication failed - unauthorized request");
        Err((
            StatusCode::UNAUTHORIZED,
            "Unauthorized: Invalid or missing authentication".to_string(),
        ))
    }
}

