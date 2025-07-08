//! Lua rule engine for dynamic harmony rule execution
//!
//! This module provides a Lua scripting engine that allows users to define
//! custom linguistic rules without modifying Rust code.

use std::collections::HashMap;
use std::error::Error;
use mlua::{Lua, UserData, UserDataMethods};

/// Syllable structure exposed to Lua scripts
#[derive(Debug, Clone)]
pub struct LuaSyllable {
    pub onset: Vec<String>,
    pub nucleus: Vec<String>,
    pub coda: Vec<String>,
    pub stressed: bool,
    pub position: String, // "initial", "medial", "final"
}

impl UserData for LuaSyllable {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("get_onset", |_, this, ()| {
            Ok(this.onset.clone())
        });
        
        methods.add_method("get_nucleus", |_, this, ()| {
            Ok(this.nucleus.clone())
        });
        
        methods.add_method("get_coda", |_, this, ()| {
            Ok(this.coda.clone())
        });
        
        methods.add_method("is_stressed", |_, this, ()| {
            Ok(this.stressed)
        });
        
        methods.add_method("get_position", |_, this, ()| {
            Ok(this.position.clone())
        });
        
        methods.add_method_mut("set_nucleus", |_, this, new_nucleus: Vec<String>| {
            this.nucleus = new_nucleus;
            Ok(())
        });
        
        methods.add_method_mut("set_onset", |_, this, new_onset: Vec<String>| {
            this.onset = new_onset;
            Ok(())
        });
        
        methods.add_method_mut("set_coda", |_, this, new_coda: Vec<String>| {
            this.coda = new_coda;
            Ok(())
        });
    }
}

/// Context passed to Lua scripts
#[derive(Debug, Clone)]
pub struct RuleContext {
    pub schwa: String,
    pub language_features: HashMap<String, String>,
}

impl UserData for RuleContext {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("get_schwa", |_, this, ()| {
            Ok(this.schwa.clone())
        });
        
        methods.add_method("get_feature", |_, this, name: String| {
            Ok(this.language_features.get(&name).cloned())
        });
    }
}

/// Lua rule engine for executing custom harmony rules
pub struct LuaRuleEngine {
    lua: Lua,
}

impl LuaRuleEngine {
    /// Create a new Lua rule engine
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let lua = Lua::new();
        
        // Add helper functions to Lua global scope
        lua.globals().set("is_vowel", lua.create_function(|_, phoneme: String| {
            Ok(Self::is_vowel_phoneme(&phoneme))
        })?)?;
        
        lua.globals().set("is_consonant", lua.create_function(|_, phoneme: String| {
            Ok(Self::is_consonant_phoneme(&phoneme))
        })?)?;
        
        lua.globals().set("is_front_vowel", lua.create_function(|_, phoneme: String| {
            Ok(Self::is_front_vowel_phoneme(&phoneme))
        })?)?;
        
        lua.globals().set("is_back_vowel", lua.create_function(|_, phoneme: String| {
            Ok(Self::is_back_vowel_phoneme(&phoneme))
        })?)?;
        
        lua.globals().set("reduce_to_schwa", lua.create_function(|_, _vowels: Vec<String>| {
            Ok(vec!["[ə]".to_string()])
        })?)?;
        
        Ok(LuaRuleEngine { lua })
    }
    
    /// Load a Lua script into the engine
    pub fn load_script(&mut self, script: &str) -> Result<(), Box<dyn Error>> {
        self.lua.load(script).exec()?;
        Ok(())
    }
    
    /// Apply a rule to syllables using Lua script
    pub fn apply_rule(&self, syllables: &mut [crate::generators::profile_generator::Syllable], context: &RuleContext) -> Result<(), Box<dyn Error>> {
        // Convert syllables to Lua-compatible format
        let lua_syllables: Vec<LuaSyllable> = syllables.iter().enumerate().map(|(i, syl)| {
            let position = if i == 0 {
                "initial".to_string()
            } else if i == syllables.len() - 1 {
                "final".to_string()
            } else {
                "medial".to_string()
            };
            
            LuaSyllable {
                onset: syl.onset.clone(),
                nucleus: syl.nucleus.clone(),
                coda: syl.coda.clone(),
                stressed: false, // TODO: Add stress information to Syllable struct
                position,
            }
        }).collect();
        
        // Call the Lua apply_rule function
        let apply_rule_func: mlua::Function = self.lua.globals().get("apply_rule")?;
        apply_rule_func.call::<_, ()>((lua_syllables.clone(), context.clone()))?;
        
        // Convert back to Rust syllables
        for (i, lua_syl) in lua_syllables.iter().enumerate() {
            if i < syllables.len() {
                syllables[i].onset = lua_syl.onset.clone();
                syllables[i].nucleus = lua_syl.nucleus.clone();
                syllables[i].coda = lua_syl.coda.clone();
            }
        }
        
        Ok(())
    }
    
    /// Execute Lua code and return the result (for testing)
    pub fn execute_lua<T>(&self, code: &str) -> Result<T, Box<dyn Error>>
    where
        for<'lua> T: mlua::FromLua<'lua>,
    {
        Ok(self.lua.load(code).eval()?)
    }
    fn is_vowel_phoneme(phoneme: &str) -> bool {
        // Simple vowel detection based on IPA content
        let vowel_symbols = [
            "a", "e", "i", "o", "u", "ɑ", "ɒ", "ɔ", "ɛ", "ɪ", "ʊ", "ʌ", "ə", "ɜ", "æ", "ɐ",
            "y", "ø", "œ", "ɵ", "ɤ", "ɯ", "ɨ", "ʏ", "ʉ", "ɘ", "ɞ", "ɶ"
        ];
        
        let content = phoneme.trim_matches(['[', ']']);
        vowel_symbols.iter().any(|&v| content.contains(v))
    }
    
    /// Helper function to check if a phoneme is a consonant
    fn is_consonant_phoneme(phoneme: &str) -> bool {
        !Self::is_vowel_phoneme(phoneme)
    }
    
    /// Helper function to check if a phoneme is a front vowel
    fn is_front_vowel_phoneme(phoneme: &str) -> bool {
        let front_vowels = ["i", "e", "ɛ", "æ", "ɪ", "y", "ø", "œ", "ʏ"];
        let content = phoneme.trim_matches(['[', ']']);
        front_vowels.iter().any(|&v| content.contains(v))
    }
    
    /// Helper function to check if a phoneme is a back vowel
    fn is_back_vowel_phoneme(phoneme: &str) -> bool {
        let back_vowels = ["u", "o", "ɔ", "ɑ", "ɒ", "ʊ", "ʌ", "ɯ", "ɤ", "ɨ"];
        let content = phoneme.trim_matches(['[', ']']);
        back_vowels.iter().any(|&v| content.contains(v))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_lua_engine_creation() {
        let engine = LuaRuleEngine::new();
        assert!(engine.is_ok());
    }
    
    #[test]
    fn test_vowel_detection() {
        assert!(LuaRuleEngine::is_vowel_phoneme("[a]"));
        assert!(LuaRuleEngine::is_vowel_phoneme("[ə]"));
        assert!(!LuaRuleEngine::is_vowel_phoneme("[p]"));
        assert!(!LuaRuleEngine::is_vowel_phoneme("[t]"));
    }
    
    #[test]
    fn test_front_back_vowel_detection() {
        assert!(LuaRuleEngine::is_front_vowel_phoneme("[i]"));
        assert!(LuaRuleEngine::is_front_vowel_phoneme("[e]"));
        assert!(LuaRuleEngine::is_back_vowel_phoneme("[u]"));
        assert!(LuaRuleEngine::is_back_vowel_phoneme("[o]"));
    }
    
    #[test]
    fn test_basic_lua_script() {
        let mut engine = LuaRuleEngine::new().unwrap();
        let script = r#"
            function apply_rule(syllables, context)
                -- Simple test rule
                return true
            end
        "#;
        
        assert!(engine.load_script(script).is_ok());
    }
}