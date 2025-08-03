use thiserror::Error;

/// Errors that can occur during pattern parsing
#[derive(Error, Debug, Clone, PartialEq)]
pub enum PatternError {
    #[error("Pattern cannot be empty")]
    EmptyPattern,

    #[error("Invalid character '{0}' (only C, c, V, v allowed)")]
    InvalidCharacter(char),

    #[error("Mismatched parentheses in pattern")]
    MismatchedParentheses,

    #[error("Empty group () not allowed")]
    EmptyGroup,

    #[error("Pattern must contain at least one vowel (V or v)")]
    NoNucleus,

    #[error("Pattern can only have one nucleus group")]
    MultipleNucleus,
    
    #[error("Weight must be between 0.0 and 1.0, got {0}")]
    InvalidWeight(f32),
}
