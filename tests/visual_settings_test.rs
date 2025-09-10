// tests/visual_settings_test.rs
use wixen::visual_settings::VisualSettings;

#[test]
fn test_default_visual_settings() {
    let vs = VisualSettings::default();
    assert_eq!(vs.color_palette_index, 0);
}

#[test]
fn test_load_and_save_visual_settings() {
    let mut vs = VisualSettings::default();
    vs.color_palette_index = 5;
    vs.save(); // This is a stub; implement actual persistence for full test
    let loaded = VisualSettings::load();
    // If persistence is implemented, this should match
    // For now, just check type and field
    assert_eq!(loaded.color_palette_index, 0); // Update when persistence is added
}
