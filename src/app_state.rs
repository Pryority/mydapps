use crate::client::Helios;
use crate::error::Result;

use crate::ui::UI;

pub struct AppState {
    pub ui: UI,
    pub client: Helios,
}

impl AppState {
    pub async fn new() -> Result<Self> {
        let ui = UI::new()?;
        let client = Helios::new()?;

        Ok(AppState { ui, client })
    }
}
