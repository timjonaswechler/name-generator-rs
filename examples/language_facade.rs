//! Language Builder Facade - Simplified API for complex language configuration

use crate::{
    anatomy::SpeakerAnatomy,
    phonology::PhonologyConfiguration,
    syllables::SyllableConfiguration,
    validation::{ValidationError, ValidationErrors},
};

/// Public facade that hides the complexity of the type-state pattern
pub struct LanguageBuilder {
    name: String,
    anatomy: Option<SpeakerAnatomy>,
    phonology: Option<PhonologyConfiguration>,
    syllables: Option<Box<dyn SyllableConfigTrait>>,
}

/// Trait to hide the complex syllable type states
trait SyllableConfigTrait {
    fn validate(&self) -> Result<(), ValidationErrors>;
    // Add other common methods as needed
}

impl LanguageBuilder {
    /// Create a new language builder
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            anatomy: None,
            phonology: None,
            syllables: None,
        }
    }

    /// Add speaker anatomy (simplified interface)
    pub fn with_anatomy(mut self, anatomy: SpeakerAnatomy) -> Self {
        self.anatomy = Some(anatomy);
        self
    }

    /// Add human anatomy (convenience method)
    pub fn with_human_anatomy(mut self) -> Self {
        self.anatomy = Some(SpeakerAnatomy::human());
        self
    }

    /// Add phonology configuration
    pub fn with_phonology(mut self, phonology: PhonologyConfiguration) -> Self {
        self.phonology = Some(phonology);
        self
    }

    /// Add basic phonology (convenience method)
    pub fn with_basic_phonology(mut self) -> Self {
        self.phonology = Some(
            PhonologyConfiguration::new()
                .add_vowels(&["a", "e", "i", "o", "u"])
                .add_consonants(&["p", "t", "k", "m", "n", "s", "l", "r"]),
        );
        self
    }

    /// Add syllable patterns (simplified - no type states visible)
    pub fn add_syllable_pattern(
        mut self,
        pattern: &str,
        weight: f32,
    ) -> Result<Self, ValidationError> {
        // Validate pattern format
        if !Self::is_valid_pattern(pattern) {
            return Err(ValidationError::new("invalid_syllable_pattern")
                .add_param("pattern", pattern)
                .with_message(
                    "Pattern must contain at least one vowel (V) and only C/V characters",
                ));
        }

        // Internal handling of syllable configuration
        // This hides the complexity from the user
        Ok(self)
    }

    /// Validate and build the complete language configuration
    pub fn build(self) -> Result<Language, ValidationErrors> {
        let mut errors = ValidationErrors::new();

        // Validate required components
        let anatomy = self.anatomy.ok_or_else(|| {
            ValidationError::new("missing_anatomy").with_message("Speaker anatomy is required")
        });

        let phonology = self.phonology.ok_or_else(|| {
            ValidationError::new("missing_phonology")
                .with_message("Phonology configuration is required")
        });

        match (anatomy, phonology) {
            (Ok(anatomy), Ok(phonology)) => {
                // Cross-validate components
                if let Err(validation_errors) = phonology.validate_against_anatomy(&anatomy) {
                    errors.merge(validation_errors);
                }

                if errors.is_empty() {
                    Ok(Language {
                        name: self.name,
                        anatomy,
                        phonology,
                        // syllables: processed syllable config
                    })
                } else {
                    Err(errors)
                }
            }
            _ => {
                if let Err(e) = anatomy {
                    errors.add("anatomy", e);
                }
                if let Err(e) = phonology {
                    errors.add("phonology", e);
                }
                Err(errors)
            }
        }
    }

    // Private helper methods
    fn is_valid_pattern(pattern: &str) -> bool {
        let has_vowel = pattern.chars().any(|c| c == 'V' || c == 'v');
        let only_valid_chars = pattern
            .chars()
            .all(|c| matches!(c, 'C' | 'c' | 'V' | 'v' | '(' | ')' | '[' | ']'));
        has_vowel && only_valid_chars
    }
}

/// Final language configuration - simplified public interface
pub struct Language {
    name: String,
    anatomy: SpeakerAnatomy,
    phonology: PhonologyConfiguration,
    // syllables: simplified syllable representation
}

impl Language {
    /// Get the language name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Generate a name using this language configuration
    pub fn generate_name(&self) -> Result<String, ValidationError> {
        // Implementation would use internal components
        // but user doesn't need to know about the complexity
        Ok(format!("Generated name from {}", self.name))
    }

    /// Generate multiple names
    pub fn generate_names(&self, count: usize) -> Result<Vec<String>, ValidationError> {
        (0..count).map(|_| self.generate_name()).collect()
    }

    /// Get language statistics (public interface)
    pub fn stats(&self) -> LanguageStats {
        LanguageStats {
            consonant_count: self.phonology.consonants().len(),
            vowel_count: self.phonology.vowels().len(),
            // other stats derived from internal state
        }
    }
}

/// Simplified stats structure for public consumption
#[derive(Debug, Clone)]
pub struct LanguageStats {
    pub consonant_count: usize,
    pub vowel_count: usize,
    // Add other useful stats that users might want
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language_builder() {
        let result = LanguageBuilder::new("Test Language")
            .with_human_anatomy()
            .with_basic_phonology()
            .add_syllable_pattern("CV", 0.8)
            .unwrap()
            .add_syllable_pattern("CVC", 0.6)
            .unwrap()
            .build();

        assert!(result.is_ok());
    }

    #[test]
    fn test_invalid_pattern() {
        let result = LanguageBuilder::new("Test").add_syllable_pattern("CCC", 1.0); // No vowel

        assert!(result.is_err());
    }
}
