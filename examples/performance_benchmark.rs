//! Performance benchmark for LanguageProfile system
//!
//! This example measures generation speed, memory usage, and compares
//! the LanguageProfile system with the traditional pattern-based approach.

use name_generator::{
    core::Name,
    categories::{
        examples::StandardStar,
        profile_examples::{GermanLanguageProfile, EnglishLanguageProfile},
    },
    language_profile::profile::LanguageProfile,
    generators::profile_generator::LanguageProfileGenerator,
};
use rand::thread_rng;
use std::time::{Duration, Instant};

fn main() {
    println!("=== LanguageProfile Performance Benchmark ===\n");
    
    let mut rng = thread_rng();
    
    // Test different generation counts
    let test_counts = [100, 1000, 5000, 10000];
    
    for &count in &test_counts {
        println!("--- Performance Test: {} names ---", count);
        
        // 1. Pattern-based system benchmark
        println!("1. Pattern-based system:");
        let start = Instant::now();
        let mut pattern_names = Vec::new();
        
        for _ in 0..count {
            let name = Name::<StandardStar>::new().generate(&mut rng);
            pattern_names.push(name);
        }
        
        let pattern_duration = start.elapsed();
        let pattern_speed = count as f64 / pattern_duration.as_secs_f64();
        
        println!("   Time: {:?}", pattern_duration);
        println!("   Speed: {:.0} names/second", pattern_speed);
        println!("   Avg length: {:.1} chars", 
                 pattern_names.iter().map(|n| n.len()).sum::<usize>() as f64 / count as f64);
        
        // 2. German LanguageProfile benchmark
        println!("2. German LanguageProfile:");
        let start = Instant::now();
        let mut german_names = Vec::new();
        
        for _ in 0..count {
            let name = Name::<GermanLanguageProfile>::new().generate(&mut rng);
            german_names.push(name);
        }
        
        let german_duration = start.elapsed();
        let german_speed = count as f64 / german_duration.as_secs_f64();
        
        println!("   Time: {:?}", german_duration);
        println!("   Speed: {:.0} names/second", german_speed);
        println!("   Avg length: {:.1} chars", 
                 german_names.iter().map(|n| n.len()).sum::<usize>() as f64 / count as f64);
        
        // 3. English LanguageProfile benchmark
        println!("3. English LanguageProfile:");
        let start = Instant::now();
        let mut english_names = Vec::new();
        
        for _ in 0..count {
            let name = Name::<EnglishLanguageProfile>::new().generate(&mut rng);
            english_names.push(name);
        }
        
        let english_duration = start.elapsed();
        let english_speed = count as f64 / english_duration.as_secs_f64();
        
        println!("   Time: {:?}", english_duration);
        println!("   Speed: {:.0} names/second", english_speed);
        println!("   Avg length: {:.1} chars", 
                 english_names.iter().map(|n| n.len()).sum::<usize>() as f64 / count as f64);
        
        // 4. Direct profile usage benchmark
        println!("4. Direct Profile Usage:");
        let german_profile = LanguageProfile::load_from_file("languages/german.yml")
            .expect("Failed to load German profile");
        let generator = LanguageProfileGenerator::new(&german_profile);
        
        let start = Instant::now();
        let mut direct_names = Vec::new();
        
        for _ in 0..count {
            let name = generator.generate(&mut rng);
            direct_names.push(name);
        }
        
        let direct_duration = start.elapsed();
        let direct_speed = count as f64 / direct_duration.as_secs_f64();
        
        println!("   Time: {:?}", direct_duration);
        println!("   Speed: {:.0} names/second", direct_speed);
        println!("   Avg length: {:.1} chars", 
                 direct_names.iter().map(|n| n.len()).sum::<usize>() as f64 / count as f64);
        
        // Performance comparison
        println!("\n   Performance Comparison:");
        println!("   Pattern-based:    {:.0} names/s (baseline)", pattern_speed);
        println!("   German Profile:   {:.0} names/s ({:.1}x)", german_speed, german_speed / pattern_speed);
        println!("   English Profile:  {:.0} names/s ({:.1}x)", english_speed, english_speed / pattern_speed);
        println!("   Direct Profile:   {:.0} names/s ({:.1}x)", direct_speed, direct_speed / pattern_speed);
        
        println!();
    }
    
    // Memory usage estimation
    println!("--- Memory Usage Analysis ---");
    
    // Profile loading
    let german_profile = LanguageProfile::load_from_file("languages/german.yml")
        .expect("Failed to load German profile");
    let english_profile = LanguageProfile::load_from_file("languages/english.yml")
        .expect("Failed to load English profile");
    
    println!("Profile sizes:");
    println!("  German profile phonemes: {}", german_profile.phonetic_inventory.phonemes.len());
    println!("  German profile groups: {}", german_profile.phonetic_inventory.phoneme_groups.len());
    println!("  German profile patterns: {}", german_profile.syllable_structure.patterns.len());
    println!("  German profile onsets: {}", german_profile.syllable_structure.onsets.len());
    println!("  German profile nuclei: {}", german_profile.syllable_structure.nuclei.len());
    println!("  German profile codas: {}", german_profile.syllable_structure.codas.len());
    println!("  German profile prefixes: {}", german_profile.word_composition.prefixes.len());
    println!("  German profile suffixes: {}", german_profile.word_composition.suffixes.len());
    println!("  German profile forbidden_transitions: {}", german_profile.word_composition.forbidden_transitions.len());
    println!("  German profile harmony_rules: {}", german_profile.style_rules.harmony_rules.len());
    println!("  German profile frequency_adjustments: {}", german_profile.style_rules.frequency_adjustments.len());
    
    println!();
    println!("  English profile phonemes: {}", english_profile.phonetic_inventory.phonemes.len());
    println!("  English profile groups: {}", english_profile.phonetic_inventory.phoneme_groups.len());
    println!("  English profile patterns: {}", english_profile.syllable_structure.patterns.len());
    println!("  English profile onsets: {}", english_profile.syllable_structure.onsets.len());
    println!("  English profile nuclei: {}", english_profile.syllable_structure.nuclei.len());
    println!("  English profile codas: {}", english_profile.syllable_structure.codas.len());
    
    // Quality analysis
    println!("\n--- Quality Analysis ---");
    println!("Sample generated names:");
    
    let mut rng = thread_rng();
    
    println!("German names with enhanced profile:");
    for i in 1..=10 {
        let name = Name::<GermanLanguageProfile>::new().generate(&mut rng);
        println!("  {}: {}", i, name);
    }
    
    println!("\nEnglish names:");
    for i in 1..=10 {
        let name = Name::<EnglishLanguageProfile>::new().generate(&mut rng);
        println!("  {}: {}", i, name);
    }
    
    println!("\nPattern-based names:");
    for i in 1..=10 {
        let name = Name::<StandardStar>::new().generate(&mut rng);
        println!("  {}: {}", i, name);
    }
    
    println!("\n=== Performance Benchmark Complete ===");
}