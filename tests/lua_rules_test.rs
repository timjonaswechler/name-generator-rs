use name_generator::language_profile::profile::LanguageProfile;
use name_generator::generators::profile_generator::LanguageProfileGenerator;
use name_generator::scripting::LuaRuleEngine;
use rand::thread_rng;

#[test]
fn test_lua_rule_integration() {
    // Create a test profile with a Lua rule
    let mut profile = LanguageProfile::create_simple_test_profile();
    
    // Add a Lua rule for vowel reduction
    profile.style_rules.harmony_rules.push(name_generator::language_profile::profile::HarmonyRule {
        name: "vowel_reduction".to_string(),
        condition: "unstressed_syllable".to_string(),
        requirement: None,
        script: Some(r#"
            function apply_rule(syllables, context)
                -- Simple vowel reduction: replace vowels with schwa in non-initial syllables
                for i = 2, #syllables do
                    if syllables[i]:get_nucleus()[1] then
                        syllables[i]:set_nucleus({context:get_schwa()})
                    end
                end
            end
        "#.to_string()),
        strength: 0.8,
    });
    
    // Test that the generator can handle the Lua rule
    let generator = LanguageProfileGenerator::new(&profile);
    let mut rng = thread_rng();
    
    // Generate a name and ensure it works
    let name = generator.generate(&mut rng);
    assert!(!name.is_empty());
}

#[test]
fn test_lua_engine_with_phonetic_functions() {
    let mut engine = LuaRuleEngine::new().unwrap();
    
    let script = r#"
        function test_phonetic_functions()
            local vowel_test = is_vowel("[a]")
            local consonant_test = is_consonant("[p]")
            local front_test = is_front_vowel("[i]")
            local back_test = is_back_vowel("[u]")
            
            return vowel_test and consonant_test and front_test and back_test
        end
    "#;
    
    engine.load_script(script).unwrap();
    
    // Test that the phonetic functions work in Lua
    let function_result: bool = engine.execute_lua("test_phonetic_functions()").unwrap();
    assert!(function_result);
}

#[test]
fn test_backward_compatibility_with_hardcoded_rules() {
    // Test that existing hardcoded rules still work
    let profile = LanguageProfile::create_simple_test_profile();
    let generator = LanguageProfileGenerator::new(&profile);
    let mut rng = thread_rng();
    
    // This should work with the existing hardcoded rule system
    let name = generator.generate(&mut rng);
    assert!(!name.is_empty());
}