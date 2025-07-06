//! Tests for enhanced language profiles
//! 
//! This module tests the enhanced German and English profiles with
//! their new morphological and harmony rules.

use name_generator::{
    language_profile::profile::LanguageProfile,
    generators::profile_generator::LanguageProfileGenerator,
    core::Name,
    categories::profile_examples::{GermanLanguageProfile, EnglishLanguageProfile},
};
use rand::thread_rng;

#[test]
fn test_enhanced_german_profile_loading() {
    let profile = LanguageProfile::load_from_file("languages/german.yml")
        .expect("Failed to load German profile");
    
    // Test enhanced features
    assert_eq!(profile.word_composition.prefixes.len(), 4);
    assert_eq!(profile.word_composition.suffixes.len(), 4);
    assert_eq!(profile.word_composition.forbidden_transitions.len(), 5);
    assert_eq!(profile.style_rules.harmony_rules.len(), 4);
    assert_eq!(profile.style_rules.frequency_adjustments.len(), 7);
    
    // Test specific prefixes
    let prefix_graphemes: Vec<&str> = profile.word_composition.prefixes.iter()
        .map(|p| p.grapheme.as_str()).collect();
    assert!(prefix_graphemes.contains(&"ge"));
    assert!(prefix_graphemes.contains(&"ver"));
    assert!(prefix_graphemes.contains(&"ent"));
    assert!(prefix_graphemes.contains(&"un"));
    
    // Test specific suffixes
    let suffix_graphemes: Vec<&str> = profile.word_composition.suffixes.iter()
        .map(|s| s.grapheme.as_str()).collect();
    assert!(suffix_graphemes.contains(&"er"));
    assert!(suffix_graphemes.contains(&"in"));
    assert!(suffix_graphemes.contains(&"chen"));
    assert!(suffix_graphemes.contains(&"lein"));
    
    // Test that profile validates
    assert!(profile.validate().is_ok());
}

#[test]
fn test_enhanced_english_profile_loading() {
    let profile = LanguageProfile::load_from_file("languages/english.yml")
        .expect("Failed to load English profile");
    
    // Test enhanced features
    assert_eq!(profile.word_composition.prefixes.len(), 3);
    assert_eq!(profile.word_composition.suffixes.len(), 3);
    assert_eq!(profile.word_composition.forbidden_transitions.len(), 4);
    assert_eq!(profile.style_rules.harmony_rules.len(), 3);
    assert_eq!(profile.style_rules.frequency_adjustments.len(), 7);
    
    // Test that profile validates
    assert!(profile.validate().is_ok());
}

#[test]
fn test_enhanced_profile_generation() {
    let mut rng = thread_rng();
    
    // Test German profile generation
    let german_generator = Name::<GermanLanguageProfile>::new();
    for _ in 0..10 {
        let name = german_generator.generate(&mut rng);
        assert!(!name.is_empty());
        assert!(name.len() <= 20); // Reasonable upper bound
    }
    
    // Test English profile generation
    let english_generator = Name::<EnglishLanguageProfile>::new();
    for _ in 0..10 {
        let name = english_generator.generate(&mut rng);
        assert!(!name.is_empty());
        assert!(name.len() <= 20); // Reasonable upper bound
    }
}

#[test]
fn test_enhanced_profile_phoneme_coverage() {
    let german_profile = LanguageProfile::load_from_file("languages/german.yml")
        .expect("Failed to load German profile");
    
    // Check that enhanced phonemes are present
    let phoneme_ipas: Vec<&str> = german_profile.phonetic_inventory.phonemes.iter()
        .map(|p| p.ipa.as_str()).collect();
    
    assert!(phoneme_ipas.contains(&"[ç]")); // ich-sound
    assert!(phoneme_ipas.contains(&"[ə]")); // schwa
    assert!(phoneme_ipas.contains(&"[ʃ]")); // sch-sound
    assert!(phoneme_ipas.contains(&"[ʁ]")); // German r
    
    // Check phoneme groups
    assert!(german_profile.phonetic_inventory.phoneme_groups.contains_key("central_vowels"));
    assert!(german_profile.phonetic_inventory.phoneme_groups.contains_key("fricatives"));
    
    let fricatives = german_profile.phonetic_inventory.phoneme_groups.get("fricatives").unwrap();
    assert!(fricatives.contains(&"[ç]".to_string()));
}

#[test]
fn test_harmony_rule_structure() {
    let german_profile = LanguageProfile::load_from_file("languages/german.yml")
        .expect("Failed to load German profile");
    
    // Check harmony rules
    let harmony_rules = &german_profile.style_rules.harmony_rules;
    assert!(!harmony_rules.is_empty());
    
    let rule_names: Vec<&str> = harmony_rules.iter().map(|r| r.name.as_str()).collect();
    assert!(rule_names.contains(&"front_vowel_harmony"));
    assert!(rule_names.contains(&"back_vowel_harmony"));
    assert!(rule_names.contains(&"consonant_cluster_avoidance"));
    assert!(rule_names.contains(&"vowel_length_consistency"));
    
    // Check rule structure
    for rule in harmony_rules {
        assert!(!rule.name.is_empty());
        assert!(!rule.condition.is_empty());
        assert!(!rule.requirement.is_empty());
        assert!(rule.strength >= 0.0 && rule.strength <= 1.0);
    }
}

#[test]
fn test_frequency_adjustments() {
    let german_profile = LanguageProfile::load_from_file("languages/german.yml")
        .expect("Failed to load German profile");
    
    let adjustments = &german_profile.style_rules.frequency_adjustments;
    assert!(!adjustments.is_empty());
    
    // Check specific adjustments
    assert!(adjustments.contains_key("word_initial"));
    assert!(adjustments.contains_key("word_medial"));
    assert!(adjustments.contains_key("word_final"));
    assert!(adjustments.contains_key("stressed_syllable"));
    assert!(adjustments.contains_key("unstressed_syllable"));
    
    // Check reasonable values
    for (key, value) in adjustments {
        assert!(*value >= 0.0 && *value <= 2.0, "Adjustment {} has unreasonable value {}", key, value);
    }
}

#[test]
fn test_morphological_rules() {
    let german_profile = LanguageProfile::load_from_file("languages/german.yml")
        .expect("Failed to load German profile");
    
    // Test prefix frequencies sum to reasonable value
    let total_prefix_freq: f32 = german_profile.word_composition.prefixes.iter()
        .map(|p| p.frequency).sum();
    assert!(total_prefix_freq <= 1.0, "Total prefix frequency should not exceed 1.0");
    
    // Test suffix frequencies sum to reasonable value
    let total_suffix_freq: f32 = german_profile.word_composition.suffixes.iter()
        .map(|s| s.frequency).sum();
    assert!(total_suffix_freq <= 1.0, "Total suffix frequency should not exceed 1.0");
    
    // Test forbidden transitions
    let forbidden_transitions = &german_profile.word_composition.forbidden_transitions;
    assert!(!forbidden_transitions.is_empty());
    
    for transition in forbidden_transitions {
        assert!(!transition.coda.is_empty());
        assert!(!transition.onset.is_empty());
        assert!(transition.forbidden);
    }
}

#[test]
fn test_profile_consistency() {
    let german_profile = LanguageProfile::load_from_file("languages/german.yml")
        .expect("Failed to load German profile");
    
    // Test that all phonemes used in prefixes/suffixes are defined
    for prefix in &german_profile.word_composition.prefixes {
        for phoneme in &prefix.phonemes {
            assert!(german_profile.phonetic_inventory.get_phoneme(phoneme).is_some(),
                   "Phoneme {} in prefix {} is not defined in inventory", phoneme, prefix.grapheme);
        }
    }
    
    for suffix in &german_profile.word_composition.suffixes {
        for phoneme in &suffix.phonemes {
            assert!(german_profile.phonetic_inventory.get_phoneme(phoneme).is_some(),
                   "Phoneme {} in suffix {} is not defined in inventory", phoneme, suffix.grapheme);
        }
    }
}

#[test]
fn test_direct_profile_usage_with_enhancement() {
    let german_profile = LanguageProfile::load_from_file("languages/german.yml")
        .expect("Failed to load German profile");
    let generator = LanguageProfileGenerator::new(&german_profile);
    
    let mut rng = thread_rng();
    let mut names = Vec::new();
    
    // Generate names and check they're reasonable
    for _ in 0..50 {
        let name = generator.generate(&mut rng);
        names.push(name);
    }
    
    // Check names are not empty
    assert!(names.iter().all(|n| !n.is_empty()));
    
    // Check reasonable length distribution
    let avg_length: f64 = names.iter().map(|n| n.len()).sum::<usize>() as f64 / names.len() as f64;
    assert!(avg_length >= 2.0 && avg_length <= 15.0, "Average name length {} is unreasonable", avg_length);
    
    // Check some variety in names
    let unique_names: std::collections::HashSet<String> = names.into_iter().collect();
    assert!(unique_names.len() >= 30, "Not enough variety in generated names");
}