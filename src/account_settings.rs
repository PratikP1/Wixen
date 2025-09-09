// src/account_settings.rs
// Per-account settings and configuration

#[derive(Default)]
pub struct AccountSettings {
    pub username: String,
    pub display_name: String,
    pub avatar_url: Option<String>,
    pub notifications_enabled: bool,
    pub language: Option<String>, // For future localization
    // Add more fields as needed
}
