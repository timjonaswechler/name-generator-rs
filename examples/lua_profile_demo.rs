//! Lua Profile Demo
//!
//! This example demonstrates the Lua-based language profile system,
//! showcasing dynamic profile creation and name generation.

use name_generator::language_profile::{LuaProfileLoader, LuaLanguageProfile};
use name_generator::generators::LuaProfileGenerator;
use rand::thread_rng;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Lua Language Profile System Demo ===\n");
    
    // 1. Create a Lua profile loader
    let mut loader = LuaProfileLoader::new()?;
    
    // 2. Create a test profile
    println!("1. Creating test profile...");
    let test_profile = loader.create_test_profile()?;
    println!("   Profile name: {}", test_profile.get_name()?);
    println!("   Phonemes: {}", test_profile.get_phonemes()?.len());
    println!("   Syllable patterns: {}", test_profile.get_syllable_patterns()?.len());
    
    // 3. Generate names with the test profile
    println!("\n2. Generating names with test profile:");
    let generator = LuaProfileGenerator::new(test_profile);
    let mut rng = thread_rng();
    
    for i in 1..=10 {
        let name = generator.generate(&mut rng)?;
        println!("   {}. {}", i, name);
    }
    
    // 4. Try to load the German Lua profile if it exists
    println!("\n3. Attempting to load German Lua profile...");
    match loader.load_builtin("german") {
        Ok(german_profile) => {
            println!("   Successfully loaded German profile!");
            println!("   Profile name: {}", german_profile.get_name()?);
            println!("   Phonemes: {}", german_profile.get_phonemes()?.len());
            
            println!("\n   Generating German names:");
            let german_generator = LuaProfileGenerator::new(german_profile);
            
            for i in 1..=5 {
                let name = german_generator.generate(&mut rng)?;
                println!("   {}. {}", i, name);
            }
        }
        Err(e) => {
            println!("   Could not load German profile: {}", e);
            println!("   This is expected if german.lua is not properly set up.");
        }
    }
    
    // 5. Demonstrate dynamic Lua profile creation
    println!("\n4. Creating dynamic Lua profile...");
    let dynamic_script = r#"
        profile = {}
        
        profile.name = "Dynamic Test Language"
        
        -- Programmatically create phonemes
        profile.phonemes = {}
        
        -- Add consonants
        local consonants = {
            {ipa = "[p]", grapheme = "p"},
            {ipa = "[t]", grapheme = "t"},
            {ipa = "[k]", grapheme = "k"},
            {ipa = "[m]", grapheme = "m"},
            {ipa = "[n]", grapheme = "n"},
            {ipa = "[l]", grapheme = "l"},
        }
        
        for _, cons in ipairs(consonants) do
            table.insert(profile.phonemes, {
                ipa = cons.ipa,
                phoneme_type = "Consonant",
                grapheme = cons.grapheme,
                frequency = 0.7
            })
        end
        
        -- Add vowels
        local vowels = {
            {ipa = "[a]", grapheme = "a", freq = 0.9},
            {ipa = "[e]", grapheme = "e", freq = 0.8},
            {ipa = "[i]", grapheme = "i", freq = 0.8},
            {ipa = "[o]", grapheme = "o", freq = 0.6},
            {ipa = "[u]", grapheme = "u", freq = 0.6},
        }
        
        for _, vowel in ipairs(vowels) do
            table.insert(profile.phonemes, {
                ipa = vowel.ipa,
                phoneme_type = "Vowel",
                grapheme = vowel.grapheme,
                frequency = vowel.freq
            })
        end
        
        -- Dynamic syllable patterns
        profile.syllable_patterns = {
            {pattern = "CV", frequency = 0.5},
            {pattern = "CVC", frequency = 0.4},
            {pattern = "VC", frequency = 0.1},
        }
        
        -- Simple harmony rule
        function profile.apply_harmony_rules(syllables, context)
            -- Make all vowels in multi-syllable words more similar
            if #syllables > 1 then
                local first_vowel = nil
                if #syllables[1].nucleus > 0 then
                    first_vowel = syllables[1].nucleus[1]
                end
                
                if first_vowel then
                    for i = 2, #syllables do
                        if #syllables[i].nucleus > 0 then
                            -- Simple vowel harmony: copy first vowel
                            syllables[i].nucleus[1] = first_vowel
                        end
                    end
                end
            end
        end
    "#;
    
    let dynamic_profile = LuaLanguageProfile::from_lua_script(dynamic_script)?;
    println!("   Created profile: {}", dynamic_profile.get_name()?);
    println!("   Phonemes: {}", dynamic_profile.get_phonemes()?.len());
    
    println!("\n   Generating names with dynamic profile:");
    let dynamic_generator = LuaProfileGenerator::new(dynamic_profile);
    
    for i in 1..=5 {
        let name = dynamic_generator.generate(&mut rng)?;
        println!("   {}. {}", i, name);
    }
    
    // 6. Show profile validation
    println!("\n5. Profile validation examples...");
    
    // Valid profile
    let valid_script = r#"
        profile = {}
        profile.name = "Valid Profile"
        profile.phonemes = {
            {ipa = "[p]", phoneme_type = "Consonant", grapheme = "p", frequency = 0.8}
        }
        profile.syllable_patterns = {
            {pattern = "CV", frequency = 0.5}
        }
    "#;
    
    let valid_profile = LuaLanguageProfile::from_lua_script(valid_script)?;
    match valid_profile.validate() {
        Ok(_) => println!("   ✓ Valid profile passed validation"),
        Err(e) => println!("   ✗ Valid profile failed validation: {}", e),
    }
    
    // Invalid profile (bad frequency)
    let invalid_script = r#"
        profile = {}
        profile.name = "Invalid Profile"
        profile.phonemes = {
            {ipa = "[p]", phoneme_type = "Consonant", grapheme = "p", frequency = 2.0}
        }
        profile.syllable_patterns = {
            {pattern = "CV", frequency = 0.5}
        }
    "#;
    
    let invalid_profile = LuaLanguageProfile::from_lua_script(invalid_script)?;
    match invalid_profile.validate() {
        Ok(_) => println!("   ✗ Invalid profile incorrectly passed validation"),
        Err(e) => println!("   ✓ Invalid profile correctly failed validation: {}", e),
    }
    
    println!("\n=== Demo Complete ===");
    println!("\nKey features demonstrated:");
    println!("• Lua-based language profiles with dynamic content");
    println!("• Programmable harmony rules as Lua functions");
    println!("• Profile validation and error handling");
    println!("• Name generation with Lua profiles");
    println!("• Backward compatibility with existing YAML system");
    
    Ok(())
}