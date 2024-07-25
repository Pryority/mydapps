use crate::error::Result;
use ethers::types::U256;
use slint::ComponentHandle;

slint::include_modules!();

pub struct UI {
    pub window: AppWindow,
}

impl UI {
    pub fn new() -> Result<Self> {
        let window =
            AppWindow::new().map_err(|e| crate::error::AppError::SlintError(e.to_string()))?;
        Ok(Self { window })
    }

    pub fn run(&self) -> Result<()> {
        self.window
            .run()
            .map_err(|e| crate::error::AppError::SlintError(e.to_string()))?;
        Ok(())
    }

    pub fn setup_ui_callbacks(&self) {
        let window_handle_dapp = self.window.as_weak();
        self.window.on_select_dapp(move |d| {
            if let Some(window) = window_handle_dapp.upgrade() {
                window.set_active_dapp(d.clone());
                let dapp = window.get_active_dapp();
                println!("Active Dapp: {:?}", dapp.name);
            } else {
                eprintln!("Failed to upgrade window handle for dapp");
            }
        });

        let window_handle_chain = self.window.as_weak();
        self.window.on_select_chain(move |c| {
            if let Some(window) = window_handle_chain.upgrade() {
                window.set_active_chain(c.clone());
                let chain = window.get_active_chain();
                println!("Active Chain: {:?}", chain.name);
            } else {
                eprintln!("Failed to upgrade window handle for chain");
            }
        });
    }

    pub fn set_sync_status(&self, status: bool) {
        let window = self.window.as_weak();
        slint::invoke_from_event_loop(move || {
            if let Some(window) = window.upgrade() {
                window.set_sync_status(status);
            } else {
                eprintln!("Failed to upgrade window handle when setting sync status");
            }
        })
        .expect("Failed to invoke from event loop");
    }

    pub fn set_latest_block(&self, block_number: U256) {
        if let Some(block_number_i32) = block_number.as_u32().try_into().ok() {
            let window = self.window.as_weak();
            slint::invoke_from_event_loop(move || {
                if let Some(window) = window.upgrade() {
                    window.set_block_number(block_number_i32);
                } else {
                    eprintln!("Failed to upgrade window handle when setting latest block");
                }
            })
            .expect("Failed to invoke from event loop");
        } else {
            eprintln!("Block number is out of range for i32: {:?}", block_number);
            // Handle the case where block_number cannot be converted
            // You might want to use a default value or log an error
        }
    }
}
