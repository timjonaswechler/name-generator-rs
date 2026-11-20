use super::{consonants::all_consonants, vowels::all_vowels, SoundDefinition, SoundId};
use crate::validation::{ValidationError, ValidationErrors};
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct SoundCatalog {
    sounds: BTreeMap<SoundId, SoundDefinition>,
}
impl SoundCatalog {
    pub fn new(
        sounds: impl IntoIterator<Item = SoundDefinition>,
    ) -> Result<Self, ValidationErrors> {
        let mut entries = BTreeMap::new();
        let mut errors = ValidationErrors::new();
        for sound in sounds {
            if entries
                .values()
                .any(|existing: &SoundDefinition| existing.ipa() == sound.ipa())
            {
                errors.add(
                    sound.id().to_string(),
                    ValidationError::new("duplicate_ipa_notation"),
                );
            }
            if let Err(e) = sound.validate() {
                errors.merge(e);
            }
            if entries.contains_key(sound.id()) {
                errors.add(
                    sound.id().to_string(),
                    ValidationError::new("duplicate_sound_id"),
                );
            } else {
                entries.insert(sound.id().clone(), sound);
            }
        }
        if errors.is_empty() {
            Ok(Self { sounds: entries })
        } else {
            Err(errors)
        }
    }
    /// Existing catalogue entries only; not a claim of complete IPA coverage.
    pub fn builtin() -> Self {
        Self::new(all_consonants().into_iter().chain(all_vowels()).cloned())
            .expect("built-in sound IDs and definitions must be valid")
    }
    pub fn get(&self, id: &SoundId) -> Option<&SoundDefinition> {
        self.sounds.get(id)
    }
    pub fn find_ipa(&self, ipa: &str) -> Option<&SoundDefinition> {
        self.sounds.values().find(|sound| sound.ipa() == ipa)
    }
    pub fn iter(&self) -> impl Iterator<Item = &SoundDefinition> {
        self.sounds.values()
    }
    pub fn len(&self) -> usize {
        self.sounds.len()
    }
    pub fn is_empty(&self) -> bool {
        self.sounds.is_empty()
    }
}
