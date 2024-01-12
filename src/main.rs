slint::include_modules!();
use dotenv::dotenv;
use std::{path::PathBuf};
use helios::{
    client::{ClientBuilder, Client, FileDB},
    config::{checkpoints, networks, networks::Network},
};
use tracing::info;
use eyre::Report;

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
    .build() {
        Ok(client) => client,
        Err(err) => {
            // Handle the error here, log it, print a message, etc.
            // For now, let's print the error to the console.
            eprintln!("Error building Helios client: {}", err);
            return Err(err);
        }
    };

    println!(
        "Built client on network \"{}\" with external checkpoint fallbacks",
        Network::MAINNET
    );

    client.start().await.unwrap();
    client.wait_synced().await;
    
    let head_block_num = client.get_block_number().await.unwrap();
    println!("synced up to block: {}", head_block_num);

    // Create an instance of the generated component
    let ui = AppWindow::new()?;

    // Clone strong handles for properties
    let client_handle = ui.as_weak();
    let active_dapp_handle = ui.as_weak();
    let active_chain_handle = ui.as_weak();
    let checkpoint_handle = ui.as_weak();

    // HELIOS ---------------------------------
    let cf = checkpoints::CheckpointFallback::new()
        .build()
        .await
        .unwrap();
    let cf_clone = cf.clone();

    let mainnet_checkpoint = cf_clone
        .fetch_latest_checkpoint(&networks::Network::MAINNET)
        .await
        .unwrap();

    // ui.on_sync(move || {
    //     let ui = client_handle.unwrap();
    //     ui.set_sync_status(synced());
    // });

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
        println!("Active Chain: {:?}", chain.name);
        ui.set_active_chain(c.clone());
        println!("New Active Chain: {:?}", chain.name);
    });

    // PERSONAL -------------------------------
    ui.on_request_send_tokens(move || {
        println!("Initiating SEND sequence.");
    });

    ui.on_request_all_balances(move || {
        println!("100 ETH, 30,000 OP, 4.2 BTC");
    });

    Ok(ui.run()?)
}
