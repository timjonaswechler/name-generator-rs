use name_generator::language_profile::profile::LanguageProfile;
use name_generator::generators::profile_generator::LanguageProfileGenerator;
use rand::thread_rng;

fn main() {
    println!("Testing Lua scripting functionality...");
    
    // Test the new Lua example profile
    match LanguageProfile::load_from_file("languages/lua_example.yml") {
        Ok(profile) => {
            println!("✓ Successfully loaded Lua example profile");
            println!("  - Profile name: {}", profile.name);
            println!("  - Number of harmony rules: {}", profile.style_rules.harmony_rules.len());
            
            // Count rules with and without scripts
            let mut lua_rules = 0;
            let mut hardcoded_rules = 0;
            
            for rule in &profile.style_rules.harmony_rules {
                if rule.script.is_some() {
                    lua_rules += 1;
                    println!("  - Lua rule: {} (strength: {})", rule.name, rule.strength);
                } else {
                    hardcoded_rules += 1;
                    println!("  - Hardcoded rule: {} (strength: {})", rule.name, rule.strength);
                }
            }
            
            println!("  - Lua rules: {}, Hardcoded rules: {}", lua_rules, hardcoded_rules);
            
            // Generate some names
            let generator = LanguageProfileGenerator::new(&profile);
            let mut rng = thread_rng();
            
            println!("\nGenerating names with Lua rules:");
            for i in 1..=5 {
                let name = generator.generate(&mut rng);
                println!("  {}. {}", i, name);
            }
        }
        Err(e) => {
            eprintln!("✗ Failed to load Lua example profile: {}", e);
        }
    }
    
    // Test backward compatibility with existing profiles
    println!("\nTesting backward compatibility...");
    match LanguageProfile::load_from_file("languages/english.yml") {
        Ok(profile) => {
            println!("✓ Successfully loaded English profile (backward compatibility)");
            let generator = LanguageProfileGenerator::new(&profile);
            let mut rng = thread_rng();
            
            println!("Generating names with existing profile:");
            for i in 1..=3 {
                let name = generator.generate(&mut rng);
                println!("  {}. {}", i, name);
            }
        }
        Err(e) => {
            eprintln!("✗ Failed to load English profile: {}", e);
        }
    }
    
    println!("\n✓ Lua scripting integration complete!");
}