use crate::error::Result;
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
        // Your existing callback setup code here
        // let tx = self.tx.clone();
        // self.window.on_update_checkpoint(move |checkpoint| {
        //     let tx = tx.clone();
        //     tokio::spawn(async move {
        //         let _ = tx
        //             .send(ClientMessage::UpdateCheckpoint(checkpoint.to_string()))
        //             .await;
        //     });
        // });
    }
}
