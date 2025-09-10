// tests/settings_rss_secure_test.rs
// Tests for secure storage of RSS feed credentials
use wixen::settings::Settings;

#[test]
fn test_rss_credentials_storage() {
    let test_path = "test_settings_rss_secure.json";
    let mut settings = Settings::default();
    let username = "rss_user";
    let password = "rss_pass";
    settings.rss_username = Some(username.to_string());
    settings.rss_password = Some(password.to_string());
    settings.save(test_path);

    let loaded = Settings::load(test_path);
    assert_eq!(loaded.rss_username.as_deref(), Some(username));
    assert_eq!(loaded.rss_password.as_deref(), Some(password));
    std::fs::remove_file(test_path).unwrap();
}

#[test]
fn test_rss_credentials_description() {
    let settings = Settings::default();
    assert!(settings.rss_username_desc.contains("RSS feed username"));
    assert!(settings.rss_password_desc.contains("RSS feed password"));
}
