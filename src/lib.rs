pub mod api;
pub mod bot;
pub mod config;
pub mod crater;
pub mod error;
pub mod platforms;
pub mod webhook;

pub use config::Config;
pub use error::{BotError, Result};
