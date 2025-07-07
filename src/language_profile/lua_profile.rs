//! Lua-based language profile system
//!
//! This module provides a Lua-based language profile system that allows
//! for dynamic and programmable language profile creation.

use mlua::{Lua, Table, Function, Error as LuaError};
use std::error::Error;
use std::fmt;

/// Lua-based phoneme representation
#[derive(Debug, Clone)]
pub struct LuaPhoneme {
    /// IPA notation for the phoneme
    pub ipa: String,
    /// Type of phoneme (Consonant, Vowel, etc.)
    pub phoneme_type: String,
    /// Grapheme representation
    pub grapheme: String,
    /// Frequency weight (0.0 - 1.0)
    pub frequency: f32,
}

/// Lua-based syllable representation
#[derive(Debug, Clone)]
pub struct LuaSyllable {
    /// IPA strings for onset consonants
    pub onset: Vec<String>,
    /// IPA strings for nucleus vowels
    pub nucleus: Vec<String>,
    /// IPA strings for coda consonants
    pub coda: Vec<String>,
    /// Whether this syllable is stressed
    pub stressed: bool,
    /// Position in word (initial, medial, final)
    pub position: String,
}

/// Lua-based syllable pattern
#[derive(Debug, Clone)]
pub struct LuaSyllablePattern {
    /// Pattern string (e.g., "CVC", "CV")
    pub pattern: String,
    /// Frequency weight (0.0 - 1.0)
    pub frequency: f32,
}

/// Lua-based phoneme cluster
#[derive(Debug, Clone)]
pub struct LuaPhonemeCluster {
    /// IPA strings of phonemes in the cluster
    pub phonemes: Vec<String>,
    /// Frequency weight (0.0 - 1.0)
    pub frequency: f32,
}

/// Main Lua language profile structure
pub struct LuaLanguageProfile {
    /// Lua runtime environment
    lua: Lua,
}

/// Error type for Lua profile operations
#[derive(Debug)]
pub enum LuaProfileError {
    /// Lua runtime error
    LuaError(LuaError),
    /// Profile validation error
    ValidationError(String),
    /// Missing required field
    MissingField(String),
    /// Type conversion error
    ConversionError(String),
}

impl fmt::Display for LuaProfileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LuaProfileError::LuaError(e) => write!(f, "Lua error: {}", e),
            LuaProfileError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            LuaProfileError::MissingField(field) => write!(f, "Missing field: {}", field),
            LuaProfileError::ConversionError(msg) => write!(f, "Conversion error: {}", msg),
        }
    }
}

impl Error for LuaProfileError {}

impl From<LuaError> for LuaProfileError {
    fn from(error: LuaError) -> Self {
        LuaProfileError::LuaError(error)
    }
}

impl LuaLanguageProfile {
    /// Create a new Lua language profile from a file
    pub fn load_from_file(path: &str) -> Result<Self, LuaProfileError> {
        let lua = Lua::new();
        let content = std::fs::read_to_string(path)
            .map_err(|e| LuaProfileError::ValidationError(format!("Failed to read file: {}", e)))?;
        
        // Load the Lua profile script and store in global 'profile'
        lua.load(&content).exec()?;
        
        Ok(Self { lua })
    }
    
    /// Create a new instance with given Lua runtime (for internal use)
    pub(crate) fn new(lua: Lua) -> Self {
        Self { lua }
    }
    
    /// Get the profile data table from Lua globals
    fn get_profile_data(&self) -> Result<Table, LuaProfileError> {
        let profile: Table = self.lua.globals().get("profile")
            .map_err(|_| LuaProfileError::MissingField("profile".to_string()))?;
        Ok(profile)
    }
    
    /// Get the profile name
    pub fn get_name(&self) -> Result<String, LuaProfileError> {
        let profile_data = self.get_profile_data()?;
        profile_data.get("name")
            .map_err(|_| LuaProfileError::MissingField("name".to_string()))
    }
    
    /// Get phonemes from the profile
    pub fn get_phonemes(&self) -> Result<Vec<LuaPhoneme>, LuaProfileError> {
        let profile_data = self.get_profile_data()?;
        let phonemes_table: Table = profile_data.get("phonemes")
            .map_err(|_| LuaProfileError::MissingField("phonemes".to_string()))?;
        
        let mut phonemes = Vec::new();
        for entry in phonemes_table.sequence_values::<Table>() {
            let table = entry?;
            let phoneme = LuaPhoneme {
                ipa: table.get("ipa")?,
                phoneme_type: table.get("phoneme_type")?,
                grapheme: table.get("grapheme")?,
                frequency: table.get("frequency")?,
            };
            phonemes.push(phoneme);
        }
        
        Ok(phonemes)
    }
    
    /// Get syllable patterns from the profile
    pub fn get_syllable_patterns(&self) -> Result<Vec<LuaSyllablePattern>, LuaProfileError> {
        let profile_data = self.get_profile_data()?;
        let patterns_table: Table = profile_data.get("syllable_patterns")
            .map_err(|_| LuaProfileError::MissingField("syllable_patterns".to_string()))?;
        
        let mut patterns = Vec::new();
        for entry in patterns_table.sequence_values::<Table>() {
            let table = entry?;
            let pattern = LuaSyllablePattern {
                pattern: table.get("pattern")?,
                frequency: table.get("frequency")?,
            };
            patterns.push(pattern);
        }
        
        Ok(patterns)
    }
    
    /// Apply harmony rules to syllables
    pub fn apply_harmony_rules(&self, syllables: &mut [LuaSyllable]) -> Result<(), LuaProfileError> {
        let profile_data = self.get_profile_data()?;
        
        // Check if the profile has a harmony rules function
        if let Ok(harmony_function) = profile_data.get::<_, Function>("apply_harmony_rules") {
            // Create a context table for the harmony rules
            let context = self.create_context()?;
            
            // Convert syllables to Lua-compatible format
            let lua_syllables = self.syllables_to_lua_table(syllables)?;
            
            // Call the harmony rules function
            harmony_function.call::<_, ()>((lua_syllables.clone(), context))?;
            
            // Convert back to Rust format
            self.lua_table_to_syllables(lua_syllables, syllables)?;
        }
        
        Ok(())
    }
    
    /// Create a context table for Lua functions
    fn create_context(&self) -> Result<Table, LuaProfileError> {
        let context = self.lua.create_table()?;
        
        // Add common phonemes for easy access
        let phonemes = self.get_phonemes()?;
        for phoneme in phonemes {
            context.set(phoneme.grapheme.clone(), phoneme.ipa.clone())?;
        }
        
        // Add schwa for vowel reduction
        context.set("schwa", "[ə]")?;
        
        Ok(context)
    }
    
    /// Convert Rust syllables to Lua table
    fn syllables_to_lua_table(&self, syllables: &[LuaSyllable]) -> Result<Table, LuaProfileError> {
        let lua_syllables = self.lua.create_table()?;
        
        for (i, syllable) in syllables.iter().enumerate() {
            let lua_syllable = self.lua.create_table()?;
            
            // Convert vectors to Lua tables
            let onset_table = self.lua.create_table()?;
            for (j, phoneme) in syllable.onset.iter().enumerate() {
                onset_table.set(j + 1, phoneme.clone())?;
            }
            
            let nucleus_table = self.lua.create_table()?;
            for (j, phoneme) in syllable.nucleus.iter().enumerate() {
                nucleus_table.set(j + 1, phoneme.clone())?;
            }
            
            let coda_table = self.lua.create_table()?;
            for (j, phoneme) in syllable.coda.iter().enumerate() {
                coda_table.set(j + 1, phoneme.clone())?;
            }
            
            lua_syllable.set("onset", onset_table)?;
            lua_syllable.set("nucleus", nucleus_table)?;
            lua_syllable.set("coda", coda_table)?;
            lua_syllable.set("stressed", syllable.stressed)?;
            lua_syllable.set("position", syllable.position.clone())?;
            
            lua_syllables.set(i + 1, lua_syllable)?;
        }
        
        Ok(lua_syllables)
    }
    
    /// Convert Lua table back to Rust syllables
    fn lua_table_to_syllables(&self, lua_syllables: Table, syllables: &mut [LuaSyllable]) -> Result<(), LuaProfileError> {
        for (i, syllable) in syllables.iter_mut().enumerate() {
            if let Ok(lua_syllable) = lua_syllables.get::<_, Table>(i + 1) {
                // Convert onset
                if let Ok(onset_table) = lua_syllable.get::<_, Table>("onset") {
                    syllable.onset.clear();
                    for entry in onset_table.sequence_values::<String>() {
                        syllable.onset.push(entry?);
                    }
                }
                
                // Convert nucleus
                if let Ok(nucleus_table) = lua_syllable.get::<_, Table>("nucleus") {
                    syllable.nucleus.clear();
                    for entry in nucleus_table.sequence_values::<String>() {
                        syllable.nucleus.push(entry?);
                    }
                }
                
                // Convert coda
                if let Ok(coda_table) = lua_syllable.get::<_, Table>("coda") {
                    syllable.coda.clear();
                    for entry in coda_table.sequence_values::<String>() {
                        syllable.coda.push(entry?);
                    }
                }
                
                // Update other fields
                if let Ok(stressed) = lua_syllable.get::<_, bool>("stressed") {
                    syllable.stressed = stressed;
                }
                if let Ok(position) = lua_syllable.get::<_, String>("position") {
                    syllable.position = position;
                }
            }
        }
        
        Ok(())
    }
    
    /// Validate the profile structure
    pub fn validate(&self) -> Result<(), LuaProfileError> {
        // Check required fields
        self.get_name()?;
        let phonemes = self.get_phonemes()?;
        let patterns = self.get_syllable_patterns()?;
        
        // Validate phonemes
        if phonemes.is_empty() {
            return Err(LuaProfileError::ValidationError("Profile must have at least one phoneme".to_string()));
        }
        
        for phoneme in &phonemes {
            if phoneme.frequency < 0.0 || phoneme.frequency > 1.0 {
                return Err(LuaProfileError::ValidationError(
                    format!("Phoneme frequency must be between 0.0 and 1.0: {}", phoneme.ipa)
                ));
            }
        }
        
        // Validate patterns
        if patterns.is_empty() {
            return Err(LuaProfileError::ValidationError("Profile must have at least one syllable pattern".to_string()));
        }
        
        for pattern in &patterns {
            if pattern.frequency < 0.0 || pattern.frequency > 1.0 {
                return Err(LuaProfileError::ValidationError(
                    format!("Pattern frequency must be between 0.0 and 1.0: {}", pattern.pattern)
                ));
            }
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_lua_profile_creation() {
        // Create a simple Lua profile for testing
        let lua_script = r#"
            profile = {}
            
            profile.name = "Test Language"
            
            profile.phonemes = {
                {ipa = "[p]", phoneme_type = "Consonant", grapheme = "p", frequency = 0.8},
                {ipa = "[a]", phoneme_type = "Vowel", grapheme = "a", frequency = 0.9},
            }
            
            profile.syllable_patterns = {
                {pattern = "CV", frequency = 0.7},
                {pattern = "CVC", frequency = 0.3},
            }
        "#;
        
        // Use absolute path for temp file to avoid issues
        let temp_path = "/tmp/test_lua_profile_creation.lua";
        std::fs::write(temp_path, lua_script).expect("Failed to write test file");
        
        // Load the profile
        let profile = LuaLanguageProfile::load_from_file(temp_path).expect("Failed to load profile");
        
        // Test basic functionality
        assert_eq!(profile.get_name().unwrap(), "Test Language");
        
        let phonemes = profile.get_phonemes().unwrap();
        assert_eq!(phonemes.len(), 2);
        assert_eq!(phonemes[0].ipa, "[p]");
        assert_eq!(phonemes[0].phoneme_type, "Consonant");
        
        let patterns = profile.get_syllable_patterns().unwrap();
        assert_eq!(patterns.len(), 2);
        assert_eq!(patterns[0].pattern, "CV");
        
        // Test validation
        assert!(profile.validate().is_ok());
        
        // Clean up
        let _ = std::fs::remove_file(temp_path);
    }
}