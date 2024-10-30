use serde_json::{Map, Value};
use tui_tree_widget::{TreeItem, TreeState};

use crate::error::{CcolError, Result};

pub fn tree_items(root: Value) -> Result<Vec<TreeItem<'static, String>>> {
    match root {
        Value::Object(object) => traverse_json_tree(object, "".to_string()),
        _ => Err(CcolError::ParseConfigError),
    }
}

pub fn traverse_json_tree(
    map: Map<String, Value>,
    path: String,
) -> Result<Vec<TreeItem<'static, String>>> {
    let mut items = Vec::new();

    for (key, subtree) in map {
        let new_path = format!("{}/{}", path, key);

        let tree_item = match subtree {
            Value::String(command) => {
                TreeItem::new_leaf(new_path.clone(), format!("{}: {}", key, command))
            }
            Value::Object(o) => {
                let children = traverse_json_tree(o, new_path.clone())?;

                TreeItem::new(key.clone(), key.clone(), children)?
            }
            _ => return Err(CcolError::ParseConfigError),
        };
        items.push(tree_item);
    }

    Ok(items)
}

pub fn get_selected_item<'a>(
    tree_state: &TreeState<String>,
    tree_items: &'a [TreeItem<'a, String>],
) -> Option<&'a TreeItem<'a, String>> {
    let selected = tree_state.selected();
    let last = match selected.last() {
        Some(identifier) => identifier,
        None => "",
    };
    let flattened_items = tree_state.flatten(tree_items);
    let selected_flat_item = flattened_items
        .iter()
        .find(|&flattened| flattened.item.identifier() == last);

    if let Some(selected) = selected_flat_item {
        return Some(selected.item);
    } else {
        return None;
    }
}

pub fn is_selected_item_a_leaf(item: &TreeItem<'_, String>) -> bool {
    item.children().is_empty()
}
