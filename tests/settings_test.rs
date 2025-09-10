// tests/settings_test.rs
// Tests for runtime settings changes and validation
use wixen::settings::Settings;

#[test]
fn test_update_and_save() {
    let test_path = "test_settings.json";
    let mut settings = Settings::default();
    settings.update_and_save(|s| s.list_page_step = 42, test_path);
    let loaded = Settings::load(test_path);
    assert_eq!(loaded.list_page_step, 42);
    std::fs::remove_file(test_path).unwrap();
}

#[test]
fn test_import_export() {
    let export_path = "export_settings.json";
    let mut settings = Settings::default();
    settings.list_page_step = 99;
    settings.export(export_path);
    let mut imported = Settings::default();
    imported.import(export_path);
    assert_eq!(imported.list_page_step, 99);
    std::fs::remove_file(export_path).unwrap();
}

#[test]
fn test_validation() {
    let mut settings = Settings::default();
    settings.list_page_step = 0;
    settings.validate();
    assert_eq!(settings.list_page_step, 1);
}
