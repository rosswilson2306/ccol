use serde_json::{Map, Value};
use tui_tree_widget::{TreeItem, TreeState};

use crate::{config::{get_config_dir, get_config_file, parse_config}, error::{CcolError, Result}, ui::json::tree_items};

#[derive(Debug)]
pub struct AppState<'a> {
    pub current_screen: CurrentScreen,
    pub tree_state: TreeState<String>,
    pub config: Option<Map<String, Value>>,
    pub current_json_node: Option<Value>,
    pub tree_items: Vec<TreeItem<'a, String>>,
}

impl<'a> AppState<'a> {
    pub fn new() -> Result<AppState<'a>> {
        let config_dir = get_config_dir()?;
        let config_file = get_config_file(config_dir);
        let json = parse_config(config_file)?;
        let tree_items = tree_items(json.clone())?;

        match json.clone() {
            Value::Object(o) => {
                Ok(AppState {
                    current_screen: CurrentScreen::Main,
                    tree_state: TreeState::<String>::default(),
                    config: Some(o),
                    current_json_node: None,
                    tree_items,
                })

            }
            _ => return Err(CcolError::ParseConfigError),
        }
    }
}

#[derive(Debug)]
pub enum CurrentScreen {
    Main,
    Editing,
}
