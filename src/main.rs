mod dashboard;
mod mastodon_client;
mod bluesky_client;
mod rss_client;
mod mastodon;
mod bluesky;
mod rss;

mod constants;
mod settings;
mod handlers;
mod ui;
// Removed event_handler and accessibility logic as part of Slint migration
// use ui::draw_ui; // This line is also removed
use handlers::{MyActivationHandler, MyActionHandler, MyDeactivationHandler};
use wixen::{AppState, FocusArea};

struct App {
    window: Option<winit::window::Window>,
    state: AppState,
    adapter: Option<Box<accesskit_winit::Adapter>>,
}

use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::WindowId;
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
    window.set_visible(true);
    self.window = Some(window);
    if let Some(window) = &self.window {
        assert!(window.id() != WindowId::dummy(), "Main window should be created successfully");
        // Removed: draw_ui(window, &self.state);
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
                        // Removed: draw_ui(window, &self.state);
                        // Removed: Update accessibility tree for screen readers
                        // if let Some(adapter) = &mut self.adapter {
                        //     adapter.update_if_active(|| build_accessibility_tree(&self.state));
                        // }
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
    // Load settings from JSON file
    let settings_path = "settings.json";
    let mut settings = settings::Settings::load(settings_path);

    // Example: use a setting
    println!("List page step from settings: {}", settings.list_page_step);

    let event_loop = EventLoop::new().unwrap();
    let mut app = App {
        window: None,
        state: AppState {
            focus: FocusArea::MainList,
            highlighted_menu: None,
            highlighted_list: Some(0),
            submenu_open: None,
        },
        adapter: None,
    };

    // Save settings back to JSON file (if changed)
    settings.save(settings_path);
    event_loop.run_app(&mut app).expect("Failed to run app");
}
