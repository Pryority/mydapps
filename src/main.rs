slint::include_modules!();
use dotenv::dotenv;
use eyre::Report;
use helios::{
    client::{Client, ClientBuilder, FileDB},
    config::{checkpoints, networks, networks::Network},
};
use std::path::PathBuf;
use tracing::info;

mod ui_callbacks;

struct AppState {
    ui: AppWindow,
    client: Client<FileDB>,
}

impl AppState {
    async fn new() -> Result<Self, Report> {
        dotenv().ok();

        // Initialize the Helios light client
        let client = init_client()?;
        println!(
            "\t🏗️  Client built on \"{}\" with external checkpoint fallbacks",
            Network::MAINNET
        );

        // Create an instance of the generated component
        let ui = AppWindow::new()?;
        let cf = checkpoints::CheckpointFallback::new()
            .build()
            .await
            .unwrap();
        let cf_clone = cf.clone();

        let mainnet_checkpoint = cf_clone
            .fetch_latest_checkpoint(&networks::Network::MAINNET)
            .await
            .unwrap();
        ui.set_latest_checkpoint(mainnet_checkpoint.to_string().into());

        Ok(AppState { ui, client })
    }

    async fn start(&mut self) -> Result<(), Report> {
        let start_time = std::time::Instant::now();
        start_client(&mut self.client).await?;
        let duration = format!("{:?}", std::time::Instant::now() - start_time);
        println!("\t🟢 Client started in {}", duration);
        Ok(())
    }

    async fn sync(&mut self) -> Result<(), Report> {
        let sync_start_time = std::time::Instant::now();
        sync_client(&mut self.client).await?;
        let duration = format!("{:?}", std::time::Instant::now() - sync_start_time);
        println!("\t🧬 Client synced in {:.5} seconds", duration);

        // Get the block number and update the UI
        let head_block_num = self.client.get_block_number().await.unwrap();
        self.ui.set_block_number(head_block_num.to_string().into());

        Ok(())
    }

    fn setup_ui_callbacks(&self) {
        // let client_handle = self.ui.as_weak();
        let active_dapp_handle = self.ui.as_weak();
        let active_chain_handle = self.ui.as_weak();
        // let checkpoint_handle = self.ui.as_weak();

        ui_callbacks::setup(
            &self.ui,
            // &mainnet_checkpoint,
            // checkpoint_handle.clone(),
            active_dapp_handle.clone(),
            active_chain_handle.clone(),
        );
    }

    fn run_ui(&self) {
        self.ui.run().unwrap();
    }
}

#[tokio::main]
async fn main() -> Result<(), Report> {
    let mut app_state = AppState::new().await?;
    app_state.setup_ui_callbacks();
    app_state.start().await?;
    app_state.sync().await?;
    app_state.run_ui();

    Ok(())
}

// SYNCHRONOUS FUNCTIONS ----------------------------------

fn init_client() -> Result<Client<FileDB>, Report> {
    let untrusted_rpc_url = std::env::var("EXECUTION")
        .expect("Please set the 'EXECUTION' environment variable with the RPC URL");
    info!("Using untrusted RPC URL [REDACTED]");

    let consensus_rpc = "https://www.lightclientdata.org";
    info!("Using consensus RPC URL: {}", consensus_rpc);

    let client: Client<FileDB> = match ClientBuilder::new()
        .network(Network::MAINNET)
        .consensus_rpc(consensus_rpc)
        .execution_rpc(&untrusted_rpc_url)
        .load_external_fallback()
        .data_dir(PathBuf::from("/tmp/helios"))
        .build()
    {
        Ok(client) => client,
        Err(err) => {
            // Handle the error here, log it, print a message, etc.
            // For now, let's print the error to the console.
            eprintln!("Error building Helios client: {}", err);
            return Err(err);
        }
    };

    Ok(client)
}

// ASYNCHRONOUS FUNCTIONS ---------------------------------

// Function to start the client
async fn start_client(client: &mut Client<FileDB>) -> Result<bool, Report> {
    println!("\t🚦 Client is starting...");
    client.start().await?;
    Ok(true)
}

// Function to wait until the client is synced
async fn sync_client(client: &mut Client<FileDB>) -> Result<bool, Report> {
    // Clone strong handles for properties
    println!("\t⏳ Client is awaiting synchronization...");
    client.wait_synced().await;
    Ok(true)
}
