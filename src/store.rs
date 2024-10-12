use tui_tree_widget::TreeState;

#[derive(Debug)]
pub struct AppState {
    pub current_screen: CurrentScreen,
    pub tree_state: TreeState<String>,
}

impl AppState {
    pub fn new(tree_state: TreeState<String>) -> AppState {
        AppState {
            current_screen: CurrentScreen::Main,
            tree_state,
        }
    }
}

#[derive(Debug)]
pub enum CurrentScreen {
    Main,
    Editing,
}
