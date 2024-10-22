use serde_json::{Map, Value};
use tui_tree_widget::TreeState;

#[derive(Debug)]
pub struct AppState {
    pub current_screen: CurrentScreen,
    pub tree_state: TreeState<String>,
    pub config: Option<Map<String, Value>>,
}

impl AppState {
    pub fn new() -> AppState {
        AppState {
            current_screen: CurrentScreen::Main,
            tree_state: TreeState::<String>::default(),
            config: None,
        }
    }
}

#[derive(Debug)]
pub enum CurrentScreen {
    Main,
    Editing,
}
