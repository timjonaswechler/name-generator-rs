//! LanguageProfile system demonstration
//!
//! This example demonstrates the LanguageProfile system and its features
//! for phonetically accurate name generation.

use name_generator::core::Name;
use name_generator::categories::profile_examples::*;
use name_generator::language_profile::profile::LanguageProfile;
use name_generator::generators::profile_generator::LanguageProfileGenerator;
use rand::thread_rng;

fn main() {
    let mut rng = thread_rng();
    
    println!("=== LanguageProfile System Demo ===\n");
    
    // 1. Direct profile usage
    println!("1. Direct Profile Usage:");
    let test_profile = LanguageProfile::create_simple_test_profile();
    let generator = LanguageProfileGenerator::new(&test_profile);
    println!("Test language names (direct profile):");
    for i in 0..5 {
        let name = generator.generate(&mut rng);
        println!("   {}: {}", i + 1, name);
    }
    
    println!("\n2. Category-based Usage (New LanguageProfile System):");
    
    // Category-based usage (integrated with existing system)
    println!("German names (category-based):");
    let german_gen = Name::<GermanLanguageProfile>::new();
    for i in 0..5 {
        let name = german_gen.generate(&mut rng);
        println!("   {}: {}", i + 1, name);
    }
    
    println!("\nEnglish names (category-based):");
    let english_gen = Name::<EnglishLanguageProfile>::new();
    for i in 0..5 {
        let name = english_gen.generate(&mut rng);
        println!("   {}: {}", i + 1, name);
    }
    
    println!("\n3. Profile Analysis:");
    analyze_profile(&test_profile);
    
    println!("\n=== LanguageProfile Demo Complete ===");
}

fn analyze_profile(profile: &LanguageProfile) {
    println!("\n--- {} Profile Analysis ---", profile.name);
    println!("Phonemes: {}", profile.phonetic_inventory.phonemes.len());
    println!("Phoneme groups: {}", profile.phonetic_inventory.phoneme_groups.len());
    println!("Syllable patterns: {}", profile.syllable_structure.patterns.len());
    println!("Onsets: {}", profile.syllable_structure.onsets.len());
    println!("Nuclei: {}", profile.syllable_structure.nuclei.len());
    println!("Codas: {}", profile.syllable_structure.codas.len());
    println!("Min syllables: {}", profile.word_composition.min_syllables);
    println!("Max syllables: {}", profile.word_composition.max_syllables);
    
    // Show some example phonemes
    println!("\nExample phonemes:");
    for (i, phoneme) in profile.phonetic_inventory.phonemes.iter().take(3).enumerate() {
        println!("  {}: {} -> '{}' ({})", 
                 i + 1, 
                 phoneme.ipa, 
                 phoneme.grapheme, 
                 if phoneme.is_vowel() { "vowel" } else { "consonant" });
    }
    
    // Show syllable patterns
    println!("\nSyllable patterns:");
    for (i, pattern) in profile.syllable_structure.patterns.iter().enumerate() {
        println!("  {}: {} (frequency: {:.2})", 
                 i + 1, 
                 pattern.pattern, 
                 pattern.frequency);
    }
}