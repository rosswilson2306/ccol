pub struct AppState {
    pub current_screen: CurrentScreen,
}

impl AppState {
    pub fn new() -> AppState {
        AppState {
            current_screen: CurrentScreen::Main,
        }
    }
}

pub enum CurrentScreen {
    Main,
    Editing,
}
