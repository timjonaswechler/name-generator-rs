use colored::*;
use indexmap::IndexMap;
use serde_json::Value;
use std::borrow::Cow;
use std::collections::HashMap;

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
                    self.0
                        .insert(field, ValidationErrorsKind::Struct(vec![error]));
                }
            }
            None => {
                self.0
                    .insert(field, ValidationErrorsKind::Struct(vec![error]));
            }
        }
    }

    pub fn add_nested(&mut self, field: impl Into<Cow<'static, str>>, errors: ValidationErrors) {
        if !errors.is_empty() {
            self.0
                .insert(field.into(), ValidationErrorsKind::Field(errors));
        }
    }

    pub fn merge(&mut self, other: ValidationErrors) {
        for (field, kind) in other.0 {
            match self.0.get_mut(&field) {
                Some(existing_kind) => {
                    match (existing_kind, &kind) {
                        (
                            ValidationErrorsKind::Struct(existing),
                            ValidationErrorsKind::Struct(new),
                        ) => {
                            existing.extend(new.clone());
                        }
                        (
                            ValidationErrorsKind::Field(existing),
                            ValidationErrorsKind::Field(new),
                        ) => {
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

    /// Rekursive Hilfsfunktion zur formatierten Ausgabe der Fehler
    fn fmt_recursive(&self, f: &mut std::fmt::Formatter<'_>, prefix: &str) -> std::fmt::Result {
        // Peekable, um das letzte Element zu erkennen und die Baumstruktur korrekt zu zeichnen
        let mut iter = self.0.iter().peekable();
        while let Some((field, kind)) = iter.next() {
            let is_last = iter.peek().is_none();

            // Baum-Zeichen: '├─' für Elemente in der Mitte, '└─' für das letzte Element
            let branch = if is_last {
                "└─".cyan()
            } else {
                "├─".cyan()
            };
            // Präfix für die nächste Ebene: '│  ' wenn es weitergeht, '   ' wenn dies der letzte Zweig war
            let new_prefix = if is_last {
                format!("{}   ", prefix)
            } else {
                format!("{}{}  ", prefix, "│".cyan())
            };

            // Feldnamen in Gelb und Fett hervorheben
            writeln!(f, "{}{} {}", prefix, branch, field.yellow().bold())?;

            match kind {
                ValidationErrorsKind::Struct(errors) => {
                    let mut error_iter = errors.iter().peekable();
                    while let Some(error) = error_iter.next() {
                        let is_last_error = error_iter.peek().is_none();
                        let error_branch = if is_last_error {
                            "└─".red()
                        } else {
                            "├─".red()
                        };
                        // Die eigentliche Fehlermeldung ausgeben, etwas eingerückt
                        writeln!(f, "{}{} {}", new_prefix, error_branch, error)?;
                    }
                }
                ValidationErrorsKind::Field(nested_errors) => {
                    // Rekursiver Aufruf für verschachtelte Fehler
                    nested_errors.fmt_recursive(f, &new_prefix)?;
                }
            }
        }
        Ok(())
    }
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(message) = &self.message {
            write!(f, "{}", message)
        } else {
            // Fallback for errors without explicit messages
            write!(f, "Validation error: {}", self.code)
        }
    }
}

impl std::fmt::Display for ValidationErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_empty() {
            return Ok(());
        }
        // --- Header ---
        writeln!(
            f,
            "\n{}",
            "══════════════ Validation Errors ══════════════"
                .red()
                .bold()
        )?;
        writeln!(f, "{}", "There are some errors:".bright_black())?;
        writeln!(f, "")?; // Leere Zeile für Abstand

        // --- Fehlerbaum ---
        self.fmt_recursive(f, "")?;
        // --- Footer ---
        writeln!(
            f,
            "{}",
            "══════════════════════════════════════════════════"
                .red()
                .bold()
        )
        // Die schöne Baumansicht oben enthält bereits alle Informationen
    }
}

impl std::error::Error for ValidationError {}
impl std::error::Error for ValidationErrors {}

/// Wrapper for ValidationErrors that uses Display formatting in Debug contexts
/// This ensures that even when used with ? operator or panic, we get nice formatting
#[derive(Clone)]
pub struct FormattedValidationErrors(pub ValidationErrors);

impl std::fmt::Debug for FormattedValidationErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Use Display formatting instead of Debug
        std::fmt::Display::fmt(&self.0, f)
    }
}

impl std::fmt::Display for FormattedValidationErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

impl std::error::Error for FormattedValidationErrors {}

impl From<ValidationErrors> for FormattedValidationErrors {
    fn from(errors: ValidationErrors) -> Self {
        FormattedValidationErrors(errors)
    }
}

impl From<FormattedValidationErrors> for ValidationErrors {
    fn from(formatted: FormattedValidationErrors) -> Self {
        formatted.0
    }
}

impl From<Box<dyn std::error::Error>> for FormattedValidationErrors {
    fn from(error: Box<dyn std::error::Error>) -> Self {
        // Try to downcast to ValidationErrors first
        match error.downcast::<ValidationErrors>() {
            Ok(validation_errors) => FormattedValidationErrors(*validation_errors),
            Err(original_error) => {
                // Create a new ValidationErrors with the generic error
                let mut errors = ValidationErrors::new();
                errors.add(
                    "general",
                    ValidationError::new("generic_error").with_message(original_error.to_string()),
                );
                FormattedValidationErrors(errors)
            }
        }
    }
}

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
        assert_eq!(
            error.params.get("param1").unwrap().as_str().unwrap(),
            "value1"
        );
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
