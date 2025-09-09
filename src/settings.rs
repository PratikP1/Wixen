// src/settings.rs
// Global settings and configuration

#[allow(dead_code)]
pub struct GlobalSettings {
    pub list_page_step: usize,
}

impl Default for GlobalSettings {
    fn default() -> Self {
        GlobalSettings {
            list_page_step: 25,
        }
    }
}

#[allow(dead_code)]
pub static mut GLOBAL_SETTINGS: GlobalSettings = GlobalSettings { list_page_step: 25 };
