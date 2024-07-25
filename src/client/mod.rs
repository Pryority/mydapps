mod init;

use ethers::types::U256;
use indicatif::{ProgressBar, ProgressStyle};

use crate::{error::Result, AppError};
use helios::{
    client::{Client as HeliosClient, FileDB},
    errors::{BlockNotFoundError, RpcError, SlotNotFoundError},
};

pub struct Helios {
    inner: HeliosClient<FileDB>,
}

impl Helios {
    pub fn new() -> Result<Self> {
        let inner = init::init_client()?;
        Ok(Self { inner })
    }

    pub async fn start(&mut self) -> Result<()> {
        let start_time = std::time::Instant::now();
        println!("🚦 Client is starting...");

        self.inner.start().await.map_err(|err| {
            eprintln!("Error building Helios client: {}", err);
            if err.is::<BlockNotFoundError>() {
                AppError::BlockNotFound(err.downcast::<BlockNotFoundError>().unwrap())
            } else if err.is::<SlotNotFoundError>() {
                AppError::SlotNotFound(err.downcast::<SlotNotFoundError>().unwrap())
            } else if err.is::<RpcError<String>>() {
                AppError::RpcError(err.to_string())
            } else {
                AppError::Unknown(err.to_string())
            }
        })?;

        let duration = format!("{:?}", std::time::Instant::now() - start_time);
        println!("🟢 Client started in {}", duration);

        let block_number = self.inner.get_block_number().await;
        println!("Block {:?}", block_number);
        Ok(())
    }

    pub async fn sync(&mut self) -> Result<()> {
        let sync_start_time = std::time::Instant::now();

        // Create a progress bar
        let progress_bar = ProgressBar::new_spinner();
        progress_bar.set_style(
            ProgressStyle::default_spinner()
                .template("{prefix:.cyan} {spinner:.green} {msg}")
                .unwrap()
                .tick_strings(&["🌍", "🌍", "🌍", "🌎", "🌎", "🌎", "🌏", "🌏", "🌏"]),
        );
        progress_bar.set_prefix("Syncing. Please wait. This usually takes ~1 minute.");

        // Spawn a task to update the progress bar
        let progress_bar_clone = progress_bar.clone();
        let update_progress_task = tokio::spawn(async move {
            while !progress_bar_clone.is_finished() {
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                progress_bar_clone.tick();
            }
        });
        self.inner.wait_synced().await;

        // Finish the progress bar
        progress_bar.finish_with_message("✅ COMPLETE");

        // Wait for the progress update task to finish
        update_progress_task
            .await
            .map_err(|e| AppError::Unknown(format!("Task join error: {:?}", e)))?;

        let duration = format!("{:?}", std::time::Instant::now() - sync_start_time);
        println!("🧬 Client synced in {:.5} seconds", duration);

        Ok(())
    }
    pub async fn get_block_number(&self) -> Result<U256> {
        self.inner
            .get_block_number()
            .await
            .map_err(|e| AppError::BlockNotFound(e.downcast::<BlockNotFoundError>().unwrap()))
    }
}
