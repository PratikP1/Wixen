slint::include_modules!();
use wixen::settings::Settings;
use wixen::visual_settings::VisualSettings;

fn main() {
    // Load settings from JSON file
    let settings_path = "settings.json";
    let settings = Settings::load(settings_path);
    let visual_settings = VisualSettings::load();

    // Example: use a setting
    println!("List page step from settings: {}", settings.list_page_step);
    println!("Selected color palette: {}", visual_settings.color_palette_index);

    // Instantiate and run the Slint UI
    let main_window = MainWindow::new().unwrap();
    main_window.run().unwrap();

    // Save settings back to JSON file (if changed)
    // settings.save(settings_path); // Uncomment if settings are changed in UI
    // visual_settings.save(); // Uncomment if visual settings are changed in UI
}
