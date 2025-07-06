//! IPA (International Phonetic Alphabet) utilities
//!
//! This module provides utilities for working with IPA notation and phonetic
//! representations in the LanguageProfile system.

use crate::phonetics::phoneme::{Phoneme, PhonemeType};

/// Utility functions for IPA processing
pub struct IpaUtils;

impl IpaUtils {
    /// Check if a string is a valid IPA representation
    pub fn is_valid_ipa(ipa: &str) -> bool {
        // Basic validation - should start and end with brackets
        ipa.starts_with('[') && ipa.ends_with(']') && ipa.len() > 2
    }
    
    /// Extract the IPA content without brackets
    pub fn extract_ipa_content(ipa: &str) -> Option<&str> {
        if Self::is_valid_ipa(ipa) {
            Some(&ipa[1..ipa.len()-1])
        } else {
            None
        }
    }
    
    /// Create a phoneme from IPA notation with automatic type detection
    pub fn create_phoneme_from_ipa(ipa: &str, grapheme: &str, frequency: f32) -> Option<Phoneme> {
        if !Self::is_valid_ipa(ipa) {
            return None;
        }
        
        let phoneme_type = Self::detect_phoneme_type(ipa);
        Some(Phoneme::new(ipa, phoneme_type, grapheme, frequency))
    }
    
    /// Detect if an IPA notation represents a vowel or consonant
    pub fn detect_phoneme_type(ipa: &str) -> PhonemeType {
        let content = Self::extract_ipa_content(ipa).unwrap_or("");
        
        // Simple heuristic based on common IPA vowel symbols
        let vowel_symbols = [
            "a", "e", "i", "o", "u", "ɑ", "ɒ", "ɔ", "ɛ", "ɪ", "ʊ", "ʌ", "ə", "ɜ", "æ", "ɐ",
            "y", "ø", "œ", "ɵ", "ɤ", "ɯ", "ɨ", "ɪ", "ʏ", "ʉ", "ʊ", "ɘ", "ɞ", "ɶ"
        ];
        
        // Check if the content contains any vowel symbols
        for vowel in &vowel_symbols {
            if content.contains(vowel) {
                return PhonemeType::Vowel;
            }
        }
        
        // Check for diphthongs (containing vowel combinations)
        if content.len() > 1 {
            let chars: Vec<char> = content.chars().collect();
            for i in 0..chars.len() {
                let char_str = chars[i].to_string();
                if vowel_symbols.contains(&char_str.as_str()) {
                    return PhonemeType::Vowel;
                }
            }
        }
        
        // Default to consonant
        PhonemeType::Consonant
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_ipa_validation() {
        assert!(IpaUtils::is_valid_ipa("[p]"));
        assert!(IpaUtils::is_valid_ipa("[aɪ]"));
        assert!(!IpaUtils::is_valid_ipa("p"));
        assert!(!IpaUtils::is_valid_ipa("["));
        assert!(!IpaUtils::is_valid_ipa("[]"));
    }
    
    #[test]
    fn test_ipa_content_extraction() {
        assert_eq!(IpaUtils::extract_ipa_content("[p]"), Some("p"));
        assert_eq!(IpaUtils::extract_ipa_content("[aɪ]"), Some("aɪ"));
        assert_eq!(IpaUtils::extract_ipa_content("p"), None);
    }
    
    #[test]
    fn test_phoneme_type_detection() {
        assert_eq!(IpaUtils::detect_phoneme_type("[a]"), PhonemeType::Vowel);
        assert_eq!(IpaUtils::detect_phoneme_type("[p]"), PhonemeType::Consonant);
        assert_eq!(IpaUtils::detect_phoneme_type("[aɪ]"), PhonemeType::Vowel);
        assert_eq!(IpaUtils::detect_phoneme_type("[ʃ]"), PhonemeType::Consonant);
    }
    
    #[test]
    fn test_phoneme_creation_from_ipa() {
        let phoneme = IpaUtils::create_phoneme_from_ipa("[p]", "p", 0.8);
        assert!(phoneme.is_some());
        let phoneme = phoneme.unwrap();
        assert_eq!(phoneme.ipa, "[p]");
        assert_eq!(phoneme.grapheme, "p");
        assert!(phoneme.is_consonant());
        
        let invalid = IpaUtils::create_phoneme_from_ipa("p", "p", 0.8);
        assert!(invalid.is_none());
    }
}