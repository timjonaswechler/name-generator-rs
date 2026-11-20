//! Written spelling is a separate, explicit presentation layer.
use crate::phonology::{LanguageInventory, SoundCatalog, SoundId};
use crate::prosody::{spelling::ProsodySpelling, ProsodyRules};
use crate::syllables::WordForm;
use crate::validation::{ValidationError, ValidationErrors};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Orthography {
    pub mappings: BTreeMap<SoundId, String>,
    #[serde(default)]
    pub prosody: ProsodySpelling,
}
impl Orthography {
    pub fn validate(
        &self,
        catalog: &SoundCatalog,
        inventory: &LanguageInventory,
        prosody: &ProsodyRules,
    ) -> Result<(), ValidationErrors> {
        let mut errors = ValidationErrors::new();
        if let Err(e) = self.prosody.validate(prosody) {
            errors.merge(e);
        }
        for id in inventory.iter() {
            if !self.mappings.contains_key(id) {
                errors.add(
                    id.to_string(),
                    ValidationError::new("missing_spelling_mapping"),
                );
            }
        }
        for (id, text) in &self.mappings {
            if catalog.get(id).is_none() {
                errors.add(
                    id.to_string(),
                    ValidationError::new("unknown_spelling_sound"),
                );
            }
            if text.trim().is_empty() {
                errors.add(
                    id.to_string(),
                    ValidationError::new("empty_spelling_mapping"),
                );
            }
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
    pub fn render(&self, word: &WordForm) -> Result<String, ValidationErrors> {
        let mut errors = ValidationErrors::new();
        let mut result = String::new();
        for syllable in word.syllables() {
            match self.prosody.stress(syllable.prosody.stress) {
                Ok(text) => result.push_str(text),
                Err(code) => errors.add("spelling", ValidationError::new(code)),
            }
            for segment in syllable.segments() {
                match self.mappings.get(&segment.sound) {
                    Some(base) if !base.trim().is_empty() => {
                        match self.prosody.segment(base, segment.duration) {
                            Ok(text) => result.push_str(&text),
                            Err(code) => errors.add("spelling", ValidationError::new(code)),
                        }
                    }
                    _ => errors.add(
                        segment.sound.to_string(),
                        ValidationError::new("missing_spelling_mapping"),
                    ),
                }
            }
            if let Some(tone) = &syllable.prosody.tone {
                match self.prosody.tone(tone) {
                    Ok(text) => result.push_str(text),
                    Err(code) => errors.add("spelling", ValidationError::new(code)),
                }
            }
        }
        if errors.is_empty() {
            Ok(result)
        } else {
            Err(errors)
        }
    }
}
