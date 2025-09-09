pub use handlers::MyActivationHandler;
// src/lib.rs
// Shared types and functions for app and tests

mod constants;
pub mod event_handler;


pub use self::constants::{MENU_ITEMS, FILE_SUBMENU_ITEMS, MAIN_LIST_ITEMS};
#[cfg(test)]
pub use self::event_handler::handle_key_event_v2_test;
mod handlers;
pub use handlers::{MyActionHandler, MyDeactivationHandler};
pub use self::event_handler::TestKeyEvent;

#[derive(Debug, PartialEq, Eq)]
pub enum FocusArea {
	None,
	Menubar,
	Submenu(usize),
	MainList,
}

#[derive(Debug)]
pub struct AppState {
	pub focus: FocusArea,
	pub highlighted_menu: Option<usize>,
	pub highlighted_list: Option<usize>,
	pub submenu_open: Option<usize>,
}

impl Default for AppState {
	fn default() -> Self {
		AppState {
			focus: FocusArea::None,
			highlighted_menu: None,
			highlighted_list: Some(0),
			submenu_open: None,
		}
	}
}

use accesskit::{TreeUpdate, Node, NodeId, Role};

pub fn build_accessibility_tree(state: &AppState) -> TreeUpdate {
	let mut nodes = Vec::new();
	let window_id = NodeId::from(0);
	let menubar_id = NodeId::from(1);
	let list_id = NodeId::from(2);

	// Menu items
	let mut menu_children = Vec::new();
	let menu_count = MENU_ITEMS.len();
	for (i, &(item, neumonic)) in MENU_ITEMS.iter().enumerate() {
		let node_id = NodeId::from(10 + i as u64);
		menu_children.push(node_id);
		let mut menu_item = Node::new(Role::MenuItem);
		let mut label = format!("{} (Alt+{})", item, neumonic.to_ascii_uppercase());
		// Announce index and total when focused
		if state.highlighted_menu == Some(i) && state.focus == FocusArea::Menubar {
			label = format!("{} ({}/{}) (Alt+{})", item, i + 1, menu_count, neumonic.to_ascii_uppercase());
			// If submenu is open for this item, announce submenu role
			if state.submenu_open == Some(i) {
				label = format!("{} ({}/{}) - Submenu (Alt+{})", item, i + 1, menu_count, neumonic.to_ascii_uppercase());
			}
			menu_item.set_live(accesskit::Live::Polite);
		}
		menu_item.set_label(label.as_str());
		// Add submenu for File
		if i == 0 {
			let mut submenu_children = Vec::new();
			for (j, &(subitem, subneumonic)) in FILE_SUBMENU_ITEMS.iter().enumerate() {
				let sub_id = NodeId::from(20 + j as u64);
				submenu_children.push(sub_id);
				let mut submenu_item = Node::new(Role::MenuItem);
				let mut sublabel = format!("{} ({}): Press '{}' to activate", subitem, "Exit", subneumonic.to_ascii_uppercase());
				if state.focus == FocusArea::Submenu(i) {
					sublabel = format!("{} ({}): Press '{}' to activate - Focused", subitem, "Exit", subneumonic.to_ascii_uppercase());
					submenu_item.set_live(accesskit::Live::Polite);
				}
				submenu_item.set_label(sublabel.as_str());
				submenu_item.set_children(vec![]);
				nodes.push((sub_id, submenu_item));
			}
			menu_item.set_children(submenu_children);
		}
		nodes.push((node_id, menu_item));
	}
	let mut menubar = Node::new(Role::MenuBar);
	let menubar_label = format!("Menubar: Main Menu ({} items)", menu_count);
	menubar.set_label(menubar_label.as_str());
	menubar.set_children(menu_children.clone());
	if state.focus == FocusArea::Menubar {
		menubar.set_live(accesskit::Live::Polite);
	}
	nodes.push((menubar_id, menubar));

	// List items
	let mut list_children = Vec::new();
	for (i, &item) in MAIN_LIST_ITEMS.iter().enumerate() {
		let node_id = NodeId::from(100 + i as u64);
		list_children.push(node_id);
		let mut list_item = Node::new(Role::ListItem);
		// Announce role and index when focused
		if state.highlighted_list == Some(i) && state.focus == FocusArea::MainList {
			let label = format!("List Item: {} ({}/{}) - Focused", item, i + 1, MAIN_LIST_ITEMS.len());
			list_item.set_label(label.as_str());
			list_item.set_live(accesskit::Live::Polite);
		} else {
			let label = format!("List Item: {}", item);
			list_item.set_label(label.as_str());
		}
		nodes.push((node_id, list_item));
	}
	let mut list = Node::new(Role::List);
	let list_label = format!("List: Main List ({} items)", MAIN_LIST_ITEMS.len());
	list.set_label(list_label.as_str());
	list.set_children(list_children.clone());
	if state.focus == FocusArea::MainList {
		list.set_live(accesskit::Live::Polite);
	}
	nodes.push((list_id, list));

	// Window root node
	let mut window_node = Node::new(Role::Window);
	window_node.set_label("Wixen");
	window_node.set_children(vec![menubar_id, list_id]);
	window_node.set_live(accesskit::Live::Polite);
	nodes.push((window_id, window_node));

	// Focus: list view if focused, else window
	let focus_id = if state.focus == FocusArea::MainList {
		list_id
	} else if let Some(idx) = state.highlighted_menu {
		NodeId::from(10 + idx as u64)
	} else {
		window_id
	};

	TreeUpdate {
		tree: Some(accesskit::Tree {
			root: window_id,
			toolkit_name: Some("Winit".to_string()),
			toolkit_version: Some(env!("CARGO_PKG_VERSION").to_string()),
		}),
		nodes,
		focus: focus_id,
	}
}
