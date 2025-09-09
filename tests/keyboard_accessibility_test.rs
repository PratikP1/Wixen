use accessible_winit_app::{AppState, FocusArea, MAIN_LIST_ITEMS, TestKeyEvent};
use accessible_winit_app::event_handler::handle_key_event_v2_test;
#[cfg(test)]
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
fn home_and_end_navigate_list() {
    let mut state = AppState::default();
    state.highlighted_list = Some(2);
    handle_key_event_v2_test(&mut state, &make_key_event(NamedKey::Home));
    assert_eq!(state.highlighted_list, Some(0));
    handle_key_event_v2_test(&mut state, &make_key_event(NamedKey::End));
    assert_eq!(state.highlighted_list, Some(MAIN_LIST_ITEMS.len() - 1));
}

// Skipped: page_up_and_down_navigate_list_by_step test, as mutating MAIN_LIST_ITEMS is unsafe and not allowed. Consider refactoring code to allow dependency injection for testability.

#[test]
fn alt_in_menubar_returns_to_list() {
    let mut state = AppState::default();
    state.focus = FocusArea::Menubar;
    state.highlighted_menu = Some(0);
    handle_key_event_v2_test(&mut state, &make_key_event(NamedKey::Alt));
    assert_eq!(state.focus, FocusArea::MainList);
}
