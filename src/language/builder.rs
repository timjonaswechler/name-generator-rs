use super::*;
use crate::anatomy::production::PossibleSoundSet;
use crate::syllables::{Syllable, WordForm};
use crate::validation::{ValidationError, ValidationErrors};

impl LanguageDraft {
    pub fn build(self, catalog: &SoundCatalog) -> Result<LanguageProfile, ValidationErrors> {
        let mut errors = ValidationErrors::new();
        if self.name.trim().is_empty() {
            errors.add("name", ValidationError::new("empty_language_name"));
        }
        let anatomy_report = match self.anatomy.validate_anatomical_consistency() {
            Ok(report) => report,
            Err(e) => {
                errors.merge(e);
                return Err(errors);
            }
        };
        let possible = PossibleSoundSet::derive(catalog, &self.anatomy)?;
        let inventory = match LanguageInventory::new(self.inventory, catalog, &possible) {
            Ok(inventory) => inventory,
            Err(e) => {
                errors.merge(e);
                return Err(errors);
            }
        };
        if let Err(e) = self.phonotactics.validate(catalog, &inventory) {
            errors.merge(e);
        }
        if let Err(e) =
            self.prosody
                .validate(&self.anatomy, catalog, &inventory, &self.phonotactics)
        {
            errors.merge(e);
        }
        if let Some(spelling) = &self.orthography {
            if let Err(e) = spelling.validate(catalog, &inventory, &self.prosody) {
                errors.merge(e);
            }
        }
        if !errors.is_empty() {
            return Err(errors);
        }
        Ok(LanguageProfile {
            name: self.name,
            anatomy: self.anatomy,
            anatomy_report,
            catalog: catalog.clone(),
            inventory,
            phonotactics: self.phonotactics,
            prosody: self.prosody,
            orthography: self.orthography,
        })
    }
}
impl LanguageProfile {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn anatomy(&self) -> &SpeakerAnatomy {
        &self.anatomy
    }
    pub fn anatomy_report(&self) -> &AnatomyReport {
        &self.anatomy_report
    }
    pub fn inventory(&self) -> &LanguageInventory {
        &self.inventory
    }
    pub fn phonotactics(&self) -> &Phonotactics {
        &self.phonotactics
    }
    pub fn prosody(&self) -> &ProsodyRules {
        &self.prosody
    }
    pub fn orthography(&self) -> Option<&Orthography> {
        self.orthography.as_ref()
    }

    pub fn validate_word(&self, syllables: Vec<Syllable>) -> Result<WordForm, ValidationErrors> {
        self.check_word(&syllables)?;
        Ok(WordForm { syllables })
    }
    fn check_word(&self, syllables: &[Syllable]) -> Result<(), ValidationErrors> {
        let mut errors = ValidationErrors::new();
        if let Err(e) = self.phonotactics.check_word(syllables) {
            errors.merge(e);
        }
        if let Err(e) = self.prosody.check_word(syllables, &self.catalog) {
            errors.merge(e);
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
    pub fn ipa(&self, word: &WordForm) -> Result<String, ValidationErrors> {
        self.check_word(word.syllables())?;
        let mut result = String::new();
        for syllable in word.syllables() {
            result.push_str(syllable.prosody.stress.ipa());
            for segment in syllable.segments() {
                let sound = self
                    .catalog
                    .get(&segment.sound)
                    .expect("validated inventory");
                result.push_str(sound.ipa());
                result.push_str(segment.duration.ipa());
            }
            if let Some(tone) = &syllable.prosody.tone {
                result.push_str(&tone.ipa());
            }
        }
        Ok(result)
    }
    pub fn spell(&self, word: &WordForm) -> Result<String, ValidationErrors> {
        self.check_word(word.syllables())?;
        let spelling = self.orthography.as_ref().ok_or_else(|| {
            let mut errors = ValidationErrors::new();
            errors.add("orthography", ValidationError::new("no_spelling_profile"));
            errors
        })?;
        spelling.render(word)
    }
    /// Return to editable input; changing anatomy or rules requires rebuilding.
    pub fn to_draft(&self) -> LanguageDraft {
        LanguageDraft {
            name: self.name.clone(),
            anatomy: self.anatomy.clone(),
            inventory: self.inventory.iter().cloned().collect(),
            phonotactics: self.phonotactics.clone(),
            prosody: self.prosody.clone(),
            orthography: self.orthography.clone(),
        }
    }
}
