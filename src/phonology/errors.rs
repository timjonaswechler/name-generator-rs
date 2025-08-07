use crate::phonology::PhonologyConfiguration;
use crate::validation::ValidationError;

/// Phoneme-specific ValidationError implementations
impl PhonologyConfiguration {
    /// Creates error for unknown consonant with suggestions
    fn unknown_consonant(&self, name: &str, suggestions: Vec<String>) -> ValidationError {
        ValidationError::new("unknown_consonant")
            .add_param("attempted_name", name)
            .add_param("suggestions", suggestions.join(", "))
            .add_param("suggestion_count", suggestions.len().to_string())
            .add_param("similarity_algorithm", "levenshtein_distance")
            .with_message(format!(
                "Unbekannter Konsonant '{}'. Meinten Sie: {:?}",
                name, suggestions
            ))
    }
    fn unknown_consonant_from_phonology(
        &self,
        name: &str,
        suggestions: Vec<String>,
    ) -> ValidationError {
        ValidationError::new("unknown_consonant_from_phonology")
            .add_param("attemted_name", name)
            .add_param("suggestions", suggestions.join(", "))
            .add_param("suggestion_count", suggestions.len().to_string())
            .add_param("similarity_algorithm", "levenshtein_distance")
            .with_message(format!(
                "Der Konsonant '{}' ist nicht in der Phonology der Sprache aufgeführt. Meinten Sie: {:?}",
                name, suggestions
            ))
    }

    /// Creates error for unknown vowel with suggestions
    fn unknown_vowel(&self, name: &str, suggestions: Vec<String>) -> ValidationError {
        ValidationError::new("unknown_vowel")
            .add_param("attempted_name", name)
            .add_param("suggestions", suggestions.join(", "))
            .add_param("suggestion_count", suggestions.len().to_string())
            .add_param("similarity_algorithm", "levenshtein_distance")
            .with_message(format!(
                "Unbekannter Vokal '{}'. Meinten Sie: {:?}",
                name, suggestions
            ))
    }

    /// Creates error for invalid diacritic
    fn invalid_diacritic(
        &self,
        base: &str,
        diacritic: &str,
        available: Vec<String>,
    ) -> ValidationError {
        ValidationError::new("invalid_diacritic")
            .add_param("base_phoneme", base)
            .add_param("attempted_diacritic", diacritic)
            .add_param("available_diacritics", available.join(", "))
            .add_param("available_count", available.len().to_string())
            .with_message(format!(
                "Ungültiges Diakritikum '{}' für '{}'. Verfügbar: {:?}",
                diacritic, base, available
            ))
    }

    /// Creates error for invalid phoneme configuration
    fn invalid_phoneme_configuration(&self, reason: &str) -> ValidationError {
        ValidationError::new("invalid_phoneme_configuration")
            .add_param("reason", reason)
            .add_param("configuration_type", "phoneme")
            .with_message(format!("Ungültige Phonem-Konfiguration: {}", reason))
    }

    /// Creates error for phoneme not found
    fn phoneme_not_found(&self, phoneme: &str) -> ValidationError {
        ValidationError::new("phoneme_not_found")
            .add_param("phoneme", phoneme)
            .add_param("registry_status", "active")
            .with_message(format!("Phonem nicht gefunden: {}", phoneme))
    }

    /// Creates error for duplicate phoneme
    fn phoneme_already_exists(&self, phoneme: &str) -> ValidationError {
        ValidationError::new("phoneme_already_exists")
            .add_param("phoneme", phoneme)
            .add_param("operation", "insert")
            .with_message(format!("Phonem existiert bereits: {}", phoneme))
    }

    /// Creates error for invalid phoneme category
    fn invalid_phoneme_category(&self, category: &str) -> ValidationError {
        ValidationError::new("invalid_phoneme_category")
            .add_param("attempted_category", category)
            .add_param("valid_categories", "consonant, vowel, diacritic")
            .with_message(format!("Ungültige Phonem-Kategorie: {}", category))
    }

    /// Creates error for empty phoneme registry
    fn empty_phoneme_registry(&self) -> ValidationError {
        ValidationError::new("empty_phoneme_registry")
            .add_param("registry_state", "empty")
            .add_param("required_phonemes", "at_least_one")
            .with_message("Phonem-Registry ist leer")
    }

    /// Creates error for JSON parsing issues
    fn phoneme_json_parse_error(&self, details: &str) -> ValidationError {
        ValidationError::new("phoneme_json_parse_error")
            .add_param("parse_error", details)
            .add_param("format", "json")
            .with_message(format!("JSON-Parse-Fehler: {}", details))
    }

    /// Creates error for Lua parsing issues
    fn phoneme_lua_parse_error(&self, details: &str) -> ValidationError {
        ValidationError::new("phoneme_lua_parse_error")
            .add_param("parse_error", details)
            .add_param("format", "lua")
            .with_message(format!("Lua-Parse-Fehler: {}", details))
    }

    /// Creates error for format detection failures
    fn phoneme_format_detection_error(&self, details: &str) -> ValidationError {
        ValidationError::new("phoneme_format_detection_error")
            .add_param("detection_error", details)
            .add_param("supported_formats", "json, lua, yaml")
            .with_message(format!("Format-Erkennung fehlgeschlagen: {}", details))
    }

    /// Creates error for invalid phoneme names
    fn invalid_phoneme_name(&self, name: &str) -> ValidationError {
        ValidationError::new("invalid_phoneme_name")
            .add_param("attempted_name", name)
            .add_param("naming_rules", "ipa_compliant")
            .with_message(format!("Ungültiger Phonem-Name: {}", name))
    }

    /// Creates error for duplicate vowels
    pub fn duplicate_vowel(&self, vowel: &str) -> ValidationError {
        ValidationError::new("duplicate_vowel")
            .add_param("vowel", vowel)
            .add_param("phoneme_type", "vowel")
            .with_message(format!("Doppelter Vokal: {}", vowel))
    }

    /// Creates error for duplicate consonants
    pub fn duplicate_consonant(&self, consonant: &str) -> ValidationError {
        ValidationError::new("duplicate_consonant")
            .add_param("consonant", consonant)
            .add_param("phoneme_type", "consonant")
            .with_message(format!("Doppelter Konsonant: {}", consonant))
    }

    /// Suggests anatomically possible alternatives for a given phoneme
    fn suggest_anatomically_possible_phonemes(
        &self,
        attempted: &str,
        all_phonemes: &[&str],
    ) -> Vec<String> {
        all_phonemes
            .iter()
            .map(|&phoneme| {
                (
                    phoneme.to_string(),
                    levenshtein_distance(attempted, phoneme),
                )
            })
            .filter(|(_, distance)| *distance <= 3) // Similarity threshold
            .collect::<Vec<_>>()
            .into_iter()
            .min_by_key(|(_, distance)| *distance)
            .map(|(phoneme, _)| vec![phoneme])
            .unwrap_or_else(Vec::new)
    }

    /// Finds best phoneme suggestions based on Levenshtein distance
    fn find_best_phoneme_suggestions(
        &self,
        attempted: &str,
        candidates: &[&str],
        max_suggestions: usize,
    ) -> Vec<String> {
        let mut suggestions: Vec<_> = candidates
            .iter()
            .map(|&candidate| {
                (
                    candidate.to_string(),
                    levenshtein_distance(attempted, candidate),
                )
            })
            .filter(|(_, distance)| *distance <= 3) // Only reasonably similar
            .collect();

        // Sort by distance (best matches first)
        suggestions.sort_by_key(|(_, distance)| *distance);

        // Take only the best matches up to max_suggestions
        suggestions
            .into_iter()
            .take(max_suggestions)
            .map(|(phoneme, _)| phoneme)
            .collect()
    }

    /// Creates error with intelligent phoneme suggestions
    fn unknown_phoneme_with_smart_suggestions(
        &self,
        attempted: &str,
        available_consonants: &[&str],
        available_vowels: &[&str],
    ) -> ValidationError {
        let consonant_suggestions =
            self.find_best_phoneme_suggestions(attempted, available_consonants, 3);
        let vowel_suggestions = self.find_best_phoneme_suggestions(attempted, available_vowels, 3);

        let mut all_suggestions = consonant_suggestions;
        all_suggestions.extend(vowel_suggestions);

        // Remove duplicates and limit to 5 total suggestions
        all_suggestions.sort();
        all_suggestions.dedup();
        all_suggestions.truncate(5);

        if all_suggestions.is_empty() {
            ValidationError::new("unknown_phoneme_no_suggestions")
                .add_param("attempted_phoneme", attempted)
                .add_param("similarity_threshold", "3")
                .add_param(
                    "available_consonants",
                    available_consonants.len().to_string(),
                )
                .add_param("available_vowels", available_vowels.len().to_string())
                .with_message(format!(
                    "Unbekanntes Phonem '{}'. Keine ähnlichen Phoneme gefunden.",
                    attempted
                ))
        } else {
            ValidationError::new("unknown_phoneme_with_suggestions")
                .add_param("attempted_phoneme", attempted)
                .add_param("suggestions", all_suggestions.join(", "))
                .add_param("suggestion_count", all_suggestions.len().to_string())
                .add_param("similarity_algorithm", "levenshtein_distances")
                .with_message(format!(
                    "Unbekanntes Phonem '{}'. Meinten Sie: {:?}",
                    attempted, all_suggestions
                ))
        }
    }
}

/// Calculate Levenshtein distance between two strings
pub fn levenshtein_distance(s1: &str, s2: &str) -> usize {
    let len1 = s1.len();
    let len2 = s2.len();

    if len1 == 0 {
        return len2;
    }
    if len2 == 0 {
        return len1;
    }

    let mut matrix = vec![vec![0; len2 + 1]; len1 + 1];

    // Initialize first row and column
    for i in 0..=len1 {
        matrix[i][0] = i;
    }
    for j in 0..=len2 {
        matrix[0][j] = j;
    }

    let chars1: Vec<char> = s1.chars().collect();
    let chars2: Vec<char> = s2.chars().collect();

    for i in 1..=len1 {
        for j in 1..=len2 {
            let cost = if chars1[i - 1] == chars2[j - 1] { 0 } else { 1 };
            matrix[i][j] = (matrix[i - 1][j] + 1)
                .min(matrix[i][j - 1] + 1)
                .min(matrix[i - 1][j - 1] + cost);
        }
    }

    matrix[len1][len2]
}
