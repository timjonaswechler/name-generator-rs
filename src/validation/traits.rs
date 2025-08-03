use crate::validation::errors::ValidationErrors;

/// Core validation trait for simple validation
pub trait Validate {
    fn validate(&self) -> Result<(), ValidationErrors>;
}

/// Context-aware validation trait for validations that require external context
pub trait ValidateWithContext<C> {
    fn validate_with_context(&self, context: &C) -> Result<(), ValidationErrors>;
}

/// Domain-specific validation traits

/// Phoneme validation trait
pub trait ValidatePhoneme {
    fn validate_phoneme(&self, phoneme: &str) -> Result<(), ValidationErrors>;
}

/// Anatomical constraints validation trait  
pub trait ValidateAnatomicalConstraints {
    fn validate_anatomical_constraints(&self, anatomy: &crate::anatomy::speaker::SpeakerAnatomy) -> Result<(), ValidationErrors>;
}

/// Language profile validation trait
pub trait ValidateLanguageProfile {
    fn validate_completeness(&self) -> Result<(), ValidationErrors>;
    fn validate_linguistic_consistency(&self) -> Result<(), ValidationErrors>;
}

/// Syllable pattern validation trait
pub trait ValidateSyllablePattern {
    fn validate_pattern(&self, pattern: &str) -> Result<(), ValidationErrors>;
    fn validate_pattern_complexity(&self, pattern: &str, max_complexity: usize) -> Result<(), ValidationErrors>;
}

/// Phonology configuration validation trait
pub trait ValidatePhonology {
    fn validate_phonology(&self) -> Result<(), ValidationErrors>;
}

/// Helper trait for providing validation suggestions
pub trait ValidationSuggestions {
    fn suggest_similar(&self, input: &str, max_suggestions: usize) -> Vec<String>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::validation::errors::ValidationError;

    struct TestValidatable {
        is_valid: bool,
    }

    impl Validate for TestValidatable {
        fn validate(&self) -> Result<(), ValidationErrors> {
            if self.is_valid {
                Ok(())
            } else {
                let mut errors = ValidationErrors::new();
                errors.add("test_field", ValidationError::new("test_error"));
                Err(errors)
            }
        }
    }

    #[test]
    fn test_validate_trait() {
        let valid_item = TestValidatable { is_valid: true };
        assert!(valid_item.validate().is_ok());

        let invalid_item = TestValidatable { is_valid: false };
        let result = invalid_item.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().has_error("test_field"));
    }

    struct TestContextValidatable {
        value: i32,
    }

    struct TestContext {
        max_value: i32,
    }

    impl ValidateWithContext<TestContext> for TestContextValidatable {
        fn validate_with_context(&self, context: &TestContext) -> Result<(), ValidationErrors> {
            if self.value <= context.max_value {
                Ok(())
            } else {
                let mut errors = ValidationErrors::new();
                errors.add("value", ValidationError::new("value_too_high")
                    .add_param("value", self.value)
                    .add_param("max_value", context.max_value));
                Err(errors)
            }
        }
    }

    #[test]
    fn test_validate_with_context_trait() {
        let item = TestContextValidatable { value: 5 };
        let context = TestContext { max_value: 10 };
        
        assert!(item.validate_with_context(&context).is_ok());

        let context_strict = TestContext { max_value: 3 };
        let result = item.validate_with_context(&context_strict);
        assert!(result.is_err());
        assert!(result.unwrap_err().has_error("value"));
    }
}