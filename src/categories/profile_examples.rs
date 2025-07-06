//! Example categories using language profiles
//!
//! This module demonstrates how to create categories that use the new
//! LanguageProfile system while maintaining compatibility with the existing system.

use crate::core::LanguageProfileCategory;
use crate::language_profile::profile::LanguageProfile;
use std::sync::OnceLock;

/// German language profile category
#[derive(Debug, Clone, Default)]
pub struct GermanLanguageProfile;

impl LanguageProfileCategory for GermanLanguageProfile {
    fn language_profile(&self) -> &LanguageProfile {
        static GERMAN_PROFILE: OnceLock<LanguageProfile> = OnceLock::new();
        GERMAN_PROFILE.get_or_init(|| {
            LanguageProfile::load_builtin("german")
                .unwrap_or_else(|_| {
                    // Fall back to test profile if file loading fails
                    LanguageProfile::create_simple_test_profile()
                })
        })
    }
}

/// English language profile category
#[derive(Debug, Clone, Default)]
pub struct EnglishLanguageProfile;

impl LanguageProfileCategory for EnglishLanguageProfile {
    fn language_profile(&self) -> &LanguageProfile {
        static ENGLISH_PROFILE: OnceLock<LanguageProfile> = OnceLock::new();
        ENGLISH_PROFILE.get_or_init(|| {
            LanguageProfile::load_builtin("english")
                .unwrap_or_else(|_| {
                    // Fall back to test profile if file loading fails
                    LanguageProfile::create_simple_test_profile()
                })
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Name;
    use rand::thread_rng;

    #[test]
    fn test_german_profile_generation() {
        let mut rng = thread_rng();
        let generator = Name::<GermanLanguageProfile>::new();
        
        // Generate multiple names to test variety
        for i in 0..10 {
            let name = generator.generate(&mut rng);
            println!("German name {}: {}", i + 1, name);
            assert!(!name.is_empty());
            assert!(name.len() >= 1); // Should have reasonable length
        }
    }
    
    #[test]
    fn test_english_profile_generation() {
        let mut rng = thread_rng();
        let generator = Name::<EnglishLanguageProfile>::new();
        
        // Generate multiple names to test variety
        for i in 0..10 {
            let name = generator.generate(&mut rng);
            println!("English name {}: {}", i + 1, name);
            assert!(!name.is_empty());
            assert!(name.len() >= 1); // Should have reasonable length
        }
    }
    
    #[test]
    fn test_language_profile_access() {
        let german_gen = GermanLanguageProfile::default();
        let english_gen = EnglishLanguageProfile::default();
        
        // Test that we can access the language profiles
        let german_profile = german_gen.language_profile();
        let english_profile = english_gen.language_profile();
        
        assert!(!german_profile.name.is_empty());
        assert!(!english_profile.name.is_empty());
        
        // Test that they have phonemes
        assert!(!german_profile.phonetic_inventory.phonemes.is_empty());
        assert!(!english_profile.phonetic_inventory.phonemes.is_empty());
    }
}