//! Language Profile structure and implementation
//!
//! This module defines the main LanguageProfile data structure and supporting types
//! for phonetically-grounded name generation.

use crate::phonetics::phoneme::Phoneme;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Main language profile structure containing all language information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageProfile {
    /// Name of the language (e.g., "German", "English")
    pub name: String,
    /// Phonetic inventory of the language
    pub phonetic_inventory: PhoneticInventory,
    /// Syllable structure patterns
    pub syllable_structure: SyllableStructure,
    /// Word composition rules
    pub word_composition: WordComposition,
    /// Style and harmony rules
    pub style_rules: StyleRules,
}

/// Phonetic inventory containing phonemes and phoneme groups
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhoneticInventory {
    /// List of phonemes in the language
    pub phonemes: Vec<Phoneme>,
    /// Groups of phonemes (e.g., "front_vowels" -> ["[i]", "[e]"])
    pub phoneme_groups: HashMap<String, Vec<String>>,
}

/// Syllable structure definitions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyllableStructure {
    /// Available syllable patterns with frequencies
    pub patterns: Vec<SyllablePattern>,
    /// Onset cluster definitions
    pub onsets: Vec<PhonemeCluster>,
    /// Nucleus (vowel) definitions
    pub nuclei: Vec<PhonemeCluster>,
    /// Coda (final consonant) definitions
    pub codas: Vec<PhonemeCluster>,
}

/// Word composition rules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WordComposition {
    /// Minimum number of syllables per word
    pub min_syllables: usize,
    /// Maximum number of syllables per word
    pub max_syllables: usize,
    /// Available prefixes
    pub prefixes: Vec<Affix>,
    /// Available suffixes
    pub suffixes: Vec<Affix>,
    /// Forbidden phoneme transitions
    pub forbidden_transitions: Vec<TransitionRule>,
}

/// Style rules for phonetic harmony and frequency adjustments
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StyleRules {
    /// Harmony rules (e.g., vowel harmony)
    pub harmony_rules: Vec<HarmonyRule>,
    /// Frequency adjustments for specific phonemes
    pub frequency_adjustments: HashMap<String, f32>,
}

/// Syllable pattern with frequency weight
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyllablePattern {
    /// Pattern string (e.g., "CVC", "CV")
    pub pattern: String,
    /// Frequency weight (0.0 - 1.0)
    pub frequency: f32,
}

/// Cluster of phonemes with frequency
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhonemeCluster {
    /// IPA strings of phonemes in the cluster
    pub phonemes: Vec<String>,
    /// Frequency weight (0.0 - 1.0)
    pub frequency: f32,
}

/// Affix (prefix or suffix) with phonetic representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Affix {
    /// Grapheme representation
    pub grapheme: String,
    /// IPA phonemes
    pub phonemes: Vec<String>,
    /// Frequency weight (0.0 - 1.0)
    pub frequency: f32,
}

/// Transition rule for forbidden phoneme combinations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitionRule {
    /// Coda phonemes
    pub coda: Vec<String>,
    /// Onset phonemes
    pub onset: Vec<String>,
    /// Whether this transition is forbidden
    pub forbidden: bool,
}

/// Harmony rule for phonetic consistency
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HarmonyRule {
    /// Name of the rule
    pub name: String,
    /// Condition for applying the rule
    pub condition: String,
    /// Required action when condition is met
    pub requirement: Option<String>, // Made optional for backward compatibility
    /// Lua script for custom rule logic
    pub script: Option<String>,
    /// Strength of the rule (0.0 - 1.0)
    pub strength: f32,
}

impl PhoneticInventory {
    /// Create a new empty phonetic inventory
    pub fn new() -> Self {
        Self {
            phonemes: Vec::new(),
            phoneme_groups: HashMap::new(),
        }
    }
    
    /// Add a phoneme to the inventory
    pub fn add_phoneme(&mut self, phoneme: Phoneme) {
        self.phonemes.push(phoneme);
    }
    
    /// Get a phoneme by its IPA representation
    pub fn get_phoneme(&self, ipa: &str) -> Option<&Phoneme> {
        self.phonemes.iter().find(|p| p.ipa == ipa)
    }
    
    /// Get a phoneme group by name
    pub fn get_group(&self, group_name: &str) -> Option<&Vec<String>> {
        self.phoneme_groups.get(group_name)
    }
    
    /// Get all consonants from the inventory
    pub fn get_consonants(&self) -> Vec<&Phoneme> {
        self.phonemes.iter().filter(|p| p.is_consonant()).collect()
    }
    
    /// Get all vowels from the inventory
    pub fn get_vowels(&self) -> Vec<&Phoneme> {
        self.phonemes.iter().filter(|p| p.is_vowel()).collect()
    }
    
    /// Add a phoneme group
    pub fn add_phoneme_group(&mut self, name: String, phonemes: Vec<String>) {
        self.phoneme_groups.insert(name, phonemes);
    }
}

impl SyllableStructure {
    /// Create a new empty syllable structure
    pub fn new() -> Self {
        Self {
            patterns: Vec::new(),
            onsets: Vec::new(),
            nuclei: Vec::new(),
            codas: Vec::new(),
        }
    }
    
    /// Add a syllable pattern
    pub fn add_pattern(&mut self, pattern: &str, frequency: f32) {
        self.patterns.push(SyllablePattern {
            pattern: pattern.to_string(),
            frequency,
        });
    }
    
    /// Get a random syllable pattern based on frequency weights
    pub fn get_random_pattern(&self, rng: &mut impl rand::Rng) -> Option<&SyllablePattern> {
        if self.patterns.is_empty() {
            return None;
        }
        
        // Weighted random selection based on frequency
        let total_weight: f32 = self.patterns.iter().map(|p| p.frequency).sum();
        if total_weight <= 0.0 {
            return self.patterns.first();
        }
        
        let mut random_value = rng.gen::<f32>() * total_weight;
        
        for pattern in &self.patterns {
            random_value -= pattern.frequency;
            if random_value <= 0.0 {
                return Some(pattern);
            }
        }
        
        // Fallback to last pattern
        self.patterns.last()
    }
}

impl Default for PhoneticInventory {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for SyllableStructure {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for WordComposition {
    fn default() -> Self {
        Self {
            min_syllables: 1,
            max_syllables: 3,
            prefixes: Vec::new(),
            suffixes: Vec::new(),
            forbidden_transitions: Vec::new(),
        }
    }
}

impl Default for StyleRules {
    fn default() -> Self {
        Self {
            harmony_rules: Vec::new(),
            frequency_adjustments: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::phonetics::phoneme::{Phoneme, PhonemeType};
    use rand::thread_rng;
    
    #[test]
    fn test_phonetic_inventory_creation() {
        let mut inventory = PhoneticInventory::new();
        assert!(inventory.phonemes.is_empty());
        assert!(inventory.phoneme_groups.is_empty());
        
        let phoneme = Phoneme::new("[p]", PhonemeType::Consonant, "p", 0.8);
        inventory.add_phoneme(phoneme);
        assert_eq!(inventory.phonemes.len(), 1);
        
        let found = inventory.get_phoneme("[p]");
        assert!(found.is_some());
        assert_eq!(found.unwrap().grapheme, "p");
    }
    
    #[test]
    fn test_syllable_structure_pattern_selection() {
        let mut structure = SyllableStructure::new();
        structure.add_pattern("CV", 0.5);
        structure.add_pattern("CVC", 0.3);
        structure.add_pattern("V", 0.2);
        
        let mut rng = thread_rng();
        for _ in 0..10 {
            let pattern = structure.get_random_pattern(&mut rng);
            assert!(pattern.is_some());
            let pattern = pattern.unwrap();
            assert!(["CV", "CVC", "V"].contains(&pattern.pattern.as_str()));
        }
    }
    
    #[test]
    fn test_consonant_vowel_filtering() {
        let mut inventory = PhoneticInventory::new();
        inventory.add_phoneme(Phoneme::new("[p]", PhonemeType::Consonant, "p", 0.8));
        inventory.add_phoneme(Phoneme::new("[a]", PhonemeType::Vowel, "a", 0.9));
        inventory.add_phoneme(Phoneme::new("[t]", PhonemeType::Consonant, "t", 0.7));
        
        let consonants = inventory.get_consonants();
        let vowels = inventory.get_vowels();
        
        assert_eq!(consonants.len(), 2);
        assert_eq!(vowels.len(), 1);
        assert!(consonants.iter().any(|c| c.grapheme == "p"));
        assert!(consonants.iter().any(|c| c.grapheme == "t"));
        assert!(vowels.iter().any(|v| v.grapheme == "a"));
    }
}