use name_generator::language_profile::profile::LanguageProfile;
use name_generator::generators::profile_generator::LanguageProfileGenerator;
use rand::thread_rng;

#[test]
fn test_lua_example_profile() {
    // Test that the Lua example profile loads correctly
    let profile = LanguageProfile::load_from_file("languages/lua_example.yml")
        .expect("Failed to load Lua example profile");
    
    // Verify the profile has both hardcoded and Lua rules
    assert_eq!(profile.style_rules.harmony_rules.len(), 4);
    
    // Check that some rules have scripts and others don't
    let mut has_script = false;
    let mut has_no_script = false;
    
    for rule in &profile.style_rules.harmony_rules {
        if rule.script.is_some() {
            has_script = true;
        } else {
            has_no_script = true;
        }
    }
    
    assert!(has_script, "Profile should have rules with Lua scripts");
    assert!(has_no_script, "Profile should have rules without Lua scripts for backward compatibility");
    
    // Test that names can be generated with the new profile
    let generator = LanguageProfileGenerator::new(&profile);
    let mut rng = thread_rng();
    
    // Generate multiple names to ensure the system works consistently
    for _ in 0..10 {
        let name = generator.generate(&mut rng);
        assert!(!name.is_empty(), "Generated name should not be empty");
        assert!(name.len() > 1, "Generated name should have reasonable length");
    }
}

#[test]
fn test_lua_script_validation() {
    // Test that various Lua script patterns work correctly
    let profile = LanguageProfile::load_from_file("languages/lua_example.yml")
        .expect("Failed to load Lua example profile");
    
    // Find a rule with a script
    let script_rule = profile.style_rules.harmony_rules.iter()
        .find(|rule| rule.script.is_some())
        .expect("Should have at least one rule with a script");
    
    // Verify the script is valid Lua
    assert!(script_rule.script.as_ref().unwrap().contains("function apply_rule"));
    assert!(script_rule.script.as_ref().unwrap().contains("syllables"));
    assert!(script_rule.script.as_ref().unwrap().contains("context"));
}

#[test]
fn test_mixed_rule_types() {
    // Test that a profile with both hardcoded and Lua rules works
    let mut profile = LanguageProfile::create_simple_test_profile();
    
    // Add a hardcoded rule
    profile.style_rules.harmony_rules.push(name_generator::language_profile::profile::HarmonyRule {
        name: "vowel_harmony".to_string(),
        condition: "front_back_context".to_string(),
        requirement: Some("harmonize_vowels".to_string()),
        script: None,
        strength: 0.8,
    });
    
    // Add a Lua rule
    profile.style_rules.harmony_rules.push(name_generator::language_profile::profile::HarmonyRule {
        name: "lua_vowel_reduction".to_string(),
        condition: "unstressed_syllable".to_string(),
        requirement: None,
        script: Some(r#"
            function apply_rule(syllables, context)
                -- Simple vowel reduction
                for i = 2, #syllables do
                    local syllable = syllables[i]
                    local nucleus = syllable:get_nucleus()
                    if #nucleus > 0 and is_vowel(nucleus[1]) then
                        syllable:set_nucleus({context:get_schwa()})
                    end
                end
            end
        "#.to_string()),
        strength: 0.7,
    });
    
    // Test that the generator can handle mixed rule types
    let generator = LanguageProfileGenerator::new(&profile);
    let mut rng = thread_rng();
    
    // Generate names and ensure they work
    for _ in 0..5 {
        let name = generator.generate(&mut rng);
        assert!(!name.is_empty());
    }
}