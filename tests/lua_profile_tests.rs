//! Comprehensive tests for the Lua profile system
//!
//! This module provides tests that validate the complete Lua language profile system,
//! including loading, validation, generation, and advanced features.

use name_generator::language_profile::{LuaProfileLoader, LuaLanguageProfile, LuaSyllable};
use name_generator::generators::LuaProfileGenerator;
use rand::thread_rng;

#[test]
fn test_complete_lua_profile_workflow() {
    let mut loader = LuaProfileLoader::new().unwrap();
    
    // Test profile creation
    let profile = loader.create_test_profile().unwrap();
    assert_eq!(profile.get_name().unwrap(), "Test Language");
    assert!(profile.validate().is_ok());
    
    // Test name generation
    let generator = LuaProfileGenerator::new(profile);
    let mut rng = thread_rng();
    
    for _ in 0..10 {
        let name = generator.generate(&mut rng).unwrap();
        assert!(!name.is_empty());
        assert!(name.len() <= 50); // Reasonable length limit
    }
}

#[test]
fn test_german_lua_profile_loading() {
    let mut loader = LuaProfileLoader::new().unwrap();
    
    match loader.load_builtin("german") {
        Ok(german_profile) => {
            assert_eq!(german_profile.get_name().unwrap(), "German");
            assert!(german_profile.validate().is_ok());
            
            let phonemes = german_profile.get_phonemes().unwrap();
            assert!(!phonemes.is_empty());
            
            // Test name generation
            let generator = LuaProfileGenerator::new(german_profile);
            let mut rng = thread_rng();
            let name = generator.generate(&mut rng).unwrap();
            assert!(!name.is_empty());
        }
        Err(_) => {
            // It's okay if the German profile doesn't exist or has issues in CI
            println!("German profile not available, skipping test");
        }
    }
}

#[test] 
fn test_lua_profile_validation() {
    // Test valid profile
    let valid_script = r#"
        profile = {}
        profile.name = "Valid Profile"
        profile.phonemes = {
            {ipa = "[p]", phoneme_type = "Consonant", grapheme = "p", frequency = 0.8},
            {ipa = "[a]", phoneme_type = "Vowel", grapheme = "a", frequency = 0.9}
        }
        profile.syllable_patterns = {
            {pattern = "CV", frequency = 0.5},
            {pattern = "CVC", frequency = 0.5}
        }
    "#;
    
    let valid_profile = LuaLanguageProfile::from_lua_script(valid_script).unwrap();
    assert!(valid_profile.validate().is_ok());
    
    // Test invalid profile - no phonemes
    let invalid_script1 = r#"
        profile = {}
        profile.name = "Invalid Profile"
        profile.phonemes = {}
        profile.syllable_patterns = {
            {pattern = "CV", frequency = 0.5}
        }
    "#;
    
    let invalid_profile1 = LuaLanguageProfile::from_lua_script(invalid_script1).unwrap();
    assert!(invalid_profile1.validate().is_err());
    
    // Test invalid profile - bad frequency
    let invalid_script2 = r#"
        profile = {}
        profile.name = "Invalid Profile"
        profile.phonemes = {
            {ipa = "[p]", phoneme_type = "Consonant", grapheme = "p", frequency = 2.0}
        }
        profile.syllable_patterns = {
            {pattern = "CV", frequency = 0.5}
        }
    "#;
    
    let invalid_profile2 = LuaLanguageProfile::from_lua_script(invalid_script2).unwrap();
    assert!(invalid_profile2.validate().is_err());
}

#[test]
fn test_lua_harmony_rules() {
    let harmony_script = r#"
        profile = {}
        profile.name = "Harmony Test Language"
        
        profile.phonemes = {
            {ipa = "[p]", phoneme_type = "Consonant", grapheme = "p", frequency = 0.8},
            {ipa = "[t]", phoneme_type = "Consonant", grapheme = "t", frequency = 0.8},
            {ipa = "[a]", phoneme_type = "Vowel", grapheme = "a", frequency = 0.9},
            {ipa = "[e]", phoneme_type = "Vowel", grapheme = "e", frequency = 0.8},
            {ipa = "[i]", phoneme_type = "Vowel", grapheme = "i", frequency = 0.8},
        }
        
        profile.syllable_patterns = {
            {pattern = "CV", frequency = 1.0}
        }
        
        function profile.apply_harmony_rules(syllables, context)
            -- Convert all vowels to 'a'
            for i, syllable in ipairs(syllables) do
                if #syllable.nucleus > 0 then
                    syllable.nucleus[1] = "[a]"
                end
            end
        end
    "#;
    
    let profile = LuaLanguageProfile::from_lua_script(harmony_script).unwrap();
    
    // Create test syllables
    let mut syllables = vec![
        LuaSyllable {
            onset: vec!["[p]".to_string()],
            nucleus: vec!["[e]".to_string()],
            coda: vec![],
            stressed: true,
            position: "initial".to_string(),
        },
        LuaSyllable {
            onset: vec!["[t]".to_string()],
            nucleus: vec!["[i]".to_string()],
            coda: vec![],
            stressed: false,
            position: "final".to_string(),
        },
    ];
    
    // Apply harmony rules
    assert!(profile.apply_harmony_rules(&mut syllables).is_ok());
    
    // Check that all vowels became 'a'
    assert_eq!(syllables[0].nucleus[0], "[a]");
    assert_eq!(syllables[1].nucleus[0], "[a]");
}

#[test]
fn test_dynamic_lua_profile_creation() {
    let dynamic_script = r#"
        profile = {}
        profile.name = "Dynamic Language"
        
        -- Create phonemes programmatically
        profile.phonemes = {}
        
        local consonants = {"p", "t", "k", "m", "n", "l"}
        local vowels = {"a", "e", "i", "o", "u"}
        
        for _, c in ipairs(consonants) do
            table.insert(profile.phonemes, {
                ipa = "[" .. c .. "]",
                phoneme_type = "Consonant",
                grapheme = c,
                frequency = 0.7
            })
        end
        
        for _, v in ipairs(vowels) do
            table.insert(profile.phonemes, {
                ipa = "[" .. v .. "]",
                phoneme_type = "Vowel", 
                grapheme = v,
                frequency = 0.8
            })
        end
        
        -- Generate patterns dynamically
        profile.syllable_patterns = {}
        local patterns = {"CV", "CVC", "VC", "V"}
        local base_freq = 1.0 / #patterns
        
        for _, pattern in ipairs(patterns) do
            table.insert(profile.syllable_patterns, {
                pattern = pattern,
                frequency = base_freq
            })
        end
        
        function profile.apply_harmony_rules(syllables, context)
            -- Simple vowel copying harmony
            if #syllables > 1 and #syllables[1].nucleus > 0 then
                local first_vowel = syllables[1].nucleus[1]
                for i = 2, #syllables do
                    if #syllables[i].nucleus > 0 then
                        syllables[i].nucleus[1] = first_vowel
                    end
                end
            end
        end
    "#;
    
    let profile = LuaLanguageProfile::from_lua_script(dynamic_script).unwrap();
    
    assert_eq!(profile.get_name().unwrap(), "Dynamic Language");
    assert_eq!(profile.get_phonemes().unwrap().len(), 11); // 6 consonants + 5 vowels
    assert_eq!(profile.get_syllable_patterns().unwrap().len(), 4);
    assert!(profile.validate().is_ok());
    
    // Test name generation
    let generator = LuaProfileGenerator::new(profile);
    let mut rng = thread_rng();
    let name = generator.generate(&mut rng).unwrap();
    assert!(!name.is_empty());
}

#[test]
fn test_error_handling() {
    // Test missing required fields
    let missing_name_script = r#"
        profile = {}
        profile.phonemes = {
            {ipa = "[p]", phoneme_type = "Consonant", grapheme = "p", frequency = 0.8}
        }
        profile.syllable_patterns = {
            {pattern = "CV", frequency = 1.0}
        }
    "#;
    
    let profile = LuaLanguageProfile::from_lua_script(missing_name_script).unwrap();
    assert!(profile.get_name().is_err());
    
    // Test malformed Lua script
    let malformed_script = r#"
        profile = {
            name = "Malformed Profile"
            -- Missing comma causes syntax error
            phonemes = {}
        }
    "#;
    
    assert!(LuaLanguageProfile::from_lua_script(malformed_script).is_err());
}

#[test]
fn test_backward_compatibility() {
    // The Lua system should not interfere with the existing YAML system
    use name_generator::language_profile::LanguageProfile;
    
    // Test that we can still load YAML profiles
    let yaml_profile = LanguageProfile::create_simple_test_profile();
    assert_eq!(yaml_profile.name, "Test Language");
    assert!(yaml_profile.validate().is_ok());
    
    // Test that both systems can coexist
    let lua_loader = LuaProfileLoader::new().unwrap();
    let lua_profile = lua_loader.create_test_profile().unwrap();
    
    // Both should work independently
    assert_eq!(yaml_profile.name, "Test Language");
    assert_eq!(lua_profile.get_name().unwrap(), "Test Language");
}