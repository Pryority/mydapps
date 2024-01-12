slint::include_modules!();
use helios::{
    // checkpoints::CheckpointFallback,
    config::{checkpoints, networks},
};

#[tokio::main]
async fn main() -> Result<(), slint::PlatformError> {
    // Create an instance of the generated component
    let ui = AppWindow::new()?;

    // Clone strong handles for properties
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

    ui.run()
}
