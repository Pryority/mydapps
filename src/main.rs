slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    let ui_handle = ui.as_weak();

    ui.on_request_open_dapp(move || {
        let ui = ui_handle.unwrap();
        // println!("{}", ui.get_active_dapp());
        ui.set_active_dapp(ui.get_active_dapp());
    });

    // ui.on_request_active_chain(move || {
    //     let ui = ui_handle.unwrap();
    //     println!("{}", ui.get_active_chain());
    //     ui.get_active_chain();
    // });

    ui.on_request_send_tokens(move || {
        println!("Initiating SEND sequence.");
    });

    // ui.on_request_dapp_config(move |d| {
    //     let ui = ui_handle.unwrap();

    //     // match the dapp passed in as 'd' in the closure to list of dapps
    //     let selected_dapp = DappData {
    //         name: d.name,
    //         logo: d.logo,
    //         url: d.url,
    //     };
    //     // println!("Getting dapp config: {:#?}", selected_dapp);
    //     let dapp_result = format!(
    //         "Name: {} | Logo: {} | URL: {}",
    //         selected_dapp.name, selected_dapp.logo, selected_dapp.url
    //     );
    //     println!("{}", dapp_result);
    //     ui.set_dapp(selected_dapp);
    // });

    ui.on_request_all_balances(move || {
        println!("100 ETH, 30,000 OP, 4.2 BTC");
        // let balances = [100, 30_000, 42];
        // ui.set_balances(balances.into());
    });

    ui.run()
}
