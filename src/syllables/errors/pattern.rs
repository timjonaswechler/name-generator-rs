use crate::syllables::patterns::{is_valid_phoneme_symbol, is_vowel};
use crate::syllables::SyllablePattern;
use crate::validation::{ValidationError, ValidationErrors};

impl SyllablePattern {
    pub fn validate_pattern(&self) -> Result<(), ValidationErrors> {
        let mut errors = ValidationErrors::new();
        let mut temp_groups: Vec<String> = Vec::new();

        // Initialize components
        let mut pattern = String::new();

        if self.schema.is_empty() {
            errors.add(
                "empty_pattern",
                ValidationError::new("empty_pattern")
                    .with_message("Das eingegeben Pattern ist leer!"),
            );
        }

        if self.schema.contains('(') || self.schema.contains(')') {
            let open_paren_count = self.schema.matches('(').count();
            let close_paren_count = self.schema.matches(')').count();

            if open_paren_count != close_paren_count {
                errors.add(
                    "mismatched_parentheses",
                    ValidationError::new("mismatched_parentheses")
                        .with_message("Mismatched parentheses in pattern"),
                );
            }

            self.schema.split('(').skip(1).for_each(|group| {
                if group.is_empty() || group == ")" {
                    errors.add(
                        "empty_group",
                        ValidationError::new("empty_group")
                            .with_message("Empty group () not allowed"),
                    );
                }
                temp_groups.push(group.split(')').next().unwrap().to_string());
            });

            for group in &temp_groups {
                pattern.push_str(group);
            }
        } else {
            pattern = self.schema.clone();
        }

        if let Some(ch) = pattern.chars().find(|&ch| !is_valid_phoneme_symbol(ch)) {
            errors.add(
                "invalid_phoneme",
                ValidationError::new("invalid_phoneme").with_message(format!(
                    "Pattern contains invalid phonemes symbol: {:?}",
                    ch
                )),
            );
        }

        let nucleus_start = pattern.chars().position(is_vowel);
        if nucleus_start.is_none() {
            errors.add(
                "no_nucleus",
                ValidationError::new("no_nucleus")
                    .with_message("Pattern must contain at least one vowel (V or v)"),
            );
            return Err(errors);
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}
