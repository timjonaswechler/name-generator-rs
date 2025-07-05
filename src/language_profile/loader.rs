//! Language Profile loader and validation
//!
//! This module provides functionality to load and validate language profiles
//! from YAML files and built-in profiles.

use super::profile::LanguageProfile;
use std::fs;
use std::error::Error;
use std::path::Path;

impl LanguageProfile {
    /// Load a language profile from a file path
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn Error>> {
        let content = fs::read_to_string(path)?;
        let profile: LanguageProfile = serde_yaml::from_str(&content)?;
        
        // Validate the profile after loading
        profile.validate()?;
        
        Ok(profile)
    }
    
    /// Load a built-in language profile by name
    pub fn load_builtin(language: &str) -> Result<Self, Box<dyn Error>> {
        let path = format!("languages/{}.yml", language);
        Self::load_from_file(&path)
    }
    
    /// Save the language profile to a file
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn Error>> {
        let content = serde_yaml::to_string(self)?;
        fs::write(path, content)?;
        Ok(())
    }
    
    /// Validate that the language profile is consistent and complete
    pub fn validate(&self) -> Result<(), String> {
        // Check basic requirements
        if self.name.is_empty() {
            return Err("Language profile must have a name".to_string());
        }
        
        if self.phonetic_inventory.phonemes.is_empty() {
            return Err("Language profile must have at least one phoneme".to_string());
        }
        
        // Validate that all referenced phonemes exist in the inventory
        let available_phonemes: std::collections::HashSet<&str> = self.phonetic_inventory.phonemes
            .iter()
            .map(|p| p.ipa.as_str())
            .collect();
        
        // Check onsets
        for cluster in &self.syllable_structure.onsets {
            for phoneme_ipa in &cluster.phonemes {
                if !available_phonemes.contains(phoneme_ipa.as_str()) {
                    return Err(format!("Unknown phoneme in onset: {}", phoneme_ipa));
                }
            }
        }
        
        // Check nuclei
        for cluster in &self.syllable_structure.nuclei {
            for phoneme_ipa in &cluster.phonemes {
                if !available_phonemes.contains(phoneme_ipa.as_str()) {
                    return Err(format!("Unknown phoneme in nucleus: {}", phoneme_ipa));
                }
            }
        }
        
        // Check codas
        for cluster in &self.syllable_structure.codas {
            for phoneme_ipa in &cluster.phonemes {
                if !available_phonemes.contains(phoneme_ipa.as_str()) {
                    return Err(format!("Unknown phoneme in coda: {}", phoneme_ipa));
                }
            }
        }
        
        // Check phoneme groups
        for (group_name, phonemes) in &self.phonetic_inventory.phoneme_groups {
            for phoneme_ipa in phonemes {
                if !available_phonemes.contains(phoneme_ipa.as_str()) {
                    return Err(format!("Unknown phoneme in group '{}': {}", group_name, phoneme_ipa));
                }
            }
        }
        
        // Check word composition constraints
        if self.word_composition.min_syllables == 0 {
            return Err("Minimum syllables must be at least 1".to_string());
        }
        
        if self.word_composition.max_syllables < self.word_composition.min_syllables {
            return Err("Maximum syllables must be greater than or equal to minimum syllables".to_string());
        }
        
        // Check that we have at least one syllable pattern
        if self.syllable_structure.patterns.is_empty() {
            return Err("Language profile must have at least one syllable pattern".to_string());
        }
        
        // Check that we have at least one nucleus (vowel sound)
        if self.syllable_structure.nuclei.is_empty() {
            return Err("Language profile must have at least one nucleus (vowel sound)".to_string());
        }
        
        // Validate frequency values are in valid range
        for phoneme in &self.phonetic_inventory.phonemes {
            if phoneme.frequency < 0.0 || phoneme.frequency > 1.0 {
                return Err(format!("Phoneme frequency must be between 0.0 and 1.0: {}", phoneme.ipa));
            }
        }
        
        for pattern in &self.syllable_structure.patterns {
            if pattern.frequency < 0.0 || pattern.frequency > 1.0 {
                return Err(format!("Pattern frequency must be between 0.0 and 1.0: {}", pattern.pattern));
            }
        }
        
        Ok(())
    }
    
    /// Create a simple language profile for testing
    pub fn create_simple_test_profile() -> Self {
        use crate::phonetics::phoneme::{Phoneme, PhonemeType};
        use crate::language_profile::profile::*;
        
        let mut inventory = PhoneticInventory::new();
        
        // Add basic phonemes
        inventory.add_phoneme(Phoneme::new("[p]", PhonemeType::Consonant, "p", 0.8));
        inventory.add_phoneme(Phoneme::new("[t]", PhonemeType::Consonant, "t", 0.9));
        inventory.add_phoneme(Phoneme::new("[k]", PhonemeType::Consonant, "k", 0.7));
        inventory.add_phoneme(Phoneme::new("[a]", PhonemeType::Vowel, "a", 0.9));
        inventory.add_phoneme(Phoneme::new("[i]", PhonemeType::Vowel, "i", 0.8));
        inventory.add_phoneme(Phoneme::new("[o]", PhonemeType::Vowel, "o", 0.7));
        
        let mut structure = SyllableStructure::new();
        structure.add_pattern("CV", 0.6);
        structure.add_pattern("CVC", 0.4);
        
        // Add onsets
        structure.onsets.push(PhonemeCluster {
            phonemes: vec!["[p]".to_string()],
            frequency: 0.8,
        });
        structure.onsets.push(PhonemeCluster {
            phonemes: vec!["[t]".to_string()],
            frequency: 0.9,
        });
        structure.onsets.push(PhonemeCluster {
            phonemes: vec!["[k]".to_string()],
            frequency: 0.7,
        });
        
        // Add nuclei
        structure.nuclei.push(PhonemeCluster {
            phonemes: vec!["[a]".to_string()],
            frequency: 0.9,
        });
        structure.nuclei.push(PhonemeCluster {
            phonemes: vec!["[i]".to_string()],
            frequency: 0.8,
        });
        structure.nuclei.push(PhonemeCluster {
            phonemes: vec!["[o]".to_string()],
            frequency: 0.7,
        });
        
        // Add codas
        structure.codas.push(PhonemeCluster {
            phonemes: vec![], // Empty coda
            frequency: 0.6,
        });
        structure.codas.push(PhonemeCluster {
            phonemes: vec!["[t]".to_string()],
            frequency: 0.4,
        });
        
        LanguageProfile {
            name: "Test Language".to_string(),
            phonetic_inventory: inventory,
            syllable_structure: structure,
            word_composition: WordComposition::default(),
            style_rules: StyleRules::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;
    
    #[test]
    fn test_simple_profile_creation_and_validation() {
        let profile = LanguageProfile::create_simple_test_profile();
        assert_eq!(profile.name, "Test Language");
        assert_eq!(profile.phonetic_inventory.phonemes.len(), 6);
        assert_eq!(profile.syllable_structure.patterns.len(), 2);
        
        // Should validate successfully
        assert!(profile.validate().is_ok());
    }
    
    #[test]
    fn test_profile_save_and_load() {
        let profile = LanguageProfile::create_simple_test_profile();
        
        // Save to temporary file
        let temp_path = "/tmp/test_profile.yml";
        profile.save_to_file(temp_path).expect("Failed to save profile");
        
        // Load from file
        let loaded_profile = LanguageProfile::load_from_file(temp_path)
            .expect("Failed to load profile");
        
        assert_eq!(profile.name, loaded_profile.name);
        assert_eq!(profile.phonetic_inventory.phonemes.len(), loaded_profile.phonetic_inventory.phonemes.len());
        
        // Clean up
        let _ = fs::remove_file(temp_path);
    }
    
    #[test]
    fn test_profile_validation_errors() {
        let mut profile = LanguageProfile::create_simple_test_profile();
        
        // Test empty name
        profile.name = "".to_string();
        assert!(profile.validate().is_err());
        
        // Reset name
        profile.name = "Test".to_string();
        
        // Test invalid phoneme reference
        profile.syllable_structure.onsets.push(crate::language_profile::profile::PhonemeCluster {
            phonemes: vec!["[invalid]".to_string()],
            frequency: 0.5,
        });
        assert!(profile.validate().is_err());
    }
}