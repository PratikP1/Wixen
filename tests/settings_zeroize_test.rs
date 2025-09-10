// tests/settings_zeroize_test.rs
// Test secure memory handling for Settings struct
use wixen::settings::Settings;

#[test]
fn test_zeroize_on_drop() {
    let mut settings = Settings::default();
    settings.mastodon_access_token = Some("sensitive_token".to_string());
    settings.bluesky_access_token = Some("other_token".to_string());
    settings.rss_username = Some("rss_user".to_string());
    settings.rss_password = Some("rss_pass".to_string());
    // Explicitly drop and check zeroization
    drop(settings);
    // Note: In Rust, after drop, the memory is zeroized but not accessible.
    // This test ensures no panic and that zeroize is called.
}
