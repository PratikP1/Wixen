// src/event_handler.rs
// Keyboard and UI event handling logic
use super::{AppState, FocusArea};

pub struct TestKeyEvent {
    pub named: NamedKey,
    pub state: winit::event::ElementState,
    pub text: Option<String>,
}

#[cfg(not(test))]
use winit::event::KeyEvent;
#[cfg(not(test))]
use winit::keyboard::Key;
use winit::keyboard::NamedKey;
use crate::constants::MENU_ITEMS;

#[cfg(not(test))]
pub fn handle_key_event_v2(state: &mut AppState, event: &KeyEvent) {
    let named = match &event.logical_key {
        Key::Named(n) => n,
        _ => return,
    };
    let state_val = event.state;
    let text_val = event.text.as_ref().map(|s| s.to_string());
    handle_key_event_v2_inner(state, named, state_val, text_val);
}

#[allow(dead_code)]
pub fn handle_key_event_v2_test(state: &mut AppState, event: &TestKeyEvent) {
    handle_key_event_v2_inner(state, &event.named, event.state, event.text.clone());
}

fn handle_key_event_v2_inner(state: &mut AppState, named: &NamedKey, state_val: winit::event::ElementState, text_val: Option<String>) {
    match state.focus {
        FocusArea::None => {
            state.focus = FocusArea::MainList;
            state.highlighted_list = Some(0);
        }
        FocusArea::Menubar => {
            match named {
                NamedKey::Tab if state_val == winit::event::ElementState::Pressed => {
                    state.focus = FocusArea::MainList;
                }
                NamedKey::Alt if state_val == winit::event::ElementState::Pressed => {
                    state.focus = FocusArea::MainList;
                    state.highlighted_menu = None;
                    state.submenu_open = None;
                }
                NamedKey::ArrowLeft if state_val == winit::event::ElementState::Pressed => {
                    if let Some(idx) = state.highlighted_menu {
                        let new_idx = if idx == 0 { MENU_ITEMS.len() - 1 } else { idx - 1 };
                        state.highlighted_menu = Some(new_idx);
                    }
                }
                NamedKey::ArrowRight if state_val == winit::event::ElementState::Pressed => {
                    if let Some(idx) = state.highlighted_menu {
                        let new_idx = if idx == MENU_ITEMS.len() - 1 { 0 } else { idx + 1 };
                        state.highlighted_menu = Some(new_idx);
                    }
                }
                NamedKey::Enter | NamedKey::Space if state_val == winit::event::ElementState::Pressed => {
                    if let Some(idx) = state.highlighted_menu {
                        state.focus = FocusArea::Submenu(idx);
                        state.submenu_open = Some(idx);
                    }
                }
                NamedKey::Escape if state_val == winit::event::ElementState::Pressed => {
                    state.focus = FocusArea::MainList;
                    state.highlighted_menu = None;
                    state.submenu_open = None;
                }
                _ if state_val == winit::event::ElementState::Pressed && text_val.is_some() && state.focus == FocusArea::Menubar => {
                    let c = text_val.unwrap().chars().next().unwrap_or('\0').to_ascii_lowercase();
                    for (i, &(_, neumonic)) in MENU_ITEMS.iter().enumerate() {
                        if c == neumonic {
                            state.focus = FocusArea::Submenu(i);
                            state.submenu_open = Some(i);
                            state.highlighted_menu = Some(i);
                        }
                    }
                }
                _ => {}
            }
        }
        FocusArea::Submenu(_idx) => {
            match named {
                NamedKey::Escape if state_val == winit::event::ElementState::Pressed => {
                    state.focus = FocusArea::Menubar;
                    state.submenu_open = None;
                }
                NamedKey::Tab if state_val == winit::event::ElementState::Pressed => {
                    state.focus = FocusArea::MainList;
                    state.submenu_open = None;
                }
                _ => {}
            }
        }
        FocusArea::MainList => {
            match named {
                NamedKey::Tab if state_val == winit::event::ElementState::Pressed => {
                    state.focus = FocusArea::Menubar;
                }
                NamedKey::Escape if state_val == winit::event::ElementState::Pressed => {
                    state.focus = FocusArea::Menubar;
                }
                NamedKey::Home if state_val == winit::event::ElementState::Pressed => {
                    state.highlighted_list = Some(0);
                }
                NamedKey::End if state_val == winit::event::ElementState::Pressed => {
                    state.highlighted_list = Some(crate::MAIN_LIST_ITEMS.len() - 1);
                }
                _ => {}
            }
        }
    }
}
