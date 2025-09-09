use zeroize::Zeroize;

// src/settings.rs
// Settings management using JSON and serde
use serde::{Serialize, Deserialize};
use std::fs;
use std::path::Path;
use keyring::Entry;
use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, KeyInit};
use rand::RngCore;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Settings {
    pub list_page_step: usize,
    // Sensitive fields are not serialized
    #[serde(skip)]
    pub mastodon_access_token: Option<String>,
    #[serde(skip)]
    pub bluesky_access_token: Option<String>,
    #[serde(skip)]
    pub rss_username: Option<String>,
    #[serde(skip)]
    pub rss_password: Option<String>,
    // Descriptions for sensitive fields
    pub mastodon_access_token_desc: String,
    pub bluesky_access_token_desc: String,
    pub rss_username_desc: String,
    pub rss_password_desc: String,
}
impl Drop for Settings {
    fn drop(&mut self) {
        if let Some(ref mut token) = self.mastodon_access_token {
            token.zeroize();
        }
        if let Some(ref mut token) = self.bluesky_access_token {
            token.zeroize();
        }
        if let Some(ref mut username) = self.rss_username {
            username.zeroize();
        }
        if let Some(ref mut password) = self.rss_password {
            password.zeroize();
        }
    }
}


impl Default for Settings {
    fn default() -> Self {
        Settings {
            list_page_step: 25,
            mastodon_access_token: None,
            bluesky_access_token: None,
            rss_username: None,
            rss_password: None,
            mastodon_access_token_desc: "Mastodon API access token (stored securely)".to_string(),
            bluesky_access_token_desc: "Bluesky API access token (stored securely)".to_string(),
            rss_username_desc: "RSS feed username (stored securely)".to_string(),
            rss_password_desc: "RSS feed password (stored securely)".to_string(),
        }
    }
}

impl Settings {
    pub fn load<P: AsRef<Path>>(path: P) -> Self {
        let mut settings = if let Ok(data) = fs::read_to_string(path) {
            serde_json::from_str(&data).unwrap_or_default()
        } else {
            Self::default()
        };
        settings.validate();
    // Load sensitive fields from keyring
    settings.mastodon_access_token = Self::load_secret("mastodon_access_token");
    settings.bluesky_access_token = Self::load_secret("bluesky_access_token");
    settings.rss_username = Self::load_secret("rss_username");
    settings.rss_password = Self::load_secret("rss_password");
        settings
    }
    /// Validate settings values and correct if needed
    pub fn validate(&mut self) {
        if self.list_page_step == 0 {
            self.list_page_step = 1; // Minimum allowed value
        }
        // Add more validation rules for new fields here
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) {
        if let Ok(json) = serde_json::to_string_pretty(self) {
            let _ = fs::write(path, json);
        }
        // Save sensitive fields to keyring
        if let Some(ref token) = self.mastodon_access_token {
            let _ = Self::save_secret("mastodon_access_token", token);
        }
        if let Some(ref token) = self.bluesky_access_token {
            let _ = Self::save_secret("bluesky_access_token", token);
        }
        if let Some(ref username) = self.rss_username {
            let _ = Self::save_secret("rss_username", username);
        }
        if let Some(ref password) = self.rss_password {
            let _ = Self::save_secret("rss_password", password);
        }
    }
    /// Save a secret to the OS keyring, fallback to encrypted file if unavailable
    pub fn save_secret(key: &str, value: &str) -> Result<(), keyring::Error> {
        // Force fallback for test key
        if key == "test_fallback" {
            let path = format!(".{}_secret", key);
            let (encrypted, nonce) = Self::encrypt_value(value);
            let mut data = nonce.to_vec();
            data.extend_from_slice(&encrypted);
            return std::fs::write(path, data).map_err(|e| keyring::Error::PlatformFailure(Box::new(e)));
        }
        match Entry::new("wixen", key) {
            Ok(entry) => match entry.set_password(value) {
                Ok(_) => Ok(()),
                Err(_) => {
                    // Fallback: save to encrypted file using AES-GCM
                    let path = format!(".{}_secret", key);
                    let (encrypted, nonce) = Self::encrypt_value(value);
                    let mut data = nonce.to_vec();
                    data.extend_from_slice(&encrypted);
                    std::fs::write(path, data).map_err(|e| keyring::Error::PlatformFailure(Box::new(e)))
                }
            },
            Err(_) => {
                // Fallback: save to encrypted file using AES-GCM
                let path = format!(".{}_secret", key);
                let (encrypted, nonce) = Self::encrypt_value(value);
                let mut data = nonce.to_vec();
                data.extend_from_slice(&encrypted);
                std::fs::write(path, data).map_err(|e| keyring::Error::PlatformFailure(Box::new(e)))
            }
        }
    }

    /// Load a secret from the OS keyring, fallback to encrypted file if unavailable
    pub fn load_secret(key: &str) -> Option<String> {
        match Entry::new("wixen", key) {
            Ok(entry) => match entry.get_password() {
                Ok(val) => Some(val),
                Err(_) => {
                    // Fallback: load from encrypted file using AES-GCM
                    let path = format!(".{}_secret", key);
                    if let Ok(data) = std::fs::read(path) {
                        if data.len() < 12 { return None; }
                        let nonce = &data[..12];
                        let encrypted = &data[12..];
                        Self::decrypt_value(encrypted, nonce)
                    } else {
                        None
                    }
                }
            },
            Err(_) => {
                // Fallback: load from encrypted file using AES-GCM
                let path = format!(".{}_secret", key);
                if let Ok(data) = std::fs::read(path) {
                    if data.len() < 12 { return None; }
                    let nonce = &data[..12];
                    let encrypted = &data[12..];
                    Self::decrypt_value(encrypted, nonce)
                } else {
                    None
                }
            }
        }
    }

    // Simple key management for demonstration (should be replaced with secure key derivation)
    fn get_encryption_key() -> Key<Aes256Gcm> {
        // In production, derive from user password or OS secret
    let key_bytes = [0u8; 32]; // Replace with secure key
    Key::<Aes256Gcm>::from_slice(&key_bytes).clone()
    }

    fn encrypt_value(value: &str) -> (Vec<u8>, [u8; 12]) {
    let key = Self::get_encryption_key();
    let cipher = Aes256Gcm::new(&key);
        let mut nonce_bytes = [0u8; 12];
        rand::thread_rng().fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);
        let encrypted = cipher.encrypt(nonce, value.as_bytes()).unwrap_or_default();
        (encrypted, nonce_bytes)
    }

    fn decrypt_value(encrypted: &[u8], nonce: &[u8]) -> Option<String> {
    let key = Self::get_encryption_key();
    let cipher = Aes256Gcm::new(&key);
        let nonce = Nonce::from_slice(nonce);
        cipher.decrypt(nonce, encrypted).ok().and_then(|bytes| String::from_utf8(bytes).ok())
    }

    /// Update a setting at runtime and save immediately
    pub fn update_and_save<F, P: AsRef<Path>>(&mut self, update_fn: F, path: P)
    where
        F: FnOnce(&mut Self),
    {
        update_fn(self);
        self.save(path);
    }

    /// Import settings from another JSON file
    pub fn import<P: AsRef<Path>>(&mut self, import_path: P) {
        if let Ok(data) = fs::read_to_string(import_path) {
            if let Ok(imported) = serde_json::from_str::<Settings>(&data) {
                *self = imported;
            }
        }
    }

    /// Export current settings to another JSON file
    pub fn export<P: AsRef<Path>>(&self, export_path: P) {
        self.save(export_path);
    }
}
