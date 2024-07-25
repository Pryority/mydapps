use helios::errors::{BlockNotFoundError, SlotNotFoundError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Block not found: {0}")]
    BlockNotFound(#[from] BlockNotFoundError),

    #[error("Slot not found: {0}")]
    SlotNotFound(#[from] SlotNotFoundError),

    #[error("RPC error: {0}")]
    RpcError(String),

    #[error("UI error: {0}")]
    UIError(String),

    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),

    #[error("Slint error: {0}")]
    SlintError(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

pub type Result<T> = std::result::Result<T, AppError>;
