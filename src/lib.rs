pub use handlers::MyActivationHandler;
// src/lib.rs
// Shared types and functions for app and tests

mod constants;
pub mod settings;
pub mod event_handler;

pub use self::constants::{MENU_ITEMS, FILE_SUBMENU_ITEMS, MAIN_LIST_ITEMS};
mod handlers;
pub use handlers::{MyActionHandler, MyDeactivationHandler};

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


