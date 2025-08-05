use crate::{syllables::builder::SyllableConfiguration, validation::ValidationError};

impl SyllableConfiguration {
    pub fn empty_pattern(&self) -> ValidationError {
        ValidationError::new("empty_pattern").with_message("Das eingegeben Pattern ist leer!")
    }

    pub fn invalid_pattern_symbol(&self, pattern_symbol: &str) -> ValidationError {
        ValidationError::new("invalid_pattern_symbol").with_message(format!(
            "Das Symbol {0} ist nicht zulässig. Bitte nutze C,c für Konsonanten und V,v für Vokale",
            pattern_symbol
        ))
    }

    fn mismatched_parentheses(&self) -> ValidationError {
        ValidationError::new("mismatched_parentheses")
            .with_message("Mismatched parentheses in pattern")
    }

    fn empty_group(&self) -> ValidationError {
        ValidationError::new("empty_group").with_message("Empty group () not allowed")
    }

    fn no_nucleus(&self) -> ValidationError {
        ValidationError::new("no_nucleus")
            .with_message("Pattern must contain at least one vowel (V or v)")
    }

    fn multiple_nucleus(&self) -> ValidationError {
        ValidationError::new("multiple_nucleus")
            .with_message("Pattern can only have one nucleus group")
    }

    pub fn invalid_weight(&self, weight: f32) -> ValidationError {
        ValidationError::new("invalid_weight")
            .add_param("attempted_weight", weight)
            .with_message(format!(
                "Weight must be between 0.0 and 1.0, got {0}",
                weight
            ))
    }
}
