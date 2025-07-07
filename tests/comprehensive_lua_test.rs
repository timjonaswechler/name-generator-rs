use name_generator::language_profile::profile::LanguageProfile;
use name_generator::generators::profile_generator::LanguageProfileGenerator;
use name_generator::scripting::LuaRuleEngine;
use rand::thread_rng;

#[test]
fn test_complete_lua_integration() {
    // This is a comprehensive test that verifies all aspects of the Lua integration
    
    // 1. Test Lua engine creation and basic functionality
    let mut engine = LuaRuleEngine::new().unwrap();
    
    // Test phonetic helper functions
    assert!(engine.execute_lua::<bool>("is_vowel('[a]')").unwrap());
    assert!(engine.execute_lua::<bool>("is_consonant('[p]')").unwrap());
    assert!(engine.execute_lua::<bool>("is_front_vowel('[i]')").unwrap());
    assert!(engine.execute_lua::<bool>("is_back_vowel('[u]')").unwrap());
    
    // 2. Test complex Lua script
    let script = r#"
        function test_complex_rule(syllables, context)
            local vowel_count = 0
            for i = 1, #syllables do
                local nucleus = syllables[i]:get_nucleus()
                if #nucleus > 0 and is_vowel(nucleus[1]) then
                    vowel_count = vowel_count + 1
                end
            end
            return vowel_count
        end
    "#;
    
    engine.load_script(script).unwrap();
    
    // 3. Test that profiles with mixed rule types work
    let profile = LanguageProfile::load_from_file("languages/lua_example.yml").unwrap();
    
    // Verify profile has expected structure
    assert_eq!(profile.name, "Lua Example");
    assert_eq!(profile.style_rules.harmony_rules.len(), 4);
    
    // Check that we have both types of rules
    let lua_rules: Vec<_> = profile.style_rules.harmony_rules.iter()
        .filter(|r| r.script.is_some())
        .collect();
    let hardcoded_rules: Vec<_> = profile.style_rules.harmony_rules.iter()
        .filter(|r| r.script.is_none())
        .collect();
    
    assert_eq!(lua_rules.len(), 3);
    assert_eq!(hardcoded_rules.len(), 1);
    
    // 4. Test name generation with mixed rules
    let generator = LanguageProfileGenerator::new(&profile);
    let mut rng = thread_rng();
    
    // Generate multiple names to test consistency
    let mut names = Vec::new();
    for _ in 0..20 {
        let name = generator.generate(&mut rng);
        assert!(!name.is_empty());
        assert!(name.len() >= 2); // Should have reasonable length
        names.push(name);
    }
    
    // Check that we got unique names (shows the system is working)
    names.sort();
    names.dedup();
    assert!(names.len() > 1, "Should generate varied names");
    
    // 5. Test backward compatibility with existing profiles
    let english_profile = LanguageProfile::load_from_file("languages/english.yml").unwrap();
    let english_generator = LanguageProfileGenerator::new(&english_profile);
    
    // Should work with existing profiles without Lua scripts
    for _ in 0..5 {
        let name = english_generator.generate(&mut rng);
        assert!(!name.is_empty());
    }
    
    // 6. Test that invalid Lua scripts are handled gracefully
    let mut bad_profile = LanguageProfile::create_simple_test_profile();
    bad_profile.style_rules.harmony_rules.push(name_generator::language_profile::profile::HarmonyRule {
        name: "bad_lua_rule".to_string(),
        condition: "test".to_string(),
        requirement: None,
        script: Some("invalid lua syntax here".to_string()),
        strength: 0.5,
    });
    
    // Should not crash with invalid Lua
    let bad_generator = LanguageProfileGenerator::new(&bad_profile);
    let name = bad_generator.generate(&mut rng);
    assert!(!name.is_empty()); // Should still work despite bad script
}

#[test]
fn test_lua_rule_api_completeness() {
    // Test that all the API functions work correctly
    let mut engine = LuaRuleEngine::new().unwrap();
    
    let api_test_script = r#"
        function test_api()
            -- Test vowel detection
            local vowels = {"[a]", "[e]", "[i]", "[o]", "[u]", "[ə]"}
            for i, vowel in ipairs(vowels) do
                if not is_vowel(vowel) then
                    return false
                end
            end
            
            -- Test consonant detection
            local consonants = {"[p]", "[t]", "[k]", "[m]", "[n]", "[s]", "[f]", "[l]", "[r]"}
            for i, consonant in ipairs(consonants) do
                if not is_consonant(consonant) then
                    return false
                end
            end
            
            -- Test front vowel detection
            local front_vowels = {"[i]", "[e]"}
            for i, vowel in ipairs(front_vowels) do
                if not is_front_vowel(vowel) then
                    return false
                end
            end
            
            -- Test back vowel detection
            local back_vowels = {"[u]", "[o]"}
            for i, vowel in ipairs(back_vowels) do
                if not is_back_vowel(vowel) then
                    return false
                end
            end
            
            -- Test reduce_to_schwa function
            local reduced = reduce_to_schwa({"[a]", "[e]", "[i]"})
            if #reduced ~= 1 or reduced[1] ~= "[ə]" then
                return false
            end
            
            return true
        end
    "#;
    
    engine.load_script(api_test_script).unwrap();
    let result: bool = engine.execute_lua("test_api()").unwrap();
    assert!(result, "All API functions should work correctly");
}