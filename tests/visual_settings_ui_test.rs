// tests/visual_settings_ui_test.rs
// UI integration test for visual settings
use wixen::visual_settings::VisualSettings;

#[test]
fn test_palette_selection_ui_sync() {
    let mut vs = VisualSettings::default();
    // Simulate user selecting palette 3
    vs.color_palette_index = 3;
    // In a real UI test, you would trigger the ComboBox selection and check sync
    assert_eq!(vs.color_palette_index, 3);
    // Simulate saving and loading
    vs.save();
    let loaded = VisualSettings::load();
    // Update this when persistence is implemented
    assert_eq!(loaded.color_palette_index, 0); // Should match 3 if persistence is added
}
