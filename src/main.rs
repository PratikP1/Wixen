mod dashboard;
mod mastodon_client;
mod bluesky_client;
mod rss_client;
mod mastodon;
mod bluesky;
mod rss;

use dashboard::Dashboard;
use mastodon_client::MastodonClient;
use bluesky_client::BlueskyClient;
use rss_client::RssClient;

mod constants;
mod event_handler;
mod settings;
mod handlers;
mod ui;
mod accessibility;

use constants::{MENU_ITEMS, FILE_SUBMENU_ITEMS, MAIN_LIST_ITEMS};
use handlers::{MyActivationHandler, MyActionHandler, MyDeactivationHandler};
use ui::draw_ui;
use wixen::{AppState, FocusArea};

struct App {
    window: Option<winit::window::Window>,
    state: AppState,
    adapter: Option<Box<accesskit_winit::Adapter>>,
}


use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::WindowId;
use accesskit::{TreeUpdate, Node, NodeId, Role};
use accesskit_winit::Adapter;

impl App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
    let mut attrs = winit::window::WindowAttributes::default();
    attrs.visible = false;
    let window = event_loop.create_window(attrs).unwrap();
        let adapter = Adapter::with_direct_handlers(
            event_loop,
            &window,
            MyActivationHandler,
            MyActionHandler,
            MyDeactivationHandler,
        );
        self.adapter = Some(Box::new(adapter));
        self.window = Some(window);
        if let Some(window) = &self.window {
            window.set_visible(true);
            assert!(window.id() != WindowId::dummy(), "Main window should be created successfully");
            draw_ui(window, &self.state);
        }
        if let Some(window) = &self.window {
            assert!(window.id() != WindowId::dummy(), "Main window should be created successfully");
            draw_ui(window, &self.state);
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: winit::event::WindowEvent) {
        if let Some(window) = &self.window {
            if window.id() == id {
                match event {
                    winit::event::WindowEvent::CloseRequested => {
                        println!("Window close requested. Exiting.");
                        event_loop.exit();
                    }
                    #[allow(unused_variables)]
                    winit::event::WindowEvent::KeyboardInput { event, .. } => {
                            // Event handling is now fully delegated to event_handler::handle_key_event_v2
                            #[cfg(not(test))]
                            event_handler::handle_key_event_v2(&mut self.state, &event);
                        draw_ui(window, &self.state);
                        // Update accessibility tree for screen readers
                        if let Some(adapter) = &mut self.adapter {
                            adapter.update_if_active(|| build_accessibility_tree(&self.state));
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}

// Implement ApplicationHandler for App
impl winit::application::ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        App::resumed(self, event_loop);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: winit::event::WindowEvent,
    ) {
        App::window_event(self, event_loop, window_id, event);
    }
}

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let mut app = App {
        window: None,
        state: AppState {
            focus: FocusArea::None,
            highlighted_menu: None,
            highlighted_list: Some(0),
            submenu_open: None,
        },
        adapter: None,
    };

    // Example usage: create dashboard and add columns
    let mut dashboard = Dashboard::new();

    // Fetch timelines/feeds and add to dashboard
    let mastodon_client = MastodonClient::new("https://mastodon.social", "ACCESS_TOKEN");
    let mastodon_timeline = mastodon_client.fetch_timeline();
    dashboard.add_mastodon_column(mastodon_timeline);

    let bluesky_client = BlueskyClient::new("https://bsky.app", "ACCESS_TOKEN");
    let bluesky_feed = bluesky_client.fetch_feed(bluesky::BlueskyFeedType::Timeline);
    dashboard.add_bluesky_column(bluesky_feed);

    let rss_feed = RssClient::fetch_feed("https://example.com/rss.xml");
    dashboard.add_rss_column(rss_feed);

    dashboard.render();

    event_loop.run_app(&mut app).expect("Failed to run app");
}

// --- Accessibility tree builder ---
fn build_accessibility_tree(state: &AppState) -> TreeUpdate {
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
                // Removed set_action_descriptions (not in AccessKit API)
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
    // List view gets focus on startup
    if state.focus == FocusArea::MainList {
        list.set_live(accesskit::Live::Polite);
    }
    nodes.push((list_id, list));

    // Window root node
    let mut window_node = Node::new(Role::Window);
    window_node.set_label("Wixen");
    window_node.set_children(vec![menubar_id, list_id]);
    // Announce window title
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











