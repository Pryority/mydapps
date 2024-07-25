use crate::error::Result;
use slint::ComponentHandle;

slint::include_modules!();

pub struct Window {
    window: AppWindow,
}

impl Window {
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
            let window = window_handle_dapp
                .upgrade()
                .expect("Failed to upgrade window handle for dapp");
            window.set_active_dapp(d.clone());
            let dapp = window.get_active_dapp();
            println!("Active Dapp: {:?}", dapp.name);
        });

        let window_handle_chain = self.window.as_weak();
        self.window.on_select_chain(move |c| {
            let window = window_handle_chain
                .upgrade()
                .expect("Failed to upgrade window handle for chain");
            window.set_active_chain(c.clone());
            let chain = window.get_active_chain();
            println!("Active Chain: {:?}", chain.name);
        });
    }

    pub fn set_sync_status(&self, status: bool) {
        self.window.set_sync_status(status);
    }
}
