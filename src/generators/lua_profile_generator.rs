//! Lua profile-based name generator
//!
//! This module implements a name generator that works with Lua language profiles
//! to create phonetically-grounded names with dynamic harmony rules.

use crate::language_profile::lua_profile::{LuaLanguageProfile, LuaSyllable, LuaPhoneme, LuaPhonemeCluster, LuaProfileError};
use rand::Rng;
use rand::distributions::{Distribution, WeightedIndex};

/// Lua profile-based name generator
pub struct LuaProfileGenerator {
    profile: LuaLanguageProfile,
}

impl LuaProfileGenerator {
    /// Create a new generator with a Lua language profile
    pub fn new(profile: LuaLanguageProfile) -> Self {
        Self { profile }
    }
    
    /// Generate a name using the Lua language profile
    pub fn generate(&self, rng: &mut impl Rng) -> Result<String, LuaProfileError> {
        // 1. Choose word length (for now, use default range)
        let syllable_count = rng.gen_range(1..=3);
        
        // 2. Generate syllables
        let mut syllables = Vec::new();
        for i in 0..syllable_count {
            let position = if i == 0 { "initial" } else if i == syllable_count - 1 { "final" } else { "medial" };
            let syllable = self.generate_syllable(rng, position)?;
            syllables.push(syllable);
        }
        
        // 3. Apply harmony rules
        self.profile.apply_harmony_rules(&mut syllables)?;
        
        // 4. Convert to graphemes
        let graphemes = self.syllables_to_graphemes(&syllables)?;
        
        Ok(graphemes.join(""))
    }
    
    /// Generate a single syllable
    fn generate_syllable(&self, rng: &mut impl Rng, position: &str) -> Result<LuaSyllable, LuaProfileError> {
        let phonemes = self.profile.get_phonemes()?;
        
        // For now, generate a simple CV syllable
        // In a full implementation, this would use the syllable patterns from the profile
        let consonants: Vec<_> = phonemes.iter().filter(|p| p.phoneme_type == "Consonant").collect();
        let vowels: Vec<_> = phonemes.iter().filter(|p| p.phoneme_type == "Vowel").collect();
        
        let mut syllable = LuaSyllable {
            onset: Vec::new(),
            nucleus: Vec::new(),
            coda: Vec::new(),
            stressed: position == "initial", // Simple stress rule
            position: position.to_string(),
        };
        
        // Generate onset (consonant)
        if !consonants.is_empty() && rng.gen_bool(0.8) {
            let consonant = consonants[rng.gen_range(0..consonants.len())];
            syllable.onset.push(consonant.ipa.clone());
        }
        
        // Generate nucleus (vowel) - required
        if !vowels.is_empty() {
            let vowel = vowels[rng.gen_range(0..vowels.len())];
            syllable.nucleus.push(vowel.ipa.clone());
        }
        
        // Generate coda (consonant) - optional
        if !consonants.is_empty() && rng.gen_bool(0.3) {
            let consonant = consonants[rng.gen_range(0..consonants.len())];
            syllable.coda.push(consonant.ipa.clone());
        }
        
        Ok(syllable)
    }
    
    /// Convert syllables to graphemes
    fn syllables_to_graphemes(&self, syllables: &[LuaSyllable]) -> Result<Vec<String>, LuaProfileError> {
        let phonemes = self.profile.get_phonemes()?;
        
        // Create IPA to grapheme mapping
        let mut ipa_to_grapheme = std::collections::HashMap::new();
        for phoneme in phonemes {
            ipa_to_grapheme.insert(phoneme.ipa.clone(), phoneme.grapheme.clone());
        }
        
        let mut graphemes = Vec::new();
        
        for syllable in syllables {
            let mut syllable_graphemes = Vec::new();
            
            // Convert onset
            for ipa in &syllable.onset {
                if let Some(grapheme) = ipa_to_grapheme.get(ipa) {
                    syllable_graphemes.push(grapheme.clone());
                }
            }
            
            // Convert nucleus
            for ipa in &syllable.nucleus {
                if let Some(grapheme) = ipa_to_grapheme.get(ipa) {
                    syllable_graphemes.push(grapheme.clone());
                }
            }
            
            // Convert coda
            for ipa in &syllable.coda {
                if let Some(grapheme) = ipa_to_grapheme.get(ipa) {
                    syllable_graphemes.push(grapheme.clone());
                }
            }
            
            graphemes.push(syllable_graphemes.join(""));
        }
        
        Ok(graphemes)
    }
    
    /// Get reference to the profile
    pub fn profile(&self) -> &LuaLanguageProfile {
        &self.profile
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::language_profile::lua_loader::LuaProfileLoader;
    
    #[test]
    fn test_lua_profile_generator_creation() {
        let loader = LuaProfileLoader::new().unwrap();
        let profile = loader.create_test_profile().unwrap();
        let generator = LuaProfileGenerator::new(profile);
        
        assert_eq!(generator.profile().get_name().unwrap(), "Test Language");
    }
    
    #[test]
    fn test_lua_profile_name_generation() {
        let loader = LuaProfileLoader::new().unwrap();
        let profile = loader.create_test_profile().unwrap();
        let generator = LuaProfileGenerator::new(profile);
        
        let mut rng = rand::thread_rng();
        let name = generator.generate(&mut rng).unwrap();
        
        // Name should not be empty
        assert!(!name.is_empty());
        
        // Name should only contain valid characters
        let valid_chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
        for c in name.chars() {
            assert!(valid_chars.contains(c), "Invalid character in name: {}", c);
        }
    }
    
    #[test]
    fn test_multiple_name_generation() {
        let loader = LuaProfileLoader::new().unwrap();
        let profile = loader.create_test_profile().unwrap();
        let generator = LuaProfileGenerator::new(profile);
        
        let mut rng = rand::thread_rng();
        let mut names = Vec::new();
        
        for _ in 0..10 {
            let name = generator.generate(&mut rng).unwrap();
            names.push(name);
        }
        
        // All names should be different (with high probability)
        assert_eq!(names.len(), 10);
        
        // Names should be reasonable length
        for name in &names {
            assert!(name.len() >= 1);
            assert!(name.len() <= 20);
        }
    }
    
    #[test]
    fn test_syllable_generation() {
        let loader = LuaProfileLoader::new().unwrap();
        let profile = loader.create_test_profile().unwrap();
        let generator = LuaProfileGenerator::new(profile);
        
        let mut rng = rand::thread_rng();
        let syllable = generator.generate_syllable(&mut rng, "initial").unwrap();
        
        // Syllable should have position set
        assert_eq!(syllable.position, "initial");
        
        // Syllable should have at least a nucleus
        assert!(!syllable.nucleus.is_empty());
    }
    
    #[test]
    fn test_harmony_rules_application() {
        let loader = LuaProfileLoader::new().unwrap();
        let profile = loader.create_test_profile().unwrap();
        
        // Create test syllables
        let mut syllables = vec![
            LuaSyllable {
                onset: vec!["[p]".to_string()],
                nucleus: vec!["[e]".to_string()],  // Front vowel
                coda: vec![],
                stressed: true,
                position: "initial".to_string(),
            },
            LuaSyllable {
                onset: vec!["[t]".to_string()],
                nucleus: vec!["[a]".to_string()],  // Back vowel - should change
                coda: vec![],
                stressed: false,
                position: "final".to_string(),
            },
        ];
        
        // Apply harmony rules
        assert!(profile.apply_harmony_rules(&mut syllables).is_ok());
        
        // Check that vowel harmony was applied
        assert_eq!(syllables[1].nucleus[0], "[e]");  // Should have changed from [a] to [e]
    }
}