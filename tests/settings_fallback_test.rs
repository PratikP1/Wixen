// tests/settings_fallback_test.rs
// Tests for fallback to encrypted file storage when OS keyring is unavailable
use wixen::settings::Settings;
use std::fs;
use base64::{engine::general_purpose, Engine as _};

#[test]
fn test_fallback_file_storage() {
    let test_path = "test_settings_fallback.json";
    let secret_file = ".test_fallback_secret";
    // Ensure clean state before test
    let _ = fs::remove_file(test_path);
    let _ = fs::remove_file(secret_file);
    let token = "fallback_token_789";
    // Test fallback path (forced)
    let _ = Settings::save_secret("test_fallback", token);
    assert!(fs::metadata(secret_file).is_ok(), "Fallback secret file was not created");
    let loaded_token = Settings::load_secret("test_fallback");
    assert_eq!(loaded_token.as_deref(), Some(token));
    let _ = fs::remove_file(secret_file);

    // Test normal keyring path (should not create fallback file)
    let _ = Settings::save_secret("mastodon_access_token", token);
    let mastodon_secret_file = ".mastodon_access_token_secret";
    // If keyring works, fallback file may not exist
    let loaded_token_keyring = Settings::load_secret("mastodon_access_token");
    assert_eq!(loaded_token_keyring.as_deref(), Some(token));
    let _ = fs::remove_file(mastodon_secret_file);
}
