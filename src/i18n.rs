use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A String-like Type which allows for easy Internationalization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct I18nString {
    translations: HashMap<Language, String>,
}

impl std::fmt::Display for I18nString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.translations.get(&Language::Japanese).unwrap().as_str())
    }
}

impl Default for I18nString {
    fn default() -> Self {
        let mut map: HashMap<Language, String> = Default::default();
        let _ = map.insert(Language::Japanese, "".to_string());
        Self { translations: map }
    }
}

impl I18nString {
    /// Blue Archive is a Japanese Game, therefore I18nString's constructor
    /// expects Japanese text.
    pub fn new(jpn_message: &str) -> Self {
        let mut map: HashMap<Language, String> = Default::default();
        let _ = map.insert(Language::Japanese, jpn_message.to_string());

        Self { translations: map }
    }

    /// This method wraps HashMap's insert, which means that:
    /// * If the Language doesn't already have a translation, one will be added
    /// * If a translation for the Language already exists, it will be replaced.
    pub fn update(&mut self, language: Language, message: &str) {
        let _ = self.translations.insert(language, message.to_string());
    }

    /// Get a Language's translation as an Owned String
    ///
    /// Will return None if there is no translation for the given language
    pub fn get(&self, language: Language) -> Option<String> {
        self.translations
            .get(&language)
            .map(|message| message.clone())
    }
}

/// This enum represents all Languages this Gacha Simulator **must** support.
///
/// This enum follows ISO-639-2/T
#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub enum Language {
    #[serde(rename = "eng")]
    English,
    #[serde(rename = "jpn")]
    Japanese,
}
