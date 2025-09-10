// src/visual_settings.rs
// Visual settings and color palette selection

#[derive(Default, Debug, Clone)]
pub struct VisualSettings {
    pub color_palette_index: usize,
}

impl VisualSettings {
    pub fn load() -> Self {
        // TODO: Load from persistent storage (JSON, etc.)
        Self { color_palette_index: 0 }
    }
    pub fn save(&self) {
        // TODO: Save to persistent storage
    }
}
