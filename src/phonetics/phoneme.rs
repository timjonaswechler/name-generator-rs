//! Phoneme representation and utilities
//!
//! This module provides the core phoneme structure and utilities for working with
//! phonetic representations in the LanguageProfile system.

use serde::{Deserialize, Serialize};

/// Represents a phoneme with IPA notation and grapheme representation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Phoneme {
    /// IPA representation like "[ʃ]", "[aɪ]", "[p]"
    pub ipa: String,
    /// Type of phoneme (consonant or vowel)
    pub phoneme_type: PhonemeType,
    /// Standard grapheme representation like "sch", "ei", "p"
    pub grapheme: String,
    /// Frequency weight for this phoneme (0.0 - 1.0)
    pub frequency: f32,
}

/// Type of phoneme - consonant or vowel
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PhonemeType {
    /// Consonant phoneme
    Consonant,
    /// Vowel phoneme
    Vowel,
}

impl Phoneme {
    /// Create a new phoneme
    pub fn new(ipa: &str, phoneme_type: PhonemeType, grapheme: &str, frequency: f32) -> Self {
        Self {
            ipa: ipa.to_string(),
            phoneme_type,
            grapheme: grapheme.to_string(),
            frequency,
        }
    }
    
    /// Check if this phoneme is a vowel
    pub fn is_vowel(&self) -> bool {
        matches!(self.phoneme_type, PhonemeType::Vowel)
    }
    
    /// Check if this phoneme is a consonant
    pub fn is_consonant(&self) -> bool {
        matches!(self.phoneme_type, PhonemeType::Consonant)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_phoneme_creation() {
        let phoneme = Phoneme::new("[p]", PhonemeType::Consonant, "p", 0.8);
        assert_eq!(phoneme.ipa, "[p]");
        assert_eq!(phoneme.grapheme, "p");
        assert!(phoneme.is_consonant());
        assert!(!phoneme.is_vowel());
    }
    
    #[test]
    fn test_phoneme_vowel() {
        let phoneme = Phoneme::new("[a]", PhonemeType::Vowel, "a", 0.9);
        assert_eq!(phoneme.ipa, "[a]");
        assert_eq!(phoneme.grapheme, "a");
        assert!(!phoneme.is_consonant());
        assert!(phoneme.is_vowel());
    }
}