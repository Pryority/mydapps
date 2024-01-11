slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    let ui_handle = ui.as_weak();

    ui.on_request_open_dapp(move || {
        let ui = ui_handle.unwrap();
        println!("{}", ui.get_dapp());
        ui.set_dapp(ui.get_dapp());
    });

    ui.on_request_send_tokens(move || {
        println!("Sent 0.25 ETH to vitalik.eth!");
    });

    ui.on_request_all_balances(move || {
        println!("100 ETH, 30,000 OP, 4.2 BTC");
        // let balances = [100, 30_000, 42];
        // ui.set_balances(balances.into());
    });

    ui.run()
}
