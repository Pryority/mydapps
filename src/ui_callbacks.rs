// ui_callbacks.rs

use crate::AppWindow;
// use ethers::types::H256;
use slint::Weak;
use std::borrow::{Borrow, BorrowMut};

pub fn setup(
    ui: &AppWindow,
    // mainnet_checkpoint: H256,
    // checkpoint_handle: Weak<AppWindow>,
    active_dapp_handle: Weak<AppWindow>,
    active_chain_handle: Weak<AppWindow>,
) {
    // ui.on_fetch_latest_mainnet_checkpoint(move || {
    //     let mut ui = checkpoint_handle
    //         .upgrade()
    //         .expect("Failed to upgrade checkpoint handle");

    //     // Fetch the latest mainnet checkpoint
    //     println!("Fetched latest mainnet checkpoint: {mainnet_checkpoint}");
    //     ui.borrow_mut()
    //         .set_latest_checkpoint(mainnet_checkpoint.to_string().into());
    // });

    // DAPPS ----------------------------------
    ui.on_select_dapp(move |d| {
        let mut ui = active_dapp_handle
            .upgrade()
            .expect("Failed to upgrade active dapp handle");
        ui.borrow_mut().set_active_dapp(d.clone());
        let dapp = ui.borrow().get_active_dapp();
        println!("Active Dapp: {:?}", dapp.name);
    });

    // CHAINS ---------------------------------
    ui.on_select_chain(move |c| {
        let mut ui = active_chain_handle
            .upgrade()
            .expect("Failed to upgrade active chain handle");
        let chain = ui.borrow().get_active_chain();
        ui.borrow_mut().set_active_chain(c.clone());
        println!("Active Chain: {:?}", chain.name);
    });

    // ... Add other UI callbacks here
}
