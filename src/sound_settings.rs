// src/sound_settings.rs
// Sound and notification settings

#[derive(Default)]
pub struct SoundSettings {
    pub enable_sounds: bool,
    pub notification_volume: u8, // 0-100
    pub mute: bool,
    // Add more fields as needed
}
