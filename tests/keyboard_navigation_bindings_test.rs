
use accessible_winit_app::{AppState, FocusArea, TestKeyEvent};
use accessible_winit_app::event_handler::handle_key_event_v2_test;
use winit::event::ElementState;
use winit::keyboard::NamedKey;

fn make_key_event(named: NamedKey) -> TestKeyEvent {
    TestKeyEvent {
        named,
        state: ElementState::Pressed,
        text: None,
    }
}

#[test]
fn tab_from_menubar_to_mainlist() {
    let mut state = AppState { focus: FocusArea::Menubar, highlighted_menu: Some(0), highlighted_list: None, submenu_open: None };
    handle_key_event_v2_test(&mut state, &make_key_event(NamedKey::Tab));
    assert_eq!(state.focus, FocusArea::MainList);
}

#[test]
fn tab_from_mainlist_to_menubar() {
    let mut state = AppState { focus: FocusArea::MainList, highlighted_menu: None, highlighted_list: Some(0), submenu_open: None };
    handle_key_event_v2_test(&mut state, &make_key_event(NamedKey::Tab));
    assert_eq!(state.focus, FocusArea::Menubar);
}

#[test]
fn alt_from_mainlist_to_menubar() {
    let mut state = AppState { focus: FocusArea::MainList, highlighted_menu: None, highlighted_list: Some(0), submenu_open: None };
    handle_key_event_v2_test(&mut state, &make_key_event(NamedKey::Alt));
    assert_eq!(state.focus, FocusArea::Menubar);
}

#[test]
fn arrow_left_menu_wraps() {
    let mut state = AppState { focus: FocusArea::Menubar, highlighted_menu: Some(0), highlighted_list: None, submenu_open: None };
    handle_key_event_v2_test(&mut state, &make_key_event(NamedKey::ArrowLeft));
    assert_eq!(state.highlighted_menu, Some(3)); // Wraps to last menu item
}

#[test]
fn arrow_right_menu_wraps() {
    let mut state = AppState { focus: FocusArea::Menubar, highlighted_menu: Some(3), highlighted_list: None, submenu_open: None };
    handle_key_event_v2_test(&mut state, &make_key_event(NamedKey::ArrowRight));
    assert_eq!(state.highlighted_menu, Some(0)); // Wraps to first menu item
}

#[test]
fn enter_opens_submenu() {
    let mut state = AppState { focus: FocusArea::Menubar, highlighted_menu: Some(0), highlighted_list: None, submenu_open: None };
    handle_key_event_v2_test(&mut state, &make_key_event(NamedKey::Enter));
    assert_eq!(state.focus, FocusArea::Submenu(0));
    assert_eq!(state.submenu_open, Some(0));
}

#[test]
fn escape_from_menubar_to_mainlist() {
    let mut state = AppState { focus: FocusArea::Menubar, highlighted_menu: Some(0), highlighted_list: None, submenu_open: None };
    handle_key_event_v2_test(&mut state, &make_key_event(NamedKey::Escape));
    assert_eq!(state.focus, FocusArea::MainList);
}

#[test]
fn escape_from_submenu_to_menubar() {
    let mut state = AppState { focus: FocusArea::Submenu(0), highlighted_menu: Some(0), highlighted_list: None, submenu_open: Some(0) };
    handle_key_event_v2_test(&mut state, &make_key_event(NamedKey::Escape));
    assert_eq!(state.focus, FocusArea::Menubar);
    assert_eq!(state.submenu_open, None);
}

#[test]
fn tab_from_submenu_to_mainlist() {
    let mut state = AppState { focus: FocusArea::Submenu(0), highlighted_menu: Some(0), highlighted_list: None, submenu_open: Some(0) };
    handle_key_event_v2_test(&mut state, &make_key_event(NamedKey::Tab));
    assert_eq!(state.focus, FocusArea::MainList);
    assert_eq!(state.submenu_open, None);
}

#[test]
fn escape_from_mainlist_to_menubar() {
    let mut state = AppState { focus: FocusArea::MainList, highlighted_menu: None, highlighted_list: Some(0), submenu_open: None };
    handle_key_event_v2_test(&mut state, &make_key_event(NamedKey::Escape));
    assert_eq!(state.focus, FocusArea::Menubar);
}
