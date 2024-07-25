use app_state::AppState;
pub use error::{AppError, Result};

mod app_state;
mod client;
mod error;
mod ui;

#[tokio::main]
async fn main() -> Result<()> {
    let mut app = AppState::new().await?;
    app.ui.setup_ui_callbacks();

    app.client.sync().await?;
    app.client.start().await?;
    app.ui.set_sync_status(true);
    let block_number = app.client.get_block_number().await?;
    app.ui.set_latest_block(block_number);

    app.ui.run()?;
    Ok(())
}
