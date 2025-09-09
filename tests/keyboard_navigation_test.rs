use accessible_winit_app::{AppState, FocusArea};
use accessible_winit_app::event_handler::handle_key_event_v2_test;
use accessible_winit_app::TestKeyEvent;
use accesskit::Role;

#[test]
fn tab_navigates_between_menubar_and_list() {
    let mut state = AppState::default();
    // Tab from list to menubar
    state.focus = FocusArea::MainList;
    handle_key_event_v2_test(&mut state, &TestKeyEvent {
        named: winit::keyboard::NamedKey::Tab,
        state: winit::event::ElementState::Pressed,
        text: None,
    });
    assert_eq!(state.focus, FocusArea::Menubar);
    // Tab from menubar to list
    handle_key_event_v2_test(&mut state, &TestKeyEvent {
        named: winit::keyboard::NamedKey::Tab,
        state: winit::event::ElementState::Pressed,
        text: None,
    });
    assert_eq!(state.focus, FocusArea::MainList);
}

#[test]
fn alt_key_focuses_first_menu_item() {
    let mut state = AppState::default();
    state.focus = FocusArea::MainList;
    handle_key_event_v2_test(&mut state, &TestKeyEvent {
        named: winit::keyboard::NamedKey::Alt,
        state: winit::event::ElementState::Pressed,
        text: None,
    });
    assert_eq!(state.focus, FocusArea::Menubar);
    assert_eq!(state.highlighted_menu, Some(0));
}

#[test]
fn list_view_gets_automatic_focus() {
    let state = AppState::default();
    assert_eq!(state.focus, FocusArea::MainList);
    assert_eq!(state.highlighted_list, Some(0));
}
