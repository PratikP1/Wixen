

Made changes.

// tests/settings_aes_fallback_test.rs
// Tests for AES-GCM fallback encryption for sensitive settings
use wixen::settings::Settings;
use std::fs;

#[test]
fn test_aes_fallback_encryption() {
    let test_path = "test_settings_aes_fallback.json";
    let key = "mastodon_access_token";
    let secret_file = ".mastodon_access_token_secret";
    let token = "aes_fallback_token_123";
    // Save fallback using AES-GCM
    let mut settings = Settings::default();
    settings.mastodon_access_token = Some(token.to_string());
    // Simulate keyring failure by saving directly to fallback
    let _ = Settings::save_secret(key, token);
    let loaded = Settings::load(test_path);
    assert_eq!(loaded.mastodon_access_token.as_deref(), Some(token));
    fs::remove_file(test_path).unwrap_or(());
    fs::remove_file(secret_file).unwrap_or(());
}
