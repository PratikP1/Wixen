use accesskit::Role;
use accessible_winit_app::{build_accessibility_tree, AppState, FocusArea};

#[test]
fn menubar_is_accessible() {
    let state = AppState {
        focus: FocusArea::Menubar,
        highlighted_menu: Some(0),
        highlighted_list: None,
        submenu_open: None,
    };
    let tree = build_accessibility_tree(&state);
    let menubar_node = tree.nodes.iter().find(|(_, node)| node.role() == Role::MenuBar);
    assert!(menubar_node.is_some(), "Menubar should be present and accessible");
}

#[test]
fn menu_items_are_accessible() {
    let state = AppState {
        focus: FocusArea::Menubar,
        highlighted_menu: Some(0),
        highlighted_list: None,
        submenu_open: None,
    };
    let tree = build_accessibility_tree(&state);
    let menu_items: Vec<_> = tree.nodes.iter().filter(|(_, node)| node.role() == Role::MenuItem).collect();
    assert!(!menu_items.is_empty(), "Menu items should be present and accessible");
}

#[test]
fn main_list_is_accessible() {
    let state = AppState {
        focus: FocusArea::MainList,
        highlighted_menu: None,
        highlighted_list: Some(0),
        submenu_open: None,
    };
    let tree = build_accessibility_tree(&state);
    let list_node = tree.nodes.iter().find(|(_, node)| node.role() == Role::List);
    assert!(list_node.is_some(), "Main list should be present and accessible");
}

#[test]
fn list_items_are_accessible() {
    let state = AppState {
        focus: FocusArea::MainList,
        highlighted_menu: None,
        highlighted_list: Some(0),
        submenu_open: None,
    };
    let tree = build_accessibility_tree(&state);
    let list_items: Vec<_> = tree.nodes.iter().filter(|(_, node)| node.role() == Role::ListItem).collect();
    assert!(!list_items.is_empty(), "List items should be present and accessible");
}
