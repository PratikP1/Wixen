// src/feed_settings.rs
// Per-feed settings and configuration

#[derive(Default)]
pub struct FeedSettings {
    pub feed_url: String,
    pub refresh_interval_secs: u32,
    pub show_images: bool,
    pub filter_keywords: Vec<String>,
    pub language: Option<String>, // For future localization
    // Add more fields as needed
}
