use crater_ohos_bot::{
    api::{create_router, AppState},
    bot::CommandProcessor,
    config::Config,
    crater::CraterClient,
    error::Result,
    platforms::gitcode::GitCodeAdapter,
    webhook::{CallbackHandler, WebhookReceiver},
};
use std::sync::Arc;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "crater_ohos_bot=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("Starting crater-ohos-bot");

    // Load configuration
    let config = Config::from_env()?;
    info!("Configuration loaded");

    // Initialize crater client
    let crater_client = Arc::new(CraterClient::new(config.crater.clone())?);
    info!("Crater client initialized");

    // Initialize command processor
    let command_processor = Arc::new(CommandProcessor::new(crater_client.clone(), &config));
    info!("Command processor initialized");

    // Initialize platform adapters
    let gitcode_adapter = if let Some(gitcode_config) = &config.platforms.gitcode {
        if gitcode_config.enabled {
            Some(Arc::new(GitCodeAdapter::new(gitcode_config.clone())?))
        } else {
            None
        }
    } else {
        None
    };

    let gitcode_adapter = gitcode_adapter
        .ok_or_else(|| crater_ohos_bot::BotError::Config("GitCode adapter not enabled".to_string()))?;

    info!("GitCode adapter initialized");

    // Initialize webhook receiver
    let webhook_receiver = Arc::new(WebhookReceiver::new(
        command_processor.clone(),
        gitcode_adapter.clone(),
        config.clone(),
    ));
    info!("Webhook receiver initialized");

    // Initialize callback handler
    let callback_handler = Arc::new(CallbackHandler::new(gitcode_adapter.clone()));
    info!("Callback handler initialized");

    // Create application state
    let app_state = AppState {
        webhook_receiver,
        callback_handler,
    };

    // Create router
    let app = create_router(app_state);

    // Start server
    let addr = format!("{}:{}", config.server.host, config.server.port);
    info!("Starting server on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
