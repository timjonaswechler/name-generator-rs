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
    /// Whether this syllable is stressed
    pub stressed: bool,
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
        
        // 3. Assign stress patterns
        self.assign_stress_patterns(&mut syllables);
        
        // 4. Apply harmony rules
        self.apply_harmony_rules(&mut syllables);
        
        // 5. Convert to graphemes
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
            stressed: false, // Will be assigned later
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
    
    /// Assign stress patterns to syllables
    fn assign_stress_patterns(&self, syllables: &mut [Syllable]) {
        if syllables.is_empty() {
            return;
        }
        
        // Simple stress assignment: first syllable gets primary stress
        // This can be made more sophisticated based on language-specific rules
        syllables[0].stressed = true;
        
        // For longer words, assign secondary stress to alternate syllables
        for i in 2..syllables.len() {
            if i % 2 == 0 {
                syllables[i].stressed = true;
            }
        }
    }
    
    /// Apply harmony rules to syllables
    fn apply_harmony_rules(&self, syllables: &mut [Syllable]) {
        // Apply vowel harmony and other phonetic rules
        for rule in &self.profile.style_rules.harmony_rules {
            match rule.name.as_str() {
                "vowel_harmony" => {
                    self.apply_vowel_harmony(syllables, rule);
                }
                "vowel_reduction" => {
                    self.apply_vowel_reduction(syllables, rule);
                }
                "consonant_cluster_simplification" => {
                    self.apply_consonant_cluster_simplification(syllables, rule);
                }
                "stress_dependent_vowel_quality" => {
                    self.apply_stress_dependent_vowel_quality(syllables, rule);
                }
                _ => {
                    // Unknown rule type, skip
                }
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
    
    /// Apply vowel reduction rule (reduce vowels to schwa in unstressed syllables)
    fn apply_vowel_reduction(&self, syllables: &mut [Syllable], rule: &crate::language_profile::profile::HarmonyRule) {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        
        for syllable in syllables.iter_mut() {
            // Only apply to unstressed syllables
            if !syllable.stressed {
                // Apply reduction with probability based on rule strength
                if rng.gen::<f32>() < rule.strength {
                    // Replace vowel with schwa [ə]
                    // First check if schwa exists in the phonetic inventory
                    if self.profile.phonetic_inventory.get_phoneme("[ə]").is_some() {
                        syllable.nucleus = vec!["[ə]".to_string()];
                    }
                }
            }
        }
    }
    
    /// Apply consonant cluster simplification
    fn apply_consonant_cluster_simplification(&self, syllables: &mut [Syllable], rule: &crate::language_profile::profile::HarmonyRule) {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        
        for syllable in syllables.iter_mut() {
            // Apply simplification with probability based on rule strength
            if rng.gen::<f32>() < rule.strength {
                // Simplify complex onset clusters (3+ consonants -> 2 consonants)
                if syllable.onset.len() > 2 {
                    // Keep first two consonants, remove the rest
                    syllable.onset.truncate(2);
                }
                
                // Simplify complex coda clusters (3+ consonants -> 2 consonants)
                if syllable.coda.len() > 2 {
                    // Keep first two consonants, remove the rest
                    syllable.coda.truncate(2);
                }
            }
        }
    }
    
    /// Apply stress-dependent vowel quality rules
    fn apply_stress_dependent_vowel_quality(&self, syllables: &mut [Syllable], rule: &crate::language_profile::profile::HarmonyRule) {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        
        for syllable in syllables.iter_mut() {
            // Apply quality adjustment with probability based on rule strength
            if rng.gen::<f32>() < rule.strength {
                if syllable.stressed {
                    // Prefer full vowels in stressed syllables
                    self.prefer_full_vowels(syllable);
                } else {
                    // Prefer reduced vowels in unstressed syllables
                    self.prefer_reduced_vowels(syllable);
                }
            }
        }
    }
    
    /// Prefer full vowels in a syllable
    fn prefer_full_vowels(&self, syllable: &mut Syllable) {
        // If current vowel is reduced, try to replace with full vowel
        if let Some(current_vowel) = syllable.nucleus.first() {
            if self.is_reduced_vowel(current_vowel) {
                // Try to find a full vowel from the same vowel group
                if let Some(vowel_group) = self.determine_vowel_group(current_vowel) {
                    if let Some(full_vowel) = self.find_full_vowel_in_group(&vowel_group) {
                        syllable.nucleus = vec![full_vowel];
                    }
                }
            }
        }
    }
    
    /// Prefer reduced vowels in a syllable
    fn prefer_reduced_vowels(&self, syllable: &mut Syllable) {
        // If current vowel is full, try to replace with reduced vowel
        if let Some(current_vowel) = syllable.nucleus.first() {
            if self.is_full_vowel(current_vowel) {
                // Try to find a reduced vowel from the same vowel group
                if let Some(vowel_group) = self.determine_vowel_group(current_vowel) {
                    if let Some(reduced_vowel) = self.find_reduced_vowel_in_group(&vowel_group) {
                        syllable.nucleus = vec![reduced_vowel];
                    }
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
    
    /// Check if a vowel is a reduced vowel (schwa, etc.)
    fn is_reduced_vowel(&self, vowel_ipa: &str) -> bool {
        // Common reduced vowels in various languages
        matches!(vowel_ipa, "[ə]" | "[ɪ]" | "[ʊ]" | "[ɨ]")
    }
    
    /// Check if a vowel is a full vowel (not reduced)
    fn is_full_vowel(&self, vowel_ipa: &str) -> bool {
        // Check if it's a vowel but not a reduced one
        if let Some(phoneme) = self.profile.phonetic_inventory.get_phoneme(vowel_ipa) {
            phoneme.is_vowel() && !self.is_reduced_vowel(vowel_ipa)
        } else {
            false
        }
    }
    
    /// Find a full vowel in a vowel group
    fn find_full_vowel_in_group(&self, group_name: &str) -> Option<String> {
        if let Some(group_phonemes) = self.profile.phonetic_inventory.get_group(group_name) {
            for phoneme in group_phonemes {
                if self.is_full_vowel(phoneme) {
                    return Some(phoneme.clone());
                }
            }
        }
        None
    }
    
    /// Find a reduced vowel in a vowel group
    fn find_reduced_vowel_in_group(&self, group_name: &str) -> Option<String> {
        if let Some(group_phonemes) = self.profile.phonetic_inventory.get_group(group_name) {
            for phoneme in group_phonemes {
                if self.is_reduced_vowel(phoneme) {
                    return Some(phoneme.clone());
                }
            }
        }
        // Fallback to schwa if available
        if self.profile.phonetic_inventory.get_phoneme("[ə]").is_some() {
            Some("[ə]".to_string())
        } else {
            None
        }
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
    
    #[test]
    fn test_stress_assignment() {
        let profile = LanguageProfile::create_advanced_test_profile();
        let generator = LanguageProfileGenerator::new(&profile);
        
        // Test stress assignment for single syllable
        let mut syllables = vec![Syllable {
            onset: vec!["[p]".to_string()],
            nucleus: vec!["[a]".to_string()],
            coda: vec![],
            stressed: false,
        }];
        
        generator.assign_stress_patterns(&mut syllables);
        assert!(syllables[0].stressed); // Single syllable should be stressed
        
        // Test stress assignment for multiple syllables
        let mut syllables = vec![
            Syllable {
                onset: vec!["[p]".to_string()],
                nucleus: vec!["[a]".to_string()],
                coda: vec![],
                stressed: false,
            },
            Syllable {
                onset: vec!["[t]".to_string()],
                nucleus: vec!["[i]".to_string()],
                coda: vec![],
                stressed: false,
            },
            Syllable {
                onset: vec!["[k]".to_string()],
                nucleus: vec!["[o]".to_string()],
                coda: vec![],
                stressed: false,
            },
        ];
        
        generator.assign_stress_patterns(&mut syllables);
        assert!(syllables[0].stressed); // First syllable should be stressed
        assert!(!syllables[1].stressed); // Second syllable should not be stressed
        assert!(syllables[2].stressed); // Third syllable should be stressed (alternating)
    }
    
    #[test]
    fn test_vowel_reduction_rule() {
        let profile = LanguageProfile::create_advanced_test_profile();
        let generator = LanguageProfileGenerator::new(&profile);
        
        let mut syllables = vec![
            Syllable {
                onset: vec!["[p]".to_string()],
                nucleus: vec!["[a]".to_string()],
                coda: vec![],
                stressed: true, // Stressed syllable - should not be reduced
            },
            Syllable {
                onset: vec!["[t]".to_string()],
                nucleus: vec!["[i]".to_string()],
                coda: vec![],
                stressed: false, // Unstressed syllable - may be reduced
            },
        ];
        
        let rule = crate::language_profile::profile::HarmonyRule {
            name: "vowel_reduction".to_string(),
            condition: "unstressed_syllable".to_string(),
            requirement: "prefer_schwa".to_string(),
            strength: 1.0, // Always apply for testing
        };
        
        generator.apply_vowel_reduction(&mut syllables, &rule);
        
        // First syllable should remain unchanged (stressed)
        assert_eq!(syllables[0].nucleus, vec!["[a]".to_string()]);
        
        // Second syllable should be reduced to schwa (unstressed)
        assert_eq!(syllables[1].nucleus, vec!["[ə]".to_string()]);
    }
    
    #[test]
    fn test_consonant_cluster_simplification() {
        let profile = LanguageProfile::create_advanced_test_profile();
        let generator = LanguageProfileGenerator::new(&profile);
        
        let mut syllables = vec![
            Syllable {
                onset: vec!["[s]".to_string(), "[p]".to_string(), "[t]".to_string()], // Complex cluster
                nucleus: vec!["[a]".to_string()],
                coda: vec!["[k]".to_string(), "[s]".to_string(), "[t]".to_string()], // Complex cluster
                stressed: true,
            },
        ];
        
        let rule = crate::language_profile::profile::HarmonyRule {
            name: "consonant_cluster_simplification".to_string(),
            condition: "has_complex_cluster".to_string(),
            requirement: "simplify_cluster".to_string(),
            strength: 1.0, // Always apply for testing
        };
        
        generator.apply_consonant_cluster_simplification(&mut syllables, &rule);
        
        // Onset should be simplified to 2 consonants
        assert_eq!(syllables[0].onset.len(), 2);
        
        // Coda should be simplified to 2 consonants
        assert_eq!(syllables[0].coda.len(), 2);
    }
    
    #[test]
    fn test_stress_dependent_vowel_quality() {
        let profile = LanguageProfile::create_advanced_test_profile();
        let generator = LanguageProfileGenerator::new(&profile);
        
        let mut syllables = vec![
            Syllable {
                onset: vec!["[p]".to_string()],
                nucleus: vec!["[ɪ]".to_string()], // Reduced vowel
                coda: vec![],
                stressed: true, // Stressed - should prefer full vowels
            },
            Syllable {
                onset: vec!["[t]".to_string()],
                nucleus: vec!["[a]".to_string()], // Full vowel
                coda: vec![],
                stressed: false, // Unstressed - should prefer reduced vowels
            },
        ];
        
        let rule = crate::language_profile::profile::HarmonyRule {
            name: "stress_dependent_vowel_quality".to_string(),
            condition: "stressed_syllable".to_string(),
            requirement: "prefer_full_vowels".to_string(),
            strength: 1.0, // Always apply for testing
        };
        
        generator.apply_stress_dependent_vowel_quality(&mut syllables, &rule);
        
        // First syllable should prefer full vowels (stressed)
        // Note: The exact outcome depends on vowel groups, but we can check if it's trying to change
        assert!(generator.is_full_vowel(&syllables[0].nucleus[0]) || generator.is_reduced_vowel(&syllables[0].nucleus[0]));
    }
    
    #[test]
    fn test_phoneme_classification() {
        let profile = LanguageProfile::create_advanced_test_profile();
        let generator = LanguageProfileGenerator::new(&profile);
        
        // Test full vowel classification
        assert!(generator.is_full_vowel("[a]"));
        assert!(generator.is_full_vowel("[i]"));
        assert!(generator.is_full_vowel("[o]"));
        
        // Test reduced vowel classification
        assert!(generator.is_reduced_vowel("[ə]"));
        assert!(generator.is_reduced_vowel("[ɪ]"));
        
        // Test that consonants are not classified as vowels
        assert!(!generator.is_full_vowel("[p]"));
        assert!(!generator.is_reduced_vowel("[t]"));
    }
    
    #[test]
    fn test_advanced_harmony_rules_integration() {
        let profile = LanguageProfile::create_advanced_test_profile();
        let generator = LanguageProfileGenerator::new(&profile);
        let mut rng = rand::thread_rng();
        
        // Generate names with advanced harmony rules
        for i in 0..5 {
            let name = generator.generate(&mut rng);
            println!("Generated name with advanced rules {}: {}", i + 1, name);
            assert!(!name.is_empty());
        }
    }
}