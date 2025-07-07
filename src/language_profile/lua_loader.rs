//! Lua profile loader and management system
//!
//! This module provides functionality to load and manage Lua-based language profiles.

use crate::language_profile::lua_profile::{LuaLanguageProfile, LuaProfileError};
use mlua::{Lua, Table};
use std::collections::HashMap;
use std::path::Path;
use std::error::Error;

/// Lua profile loader and management system
pub struct LuaProfileLoader {
    /// Lua runtime environment
    lua: Lua,
    /// Loaded common modules (as strings to avoid lifetime issues)
    common_modules: HashMap<String, String>,
}

impl LuaProfileLoader {
    /// Create a new Lua profile loader
    pub fn new() -> Result<Self, LuaProfileError> {
        let lua = Lua::new();
        
        // Initialize common modules
        let mut common_modules = HashMap::new();
        
        // Load common utility module if it exists
        if let Ok(common_module_content) = Self::load_common_module_content("languages/common.lua") {
            common_modules.insert("common".to_string(), common_module_content);
        }
        
        Ok(Self { lua, common_modules })
    }
    
    /// Load a Lua profile from file
    pub fn load_profile(&mut self, path: &str) -> Result<LuaLanguageProfile, Box<dyn Error>> {
        // First, load common modules into the Lua environment
        for (_name, content) in &self.common_modules {
            self.lua.load(content).exec()?;
            // The common module should set a global variable with its name
        }
        
        // Load the profile
        let profile = LuaLanguageProfile::load_from_file(path)?;
        
        Ok(profile)
    }
    
    /// Register a common module from string content
    pub fn register_common_module(&mut self, name: &str, content: String) -> Result<(), LuaProfileError> {
        self.common_modules.insert(name.to_string(), content);
        Ok(())
    }
    
    /// Load a common module from file
    fn load_common_module_content(path: &str) -> Result<String, LuaProfileError> {
        if Path::new(path).exists() {
            let content = std::fs::read_to_string(path)
                .map_err(|e| LuaProfileError::ValidationError(format!("Failed to read common module: {}", e)))?;
            Ok(content)
        } else {
            Err(LuaProfileError::ValidationError(format!("Common module not found: {}", path)))
        }
    }
    
    /// Get list of available profiles
    pub fn list_available_profiles(&self) -> Result<Vec<String>, Box<dyn Error>> {
        let mut profiles = Vec::new();
        
        if let Ok(entries) = std::fs::read_dir("languages/") {
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                
                if let Some(extension) = path.extension() {
                    if extension == "lua" {
                        if let Some(name) = path.file_stem() {
                            profiles.push(name.to_string_lossy().to_string());
                        }
                    }
                }
            }
        }
        
        Ok(profiles)
    }
    
    /// Load a built-in profile by name
    pub fn load_builtin(&mut self, language: &str) -> Result<LuaLanguageProfile, Box<dyn Error>> {
        let path = format!("languages/{}.lua", language);
        self.load_profile(&path)
    }
    
    /// Create a simple test profile for development
    pub fn create_test_profile(&self) -> Result<LuaLanguageProfile, LuaProfileError> {
        let lua_script = r#"
            profile = {}
            
            profile.name = "Test Language"
            
            profile.phonemes = {
                {ipa = "[p]", phoneme_type = "Consonant", grapheme = "p", frequency = 0.8},
                {ipa = "[t]", phoneme_type = "Consonant", grapheme = "t", frequency = 0.9},
                {ipa = "[k]", phoneme_type = "Consonant", grapheme = "k", frequency = 0.7},
                {ipa = "[a]", phoneme_type = "Vowel", grapheme = "a", frequency = 0.9},
                {ipa = "[e]", phoneme_type = "Vowel", grapheme = "e", frequency = 0.8},
                {ipa = "[i]", phoneme_type = "Vowel", grapheme = "i", frequency = 0.7},
            }
            
            profile.syllable_patterns = {
                {pattern = "CV", frequency = 0.4},
                {pattern = "CVC", frequency = 0.6},
            }
            
            function profile.apply_harmony_rules(syllables, context)
                -- Simple vowel harmony: if first syllable has front vowel, prefer front vowels
                local front_vowels = {"[e]", "[i]"}
                local back_vowels = {"[a]", "[o]", "[u]"}
                
                if #syllables > 0 and #syllables[1].nucleus > 0 then
                    local first_vowel = syllables[1].nucleus[1]
                    local is_front = false
                    
                    for _, vowel in ipairs(front_vowels) do
                        if first_vowel == vowel then
                            is_front = true
                            break
                        end
                    end
                    
                    if is_front then
                        -- Apply front vowel harmony
                        for i = 2, #syllables do
                            if #syllables[i].nucleus > 0 then
                                for _, vowel in ipairs(back_vowels) do
                                    if syllables[i].nucleus[1] == vowel then
                                        syllables[i].nucleus[1] = "[e]"  -- Convert to front vowel
                                        break
                                    end
                                end
                            end
                        end
                    end
                end
            end
        "#;
        
        // Save to a temporary file and load it
        let temp_path = "/tmp/test_lua_profile.lua";
        std::fs::write(temp_path, lua_script)
            .map_err(|e| LuaProfileError::ValidationError(format!("Failed to write test profile: {}", e)))?;
        
        let profile = LuaLanguageProfile::load_from_file(temp_path)?;
        
        // Clean up
        let _ = std::fs::remove_file(temp_path);
        
        Ok(profile)
    }
}

// Remove the extension impl as it's no longer needed
impl LuaLanguageProfile {
    /// Create a profile directly from Lua script (for testing)
    pub fn from_lua_script(lua_script: &str) -> Result<Self, LuaProfileError> {
        let lua = Lua::new();
        lua.load(lua_script).exec()?;
        
        Ok(LuaLanguageProfile::new(lua))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_lua_profile_loader_creation() {
        let loader = LuaProfileLoader::new();
        assert!(loader.is_ok());
    }
    
    #[test]
    fn test_test_profile_creation() {
        let loader = LuaProfileLoader::new().unwrap();
        let profile = loader.create_test_profile().unwrap();
        
        assert_eq!(profile.get_name().unwrap(), "Test Language");
        assert_eq!(profile.get_phonemes().unwrap().len(), 6);
        assert_eq!(profile.get_syllable_patterns().unwrap().len(), 2);
        assert!(profile.validate().is_ok());
    }
    
    #[test]
    fn test_harmony_rules() {
        let loader = LuaProfileLoader::new().unwrap();
        let profile = loader.create_test_profile().unwrap();
        
        // Create test syllables
        let mut syllables = vec![
            crate::language_profile::lua_profile::LuaSyllable {
                onset: vec!["[p]".to_string()],
                nucleus: vec!["[e]".to_string()],  // Front vowel
                coda: vec![],
                stressed: true,
                position: "initial".to_string(),
            },
            crate::language_profile::lua_profile::LuaSyllable {
                onset: vec!["[t]".to_string()],
                nucleus: vec!["[a]".to_string()],  // Back vowel - should change
                coda: vec![],
                stressed: false,
                position: "final".to_string(),
            },
        ];
        
        // Apply harmony rules
        assert!(profile.apply_harmony_rules(&mut syllables).is_ok());
        
        // Check that vowel harmony was applied
        assert_eq!(syllables[1].nucleus[0], "[e]");  // Should have changed from [a] to [e]
    }
}