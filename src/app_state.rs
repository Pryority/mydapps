use crate::client::Helios;
use crate::error::Result;

use crate::ui::Window;

pub struct AppState {
    pub ui: Window,
    pub client: Helios,
}

impl AppState {
    pub async fn new() -> Result<Self> {
        let ui = Window::new()?;
        let client = Helios::new()?;

        Ok(AppState { ui, client })
    }
}
