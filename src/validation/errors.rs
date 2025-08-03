use std::collections::HashMap;
use std::borrow::Cow;
use serde_json::Value;
use indexmap::IndexMap;

#[derive(Debug, Clone)]
pub struct ValidationError {
    /// Error code for programmatic handling
    pub code: Cow<'static, str>,
    /// Human-readable message (optional, can be generated from code + params)
    pub message: Option<Cow<'static, str>>,
    /// Structured parameters for error context
    pub params: HashMap<Cow<'static, str>, Value>,
}

impl ValidationError {
    pub fn new(code: impl Into<Cow<'static, str>>) -> Self {
        Self {
            code: code.into(),
            message: None,
            params: HashMap::new(),
        }
    }

    pub fn add_param(mut self, key: impl Into<Cow<'static, str>>, value: impl Into<Value>) -> Self {
        self.params.insert(key.into(), value.into());
        self
    }

    pub fn with_message(mut self, message: impl Into<Cow<'static, str>>) -> Self {
        self.message = Some(message.into());
        self
    }
}

#[derive(Debug, Clone, Default)]
pub struct ValidationErrors(IndexMap<Cow<'static, str>, ValidationErrorsKind>);

#[derive(Debug, Clone)]
pub enum ValidationErrorsKind {
    /// Errors for the struct itself
    Struct(Vec<ValidationError>),
    /// Errors for a specific field
    Field(ValidationErrors),
}

impl ValidationErrors {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, field: impl Into<Cow<'static, str>>, error: ValidationError) {
        let field = field.into();
        match self.0.get_mut(&field) {
            Some(ValidationErrorsKind::Struct(errors)) => {
                errors.push(error);
            }
            Some(ValidationErrorsKind::Field(_)) => {
                // Convert field errors to struct and add
                if let Some(ValidationErrorsKind::Field(_)) = self.0.shift_remove(&field) {
                    self.0.insert(field, ValidationErrorsKind::Struct(vec![error]));
                }
            }
            None => {
                self.0.insert(field, ValidationErrorsKind::Struct(vec![error]));
            }
        }
    }

    pub fn add_nested(&mut self, field: impl Into<Cow<'static, str>>, errors: ValidationErrors) {
        if !errors.is_empty() {
            self.0.insert(field.into(), ValidationErrorsKind::Field(errors));
        }
    }

    pub fn merge(&mut self, other: ValidationErrors) {
        for (field, kind) in other.0 {
            match self.0.get_mut(&field) {
                Some(existing_kind) => {
                    match (existing_kind, &kind) {
                        (ValidationErrorsKind::Struct(existing), ValidationErrorsKind::Struct(new)) => {
                            existing.extend(new.clone());
                        }
                        (ValidationErrorsKind::Field(existing), ValidationErrorsKind::Field(new)) => {
                            existing.merge(new.clone());
                        }
                        _ => {
                            // Handle mixed types by replacing
                            self.0.insert(field.clone(), kind);
                        }
                    }
                }
                None => {
                    self.0.insert(field, kind);
                }
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn has_error(&self, field: &str) -> bool {
        self.0.contains_key(field)
    }

    pub fn get_field_errors(&self, field: &str) -> Option<&ValidationErrorsKind> {
        self.0.get(field)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&str, &ValidationErrorsKind)> {
        self.0.iter().map(|(k, v)| (k.as_ref(), v))
    }
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(message) = &self.message {
            write!(f, "{}", message)
        } else {
            // Generate message from code and params
            match self.code.as_ref() {
                "unknown_phoneme" => {
                    let phoneme = self.params.get("phoneme").and_then(|v| v.as_str()).unwrap_or("?");
                    let suggestions = self.params.get("suggestions")
                        .and_then(|v| v.as_array())
                        .map(|arr| arr.iter().filter_map(|v| v.as_str()).collect::<Vec<_>>())
                        .unwrap_or_default();
                    
                    if suggestions.is_empty() {
                        write!(f, "Unknown phoneme '{}'", phoneme)
                    } else {
                        write!(f, "Unknown phoneme '{}'. Did you mean: {:?}", phoneme, suggestions)
                    }
                }
                "anatomically_impossible" => {
                    let phoneme = self.params.get("phoneme").and_then(|v| v.as_str()).unwrap_or("?");
                    write!(f, "Phoneme '{}' is anatomically impossible for this speaker", phoneme)
                }
                "invalid_syllable_pattern" => {
                    let pattern = self.params.get("pattern").and_then(|v| v.as_str()).unwrap_or("?");
                    write!(f, "Invalid syllable pattern '{}'", pattern)
                }
                "pattern_uses_undefined_phoneme" => {
                    let pattern = self.params.get("pattern").and_then(|v| v.as_str()).unwrap_or("?");
                    let phoneme = self.params.get("phoneme").and_then(|v| v.as_str()).unwrap_or("?");
                    write!(f, "Syllable pattern '{}' uses undefined phoneme '{}'", pattern, phoneme)
                }
                "empty_phoneme_inventory" => {
                    write!(f, "Phoneme inventory cannot be empty")
                }
                _ => write!(f, "Validation error: {}", self.code)
            }
        }
    }
}

impl std::fmt::Display for ValidationErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_empty() {
            return Ok(());
        }

        for (field, kind) in &self.0 {
            match kind {
                ValidationErrorsKind::Struct(errors) => {
                    for error in errors {
                        writeln!(f, "{}: {}", field, error)?;
                    }
                }
                ValidationErrorsKind::Field(nested_errors) => {
                    writeln!(f, "{}:", field)?;
                    for line in nested_errors.to_string().lines() {
                        writeln!(f, "  {}", line)?;
                    }
                }
            }
        }
        Ok(())
    }
}

impl std::error::Error for ValidationError {}
impl std::error::Error for ValidationErrors {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation_error_creation() {
        let error = ValidationError::new("test_code")
            .add_param("param1", "value1")
            .with_message("Test message");

        assert_eq!(error.code, "test_code");
        assert_eq!(error.message, Some("Test message".into()));
        assert_eq!(error.params.get("param1").unwrap().as_str().unwrap(), "value1");
    }

    #[test]
    fn test_validation_errors_add() {
        let mut errors = ValidationErrors::new();
        errors.add("field1", ValidationError::new("error1"));
        errors.add("field1", ValidationError::new("error2"));

        assert!(!errors.is_empty());
        assert!(errors.has_error("field1"));
        assert_eq!(errors.len(), 1);
    }

    #[test]
    fn test_validation_errors_merge() {
        let mut errors1 = ValidationErrors::new();
        errors1.add("field1", ValidationError::new("error1"));

        let mut errors2 = ValidationErrors::new();
        errors2.add("field2", ValidationError::new("error2"));

        errors1.merge(errors2);
        
        assert!(errors1.has_error("field1"));
        assert!(errors1.has_error("field2"));
        assert_eq!(errors1.len(), 2);
    }
}