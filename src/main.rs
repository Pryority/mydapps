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

    app.ui.run()?;
    Ok(())
}
