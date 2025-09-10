// tests/settings_secure_test.rs
// Tests for secure storage of sensitive settings
use wixen::settings::Settings;

#[test]
fn test_sensitive_token_storage() {
    let test_path = "test_settings_secure.json";
    let mut settings = Settings::default();
    let mastodon_token = "mastodon_test_token_123";
    let bluesky_token = "bluesky_test_token_456";
    settings.mastodon_access_token = Some(mastodon_token.to_string());
    settings.bluesky_access_token = Some(bluesky_token.to_string());
    settings.save(test_path);

    let loaded = Settings::load(test_path);
    assert_eq!(loaded.mastodon_access_token.as_deref(), Some(mastodon_token));
    assert_eq!(loaded.bluesky_access_token.as_deref(), Some(bluesky_token));
    std::fs::remove_file(test_path).unwrap();
}

#[test]
fn test_sensitive_token_description() {
    let settings = Settings::default();
    assert!(settings.mastodon_access_token_desc.contains("Mastodon API access token"));
    assert!(settings.bluesky_access_token_desc.contains("Bluesky API access token"));
}
