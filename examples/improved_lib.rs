//! Name Generator Library - Clean Public API
//!
//! Linguistically accurate name generation with modular, maintainable structure.

#![allow(dead_code)]

// Private modules - kein `pub` bedeutet sie sind nicht von außen sichtbar
mod anatomy;
mod language;
mod phonology;
mod syllables;
mod validation;

// Nur das re-exportieren, was wirklich nach außen gehört
pub use anatomy::SpeakerAnatomy;
pub use language::LanguageConfiguration;
pub use phonology::PhonologyConfiguration;
pub use syllables::SyllableConfiguration;
pub use validation::{ValidationError, ValidationErrors};

// Builder für einfache Erstellung
pub use anatomy::builder::SpeakerAnatomyBuilder;

// Convenience functions
pub use phonology::all_consonants;
pub use phonology::all_vowels;

// Presets für häufige Use Cases
pub mod presets {
    use super::*;

    pub fn human_speaker() -> SpeakerAnatomy {
        SpeakerAnatomyBuilder::human().build()
    }

    pub fn basic_phonology() -> PhonologyConfiguration {
        PhonologyConfiguration::new()
            .add_vowels(&["a", "e", "i", "o", "u"])
            .add_consonants(&["p", "t", "k", "m", "n"])
    }
}
