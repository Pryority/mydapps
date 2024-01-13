slint::include_modules!();
use dotenv::dotenv;
use eyre::Report;
use helios::{
    client::{Client, ClientBuilder, FileDB},
    config::{checkpoints, networks, networks::Network},
};
use std::path::PathBuf;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Report> {
    dotenv().ok();
    let untrusted_rpc_url = std::env::var("EXECUTION")
        .expect("Please set the 'EXECUTION' environment variable with the RPC URL");
    info!("Using untrusted RPC URL [REDACTED]");

    let consensus_rpc = "https://www.lightclientdata.org";
    info!("Using consensus RPC URL: {}", consensus_rpc);

    let mut client: Client<FileDB> = match ClientBuilder::new()
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

    println!(
        "\t🏗️  Client built on \"{}\" with external checkpoint fallbacks",
        Network::MAINNET
    );

    // Create an instance of the generated component
    let ui = AppWindow::new()?;
    // Clone strong handles for properties
    let client_handle = ui.as_weak();
    let active_dapp_handle = ui.as_weak();
    let active_chain_handle = ui.as_weak();
    let checkpoint_handle = ui.as_weak();
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

    // Start the client
    let client_start = std::time::Instant::now();
    // let start_result = start_client(&mut client).await?;
    start_client(&mut client).await?;
    let client_end = std::time::Instant::now();
    let start_duration = format!("{:?}", client_end - client_start);
    println!("\t🟢 Client started: in {} seconds", start_duration);

    ui.on_fetch_latest_mainnet_checkpoint(move || {
        let ui = checkpoint_handle.unwrap();

        // Fetch the latest mainnet checkpoint
        println!("Fetched latest mainnet checkpoint: {mainnet_checkpoint}");
        ui.set_latest_checkpoint(mainnet_checkpoint.to_string().into());
    });

    // DAPPS ----------------------------------
    ui.on_select_dapp(move |d| {
        let ui = active_dapp_handle.unwrap();
        ui.set_active_dapp(d.clone());
        let dapp = ui.get_active_dapp();
        println!("Active Dapp: {:?}", dapp.name);
    });

    // CHAINS ---------------------------------
    ui.on_select_chain(move |c| {
        let ui = active_chain_handle.unwrap();
        let chain = ui.get_active_chain();
        ui.set_active_chain(c.clone());
        println!("Active Chain: {:?}", chain.name);
    });

    // PERSONAL -------------------------------
    ui.on_request_send_tokens(move || {
        println!("Initiating SEND sequence.");
    });

    ui.on_request_all_balances(move || {
        println!("100 ETH, 30,000 OP, 4.2 BTC");
    });

    // Spawn a task for synchronization
    let wait_synced_start = std::time::Instant::now();
    let _ = sync(&mut client).await;
    let wait_synced_end = std::time::Instant::now();
    let sync_duration = format!("{:?}", wait_synced_end - wait_synced_start);
    println!("\t🧬 Client synced in {} seconds", sync_duration);

    // Get the block number and update the UI
    let head_block_num = client.get_block_number().await.unwrap();
    ui.set_block_number(head_block_num.to_string().into());

    ui.on_sync(move || {
        let ui = client_handle.unwrap();
        let wait_synced_start = std::time::Instant::now();
        let _ = sync(&mut client);
        let wait_synced_end = std::time::Instant::now();
        let sync_duration = format!("{:?}", wait_synced_end - wait_synced_start);
        println!("Client synced -- Took {} seconds", sync_duration);
        ui.set_sync_status(true);
    });

    ui.run().unwrap();

    Ok(())
}

// Function to start the client
async fn start_client(client: &mut Client<FileDB>) -> Result<bool, Report> {
    println!("\t🚦 Client is starting...");
    client.start().await?;
    Ok(true)
}

// Function to wait until the client is synced
async fn sync(client: &mut Client<FileDB>) -> Result<bool, Report> {
    // Clone strong handles for properties
    println!("\t⏳ Client is awaiting synchronization...");
    client.wait_synced().await;
    Ok(true)
}
