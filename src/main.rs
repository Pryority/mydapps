slint::include_modules!();
// slint::slint!(import { DappTile } from "../ui/dapps/dapp_tile.slint";);

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    let ui_handle = ui.as_weak();

    // let dapp_tile = DappTile::new()?;
    // let dapp_tile_handle = dapp_tile.as_weak();

    // DAPPS ----------------------------------

    // ui.on_request_open_dapp(move || {
    //     let ui = ui_handle.unwrap();
    //     // println!("{}", ui.get_active_dapp());
    //     ui.set_active_dapp(ui.get_active_dapp());
    // });

    ui.on_select_dapp(move |d| {
        let ui = ui_handle.unwrap();
        ui.set_active_dapp(d.clone());
        let dapp = ui.get_active_dapp();
        // println!("main.rs Passed-In dapp: {:?}", d);
        println!("Active Dapp: {:?}", dapp.name);
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
