// src/i18n.rs
// Translation and localization infrastructure

use std::collections::HashMap;

#[derive(Default)]
pub struct I18n {
    pub language: String,
    pub translations: HashMap<String, String>,
}

impl I18n {
    pub fn new(language: &str) -> Self {
        // In a real app, load translations from file or resource
        let mut translations = HashMap::new();
        match language {
            "en" => {
                translations.insert("menu_file".to_string(), "File".to_string());
                translations.insert("menu_edit".to_string(), "Edit".to_string());
                translations.insert("menu_view".to_string(), "View".to_string());
                translations.insert("menu_help".to_string(), "Help".to_string());
                translations.insert("submenu_exit".to_string(), "Exit".to_string());
                translations.insert("main_list_item".to_string(), "Item".to_string());
            }
            "es" => {
                translations.insert("menu_file".to_string(), "Archivo".to_string());
                translations.insert("menu_edit".to_string(), "Editar".to_string());
                translations.insert("menu_view".to_string(), "Ver".to_string());
                translations.insert("menu_help".to_string(), "Ayuda".to_string());
                translations.insert("submenu_exit".to_string(), "Salir".to_string());
                translations.insert("main_list_item".to_string(), "Elemento".to_string());
            }
            // Add more languages as needed
            _ => {}
        }
        I18n {
            language: language.to_string(),
            translations,
        }
    }

    pub fn t(&self, key: &str) -> String {
        self.translations.get(key).cloned().unwrap_or_else(|| key.to_string())
    }
}
