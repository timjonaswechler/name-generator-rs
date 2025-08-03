use crate::syllables::errors::PatternError;
use serde::{Deserialize, Serialize};

// Constants for better readability

const CONSONANT_UPPERCASE: char = 'C';
const CONSONANT_LOWERCASE: char = 'c';
const VOWEL_UPPERCASE: char = 'V';
const VOWEL_LOWERCASE: char = 'v';
const PARSING_DEFAULT_WEIGHT: f32 = 1.0;

// Check if a character represents a consonant (C or c)
fn is_consonant(ch: char) -> bool {
    ch == CONSONANT_UPPERCASE || ch == CONSONANT_LOWERCASE
}

// Check if a character represents a vowel (V or v)
fn is_vowel(ch: char) -> bool {
    ch == VOWEL_UPPERCASE || ch == VOWEL_LOWERCASE
}

// Check if a character is a valid phoneme symbol
fn is_valid_phoneme(ch: char) -> bool {
    is_consonant(ch) || is_vowel(ch)
}

fn weight_in_range(weight: f32) -> bool {
    weight >= 0.0 && weight <= 1.0
}

/// Position within a syllable
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SyllablePosition {
    /// Onset (beginning consonants)
    Onset,
    /// Nucleus (vowels) - required, at least one
    Nucleus,
    /// Coda (ending consonants)
    Coda,
}

/// A syllable component (onset, nucleus, or coda)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SyllableComponent {
    /// Position of this component in the syllable
    pub position: SyllablePosition,
    /// Pattern string (e.g., "C", "cc", "VV")
    pub pattern: String,
    /// Number of phonemes in this component
    pub size: usize,
}

impl SyllableComponent {
    /// Create a new syllable component
    pub fn new(position: SyllablePosition, pattern: String) -> Self {
        let size = pattern.len();
        Self {
            position,
            pattern,
            size,
        }
    }

    /// Check if this component is empty
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    /// Check if this component contains only consonants
    pub fn is_consonants_only(&self) -> bool {
        !self.pattern.is_empty() && self.pattern.chars().all(is_consonant)
    }

    /// Check if this component contains only vowels
    pub fn is_vowels_only(&self) -> bool {
        !self.pattern.is_empty() && self.pattern.chars().all(is_vowel)
    }
}

/// Parsed syllable pattern with onset/nucleus/coda distinction
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SyllablePattern {
    /// Original pattern string
    pub schema: String,
    /// Onset component (optional)
    pub onset: Option<SyllableComponent>,
    /// Nucleus component (required - at least one vowel)
    pub nucleus: SyllableComponent,
    /// Coda component (optional)
    pub coda: Option<SyllableComponent>,
    /// Weighted frequency for this pattern (0.0 to 1.0)
    pub weight: f32,
}

impl SyllablePattern {
    /// Create a new SyllablePattern with schema and weight
    pub fn new(schema: String, weight: f32) -> Result<Self, PatternError> {
        if weight_in_range(weight) == false {
            return Err(PatternError::InvalidWeight(weight));
        }

        let parsed = Self::parse(&schema)?;
        Ok(SyllablePattern {
            schema,
            onset: parsed.onset,
            nucleus: parsed.nucleus,
            coda: parsed.coda,
            weight,
        })
    }

    /// Parse a pattern string into a SyllablePattern
    ///
    /// Supports formats:
    /// - Simple: `CV`, `CVC`, `ccVV` (case insensitive, automatic detection)
    /// - Explicit: `(C)(V)`, `(cc)(VV)(C)` (parentheses specify components)
    pub fn parse(pattern: &str) -> Result<Self, PatternError> {
        if pattern.is_empty() {
            return Err(PatternError::EmptyPattern);
        }

        // Check if pattern uses parentheses notation
        if pattern.contains('(') {
            Self::parse_explicit(pattern)
        } else {
            Self::parse_simple(pattern)
        }
    }

    /// Parse simple pattern without parentheses: CV, CVC, ccVV
    fn parse_simple(pattern: &str) -> Result<Self, PatternError> {
        // Validate all characters are valid phonemes
        for ch in pattern.chars() {
            if !is_valid_phoneme(ch) {
                return Err(PatternError::InvalidCharacter(ch));
            }
        }

        // Find first vowel position (nucleus start)
        let nucleus_start = pattern
            .chars()
            .position(is_vowel)
            .ok_or(PatternError::NoNucleus)?;

        // Find where nucleus ends (last consecutive vowel)
        let mut nucleus_end = nucleus_start;
        for (i, ch) in pattern.chars().enumerate().skip(nucleus_start + 1) {
            if is_vowel(ch) {
                nucleus_end = i;
            } else {
                break;
            }
        }

        // Split into components
        let onset = if nucleus_start > 0 {
            Some(SyllableComponent::new(
                SyllablePosition::Onset,
                pattern[..nucleus_start].to_string(),
            ))
        } else {
            None
        };

        let nucleus = SyllableComponent::new(
            SyllablePosition::Nucleus,
            pattern[nucleus_start..=nucleus_end].to_string(),
        );

        let coda = if nucleus_end + 1 < pattern.len() {
            Some(SyllableComponent::new(
                SyllablePosition::Coda,
                pattern[nucleus_end + 1..].to_string(),
            ))
        } else {
            None
        };

        Ok(SyllablePattern {
            schema: pattern.to_string(),
            onset,
            nucleus,
            coda,
            weight: PARSING_DEFAULT_WEIGHT, // Default weight for parse function
        })
    }

    /// Parse explicit pattern with parentheses: (C)(V)(CC)
    fn parse_explicit(pattern: &str) -> Result<Self, PatternError> {
        let groups = Self::extract_groups(pattern)?;

        if groups.is_empty() {
            return Err(PatternError::EmptyPattern);
        }

        // Validate each group contains only valid phonemes
        for group in &groups {
            for ch in group.chars() {
                if !is_valid_phoneme(ch) {
                    return Err(PatternError::InvalidCharacter(ch));
                }
            }
        }

        // Determine positions based on content
        let mut onset = None;
        let mut nucleus = None;
        let mut coda = None;

        for group in groups {
            if group.chars().all(is_vowel) {
                // Pure vowel group = nucleus
                if nucleus.is_some() {
                    return Err(PatternError::MultipleNucleus);
                }
                nucleus = Some(SyllableComponent::new(SyllablePosition::Nucleus, group));
            } else if group.chars().all(is_consonant) {
                // Pure consonant group
                if nucleus.is_none() {
                    // Before nucleus = onset
                    onset = Some(SyllableComponent::new(SyllablePosition::Onset, group));
                } else {
                    // After nucleus = coda
                    coda = Some(SyllableComponent::new(SyllablePosition::Coda, group));
                }
            } else if group.chars().any(is_vowel) {
                // Mixed group with vowels - treat as nucleus
                if nucleus.is_some() {
                    return Err(PatternError::MultipleNucleus);
                }
                nucleus = Some(SyllableComponent::new(SyllablePosition::Nucleus, group));
            } else {
                // This shouldn't happen due to validation above
                return Err(PatternError::InvalidCharacter('?'));
            }
        }

        let nucleus = nucleus.ok_or(PatternError::NoNucleus)?;

        Ok(SyllablePattern {
            schema: pattern.to_string(),
            onset,
            nucleus,
            coda,
            weight: PARSING_DEFAULT_WEIGHT, // Default weight for parse function
        })
    }

    /// Extract groups from parentheses notation
    fn extract_groups(pattern: &str) -> Result<Vec<String>, PatternError> {
        let mut groups = Vec::new();
        let mut current_group = String::new();
        let mut in_group = false;
        let mut paren_count = 0;

        for ch in pattern.chars() {
            match ch {
                '(' => {
                    if in_group {
                        return Err(PatternError::MismatchedParentheses);
                    }
                    in_group = true;
                    paren_count += 1;
                    current_group.clear();
                }
                ')' => {
                    if !in_group {
                        return Err(PatternError::MismatchedParentheses);
                    }
                    if current_group.is_empty() {
                        return Err(PatternError::EmptyGroup);
                    }
                    groups.push(current_group.clone());
                    current_group.clear();
                    in_group = false;
                    paren_count -= 1;
                }
                _ => {
                    if in_group {
                        current_group.push(ch);
                    } else {
                        // Characters outside parentheses not allowed in explicit mode
                        return Err(PatternError::MismatchedParentheses);
                    }
                }
            }
        }

        if paren_count != 0 {
            return Err(PatternError::MismatchedParentheses);
        }

        Ok(groups)
    }

    /// Get the onset size (0 if no onset)
    pub fn onset_size(&self) -> usize {
        self.onset.as_ref().map(|o| o.size).unwrap_or(0)
    }

    /// Get the nucleus size
    pub fn nucleus_size(&self) -> usize {
        self.nucleus.size
    }

    /// Get the coda size (0 if no coda)
    pub fn coda_size(&self) -> usize {
        self.coda.as_ref().map(|c| c.size).unwrap_or(0)
    }

    /// Get the total pattern size
    pub fn total_size(&self) -> usize {
        self.onset_size() + self.nucleus_size() + self.coda_size()
    }

    /// Check if onset is allowed to be empty
    pub fn allows_empty_onset(&self) -> bool {
        self.onset.is_none()
    }

    /// Check if coda is allowed to be empty
    pub fn allows_empty_coda(&self) -> bool {
        self.coda.is_none()
    }

    /// Generate a standardized pattern string (uppercase)
    pub fn to_standard_pattern(&self) -> String {
        let mut result = String::new();

        if let Some(onset) = &self.onset {
            result.push_str(&onset.pattern.to_uppercase());
        }

        result.push_str(&self.nucleus.pattern.to_uppercase());

        if let Some(coda) = &self.coda {
            result.push_str(&coda.pattern.to_uppercase());
        }

        result
    }
}

impl std::fmt::Display for SyllablePattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.schema)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constants() {
        assert!(is_consonant('C'));
        assert!(is_consonant('c'));
        assert!(is_vowel('V'));
        assert!(is_vowel('v'));
        assert!(!is_consonant('V'));
        assert!(!is_vowel('C'));
        assert!(is_valid_phoneme('C'));
        assert!(is_valid_phoneme('v'));
        assert!(!is_valid_phoneme('X'));
    }

    #[test]
    fn test_new_function() {
        // Valid weight
        let pattern = SyllablePattern::new("CV".to_string(), 0.8).unwrap();
        assert_eq!(pattern.weight, 0.8);
        assert_eq!(pattern.onset_size(), 1);
        assert_eq!(pattern.nucleus_size(), 1);
        assert_eq!(pattern.coda_size(), 0);

        // Invalid weight too high
        assert!(matches!(
            SyllablePattern::new("CV".to_string(), 1.5),
            Err(PatternError::InvalidWeight(1.5))
        ));

        // Invalid weight too low
        assert!(matches!(
            SyllablePattern::new("CV".to_string(), -0.1),
            Err(PatternError::InvalidWeight(-0.1))
        ));
    }

    #[test]
    fn test_parse_simple_patterns() {
        // Basic patterns
        let pattern = SyllablePattern::parse("CV").unwrap();
        assert_eq!(pattern.onset_size(), 1);
        assert_eq!(pattern.nucleus_size(), 1);
        assert_eq!(pattern.coda_size(), 0);
        assert_eq!(pattern.weight, 1.0); // Default weight

        // Case insensitive
        let pattern = SyllablePattern::parse("cv").unwrap();
        assert_eq!(pattern.onset_size(), 1);
        assert_eq!(pattern.nucleus_size(), 1);
        assert_eq!(pattern.coda_size(), 0);

        // Multiple vowels in nucleus
        let pattern = SyllablePattern::parse("CvV").unwrap();
        assert_eq!(pattern.onset_size(), 1);
        assert_eq!(pattern.nucleus_size(), 2);
        assert_eq!(pattern.coda_size(), 0);

        // Just nucleus
        let pattern = SyllablePattern::parse("V").unwrap();
        assert_eq!(pattern.onset_size(), 0);
        assert_eq!(pattern.nucleus_size(), 1);
        assert_eq!(pattern.coda_size(), 0);
    }

    #[test]
    fn test_parse_explicit_patterns() {
        // Basic explicit patterns
        let pattern = SyllablePattern::parse("(C)(V)").unwrap();
        assert_eq!(pattern.onset_size(), 1);
        assert_eq!(pattern.nucleus_size(), 1);
        assert_eq!(pattern.coda_size(), 0);

        // Case insensitive
        let pattern = SyllablePattern::parse("(c)(v)(C)").unwrap();
        assert_eq!(pattern.onset_size(), 1);
        assert_eq!(pattern.nucleus_size(), 1);
        assert_eq!(pattern.coda_size(), 1);

        // Multiple vowels in nucleus
        let pattern = SyllablePattern::parse("(CC)(VV)").unwrap();
        assert_eq!(pattern.onset_size(), 2);
        assert_eq!(pattern.nucleus_size(), 2);
        assert_eq!(pattern.coda_size(), 0);
    }

    #[test]
    fn test_error_cases() {
        // Empty pattern
        assert!(matches!(
            SyllablePattern::parse(""),
            Err(PatternError::EmptyPattern)
        ));

        // Invalid characters
        assert!(matches!(
            SyllablePattern::parse("CXV"),
            Err(PatternError::InvalidCharacter('X'))
        ));

        // No nucleus
        assert!(matches!(
            SyllablePattern::parse("CC"),
            Err(PatternError::NoNucleus)
        ));
    }

    #[test]
    fn test_component_properties() {
        let pattern = SyllablePattern::parse("(CC)(VV)(c)").unwrap();

        let onset = pattern.onset.as_ref().unwrap();
        assert!(onset.is_consonants_only());
        assert!(!onset.is_vowels_only());
        assert_eq!(onset.position, SyllablePosition::Onset);

        let nucleus = &pattern.nucleus;
        assert!(!nucleus.is_consonants_only());
        assert!(nucleus.is_vowels_only());
        assert_eq!(nucleus.position, SyllablePosition::Nucleus);

        let coda = pattern.coda.as_ref().unwrap();
        assert!(coda.is_consonants_only());
        assert!(!coda.is_vowels_only());
        assert_eq!(coda.position, SyllablePosition::Coda);
    }

    #[test]
    fn test_to_standard_pattern() {
        let pattern = SyllablePattern::parse("(cc)(v)(C)").unwrap();
        assert_eq!(pattern.to_standard_pattern(), "CCVC");

        let pattern = SyllablePattern::parse("cvC").unwrap();
        assert_eq!(pattern.to_standard_pattern(), "CVC");
    }
}
