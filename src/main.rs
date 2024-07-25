use app_state::AppState;
pub use error::{AppError, Result};
use ethers::types::U256;
use slint::ComponentHandle;
use tokio::sync::mpsc;

mod app_state;
mod client;
mod error;
mod ui;

#[tokio::main]
async fn main() -> Result<()> {
    let mut app = AppState::new().await?;
    app.ui.setup_ui_callbacks();

    // Create a channel for communication
    let (tx, rx) = mpsc::channel(100);

    // Clone tx for use in the background task
    let tx_clone = tx.clone();

    // Start the client in the background
    tokio::spawn(async move {
        if let Err(e) = app.client.start().await {
            eprintln!("Error starting client: {:?}", e);
            let _ = tx_clone.send(ClientMessage::Error(e.to_string())).await;
            return;
        }
        let _ = tx_clone.send(ClientMessage::Started).await;

        if let Err(e) = app.client.sync().await {
            eprintln!("Error syncing client: {:?}", e);
            let _ = tx_clone.send(ClientMessage::Error(e.to_string())).await;
            return;
        }
        let _ = tx_clone.send(ClientMessage::Synced).await;

        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
            match app.client.get_block_number().await {
                Ok(block_number) => {
                    let _ = tx_clone
                        .send(ClientMessage::BlockNumber(block_number))
                        .await;
                }
                Err(e) => {
                    eprintln!("Error getting block number: {:?}", e);
                }
            }
        }
    });

    // Run the UI on the main thread
    let window = app.ui.window.as_weak();
    slint::spawn_local(async move {
        let mut rx = rx;
        while let Some(msg) = rx.recv().await {
            if let Some(window) = window.upgrade() {
                match msg {
                    ClientMessage::Started => {
                        window.set_sync_status(false);
                    }
                    ClientMessage::Synced => {
                        window.set_sync_status(true);
                    }
                    ClientMessage::BlockNumber(block_number) => {
                        if let Ok(block_number) = block_number.as_u64().try_into() {
                            window.set_block_number(block_number);
                        }
                    }
                    // ClientMessage::UpdateCheckpoint(checkpoint) => {
                    //     let mut app = app_ui.lock().await;
                    // }
                    ClientMessage::Error(err) => {
                        eprintln!("Client error: {}", err);
                        // You might want to update the UI to show this error
                    }
                }
            }
        }
    })
    .map_err(|e| AppError::Unknown(e.to_string()))?;

    // Run the UI
    app.ui.run()?;

    // If we get here, the UI has closed, so we can exit
    Ok(())
}

enum ClientMessage {
    Started,
    Synced,
    BlockNumber(U256),
    Error(String),
    // UpdateCheckpoint(String),
}
