//! Profile-based name generator
//!
//! This module implements the main generator that works with language profiles
//! to create phonetically-grounded names.

use crate::language_profile::profile::LanguageProfile;
use rand::Rng;
use rand::distributions::{Distribution, WeightedIndex};

/// Main generator that works with language profiles
pub struct LanguageProfileGenerator<'a> {
    profile: &'a LanguageProfile,
}

/// Represents a generated syllable
#[derive(Debug, Clone)]
pub struct Syllable {
    /// IPA strings for onset consonants
    pub onset: Vec<String>,
    /// IPA strings for nucleus vowels
    pub nucleus: Vec<String>,
    /// IPA strings for coda consonants
    pub coda: Vec<String>,
}

impl<'a> LanguageProfileGenerator<'a> {
    /// Create a new generator with a language profile
    pub fn new(profile: &'a LanguageProfile) -> Self {
        Self { profile }
    }
    
    /// Generate a name using the language profile
    pub fn generate(&self, rng: &mut impl Rng) -> String {
        // 1. Choose word length
        let syllable_count = rng.gen_range(
            self.profile.word_composition.min_syllables..=
            self.profile.word_composition.max_syllables
        );
        
        // 2. Generate syllables
        let mut syllables = Vec::new();
        for _i in 0..syllable_count {
            let syllable = self.generate_syllable(rng, &syllables);
            syllables.push(syllable);
        }
        
        // 3. Apply harmony rules
        self.apply_harmony_rules(&mut syllables);
        
        // 4. Convert to graphemes
        self.convert_to_graphemes(&syllables)
    }
    
    /// Generate a single syllable
    fn generate_syllable(&self, rng: &mut impl Rng, _context: &[Syllable]) -> Syllable {
        // Choose a syllable pattern
        let pattern = self.profile.syllable_structure
            .get_random_pattern(rng)
            .expect("No syllable patterns defined");
        
        let mut syllable = Syllable {
            onset: Vec::new(),
            nucleus: Vec::new(),
            coda: Vec::new(),
        };
        
        // Parse pattern and fill components
        for component in pattern.pattern.chars() {
            match component {
                'C' => {
                    // Add consonant to appropriate position
                    if syllable.nucleus.is_empty() {
                        // This is onset
                        if let Some(onset) = self.choose_onset(rng) {
                            syllable.onset = onset.phonemes.clone();
                        }
                    } else {
                        // This is coda
                        if let Some(coda) = self.choose_coda(rng) {
                            syllable.coda = coda.phonemes.clone();
                        }
                    }
                }
                'V' => {
                    // Add vowel to nucleus
                    if let Some(nucleus) = self.choose_nucleus(rng) {
                        syllable.nucleus = nucleus.phonemes.clone();
                    }
                }
                _ => {
                    // Ignore other characters
                }
            }
        }
        
        syllable
    }
    
    /// Choose an onset cluster
    fn choose_onset(&self, rng: &mut impl Rng) -> Option<&crate::language_profile::profile::PhonemeCluster> {
        self.weighted_choice(&self.profile.syllable_structure.onsets, rng)
    }
    
    /// Choose a nucleus cluster 
    fn choose_nucleus(&self, rng: &mut impl Rng) -> Option<&crate::language_profile::profile::PhonemeCluster> {
        self.weighted_choice(&self.profile.syllable_structure.nuclei, rng)
    }
    
    /// Choose a coda cluster
    fn choose_coda(&self, rng: &mut impl Rng) -> Option<&crate::language_profile::profile::PhonemeCluster> {
        self.weighted_choice(&self.profile.syllable_structure.codas, rng)
    }
    
    /// Perform weighted choice of phoneme clusters
    fn weighted_choice<'b>(&self, clusters: &'b [crate::language_profile::profile::PhonemeCluster], rng: &mut impl Rng) -> Option<&'b crate::language_profile::profile::PhonemeCluster> {
        if clusters.is_empty() {
            return None;
        }
        
        let weights: Vec<f32> = clusters.iter().map(|c| c.frequency).collect();
        let total_weight: f32 = weights.iter().sum();
        
        if total_weight <= 0.0 {
            // Fall back to uniform selection
            let index = rng.gen_range(0..clusters.len());
            return Some(&clusters[index]);
        }
        
        match WeightedIndex::new(&weights) {
            Ok(dist) => {
                let index = dist.sample(rng);
                Some(&clusters[index])
            }
            Err(_) => {
                // Fallback to uniform selection if weighted selection fails
                let index = rng.gen_range(0..clusters.len());
                Some(&clusters[index])
            }
        }
    }
    
    /// Apply harmony rules to syllables
    fn apply_harmony_rules(&self, syllables: &mut [Syllable]) {
        // Apply vowel harmony and other phonetic rules
        for rule in &self.profile.style_rules.harmony_rules {
            // Implementation depends on specific rule format
            // This is a simplified example
            if rule.name == "vowel_harmony" {
                self.apply_vowel_harmony(syllables, rule);
            }
        }
    }
    
    /// Apply vowel harmony rule
    fn apply_vowel_harmony(&self, syllables: &mut [Syllable], _rule: &crate::language_profile::profile::HarmonyRule) {
        // Simple vowel harmony implementation
        if syllables.is_empty() {
            return;
        }
        
        // Get the vowel type from first syllable
        let first_vowel = syllables[0].nucleus.first();
        if let Some(first_vowel_ipa) = first_vowel {
            // Determine vowel group
            let vowel_group = self.determine_vowel_group(first_vowel_ipa);
            
            // Apply harmony to subsequent syllables
            for syllable in syllables.iter_mut().skip(1) {
                if let Some(replacement) = self.find_harmonizing_vowel(&syllable.nucleus, &vowel_group) {
                    syllable.nucleus = vec![replacement];
                }
            }
        }
    }
    
    /// Determine which vowel group a vowel belongs to
    fn determine_vowel_group(&self, vowel_ipa: &str) -> Option<String> {
        for (group_name, phonemes) in &self.profile.phonetic_inventory.phoneme_groups {
            if phonemes.contains(&vowel_ipa.to_string()) {
                return Some(group_name.clone());
            }
        }
        None
    }
    
    /// Find a harmonizing vowel for vowel harmony
    fn find_harmonizing_vowel(&self, _current_nucleus: &[String], target_group: &Option<String>) -> Option<String> {
        if let Some(group_name) = target_group {
            if let Some(group_phonemes) = self.profile.phonetic_inventory.get_group(group_name) {
                // Find a vowel from the same group
                for phoneme in group_phonemes {
                    if let Some(p) = self.profile.phonetic_inventory.get_phoneme(phoneme) {
                        if p.is_vowel() {
                            return Some(phoneme.clone());
                        }
                    }
                }
            }
        }
        None
    }
    
    /// Convert syllables to graphemes (written form)
    fn convert_to_graphemes(&self, syllables: &[Syllable]) -> String {
        let mut result = String::new();
        
        for syllable in syllables {
            // Convert onset
            for phoneme_ipa in &syllable.onset {
                if let Some(phoneme) = self.profile.phonetic_inventory.get_phoneme(phoneme_ipa) {
                    result.push_str(&phoneme.grapheme);
                }
            }
            
            // Convert nucleus
            for phoneme_ipa in &syllable.nucleus {
                if let Some(phoneme) = self.profile.phonetic_inventory.get_phoneme(phoneme_ipa) {
                    result.push_str(&phoneme.grapheme);
                }
            }
            
            // Convert coda
            for phoneme_ipa in &syllable.coda {
                if let Some(phoneme) = self.profile.phonetic_inventory.get_phoneme(phoneme_ipa) {
                    result.push_str(&phoneme.grapheme);
                }
            }
        }
        
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::language_profile::profile::LanguageProfile;
    use rand::thread_rng;
    
    #[test]
    fn test_profile_generator_creation() {
        let profile = LanguageProfile::create_simple_test_profile();
        let generator = LanguageProfileGenerator::new(&profile);
        assert!(!generator.profile.name.is_empty());
    }
    
    #[test]
    fn test_profile_generator_generation() {
        let profile = LanguageProfile::create_simple_test_profile();
        let generator = LanguageProfileGenerator::new(&profile);
        let mut rng = thread_rng();
        
        // Generate multiple names to test variety
        for i in 0..10 {
            let name = generator.generate(&mut rng);
            println!("Generated name {}: {}", i + 1, name);
            assert!(!name.is_empty());
            assert!(name.len() >= 1); // Should have reasonable length
        }
    }
    
    #[test]
    fn test_syllable_generation() {
        let profile = LanguageProfile::create_simple_test_profile();
        let generator = LanguageProfileGenerator::new(&profile);
        let mut rng = thread_rng();
        
        let syllables = Vec::new();
        let syllable = generator.generate_syllable(&mut rng, &syllables);
        
        // Should have at least a nucleus
        assert!(!syllable.nucleus.is_empty());
    }
    
    #[test]
    fn test_weighted_choice() {
        let profile = LanguageProfile::create_simple_test_profile();
        let generator = LanguageProfileGenerator::new(&profile);
        let mut rng = thread_rng();
        
        // Test with onsets
        let onset = generator.choose_onset(&mut rng);
        assert!(onset.is_some());
        
        // Test with nuclei
        let nucleus = generator.choose_nucleus(&mut rng);
        assert!(nucleus.is_some());
        
        // Test with codas
        let coda = generator.choose_coda(&mut rng);
        assert!(coda.is_some());
    }
}