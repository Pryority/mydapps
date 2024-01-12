slint::include_modules!();
// slint::slint!(import { DappTile } from "../ui/dapps/dapp_tile.slint";);

fn main() -> Result<(), slint::PlatformError> {
    // Create an instance of the generated component
    let mut ui = AppWindow::new()?;

    // Clone strong handles for properties
    let active_dapp_handle = ui.as_weak();
    let active_chain_handle = ui.as_weak();

    // DAPPS ----------------------------------
    ui.on_select_dapp(move |d| {
        let ui = active_dapp_handle.unwrap();
        ui.set_active_dapp(d.clone());
        let dapp = ui.get_active_dapp();
        println!("Active Dapp: {:?}", dapp.name);
    });

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
        // let balances = [100, 30_000, 42];
        // ui.set_balances(balances.into());
    });

    ui.run()
}
