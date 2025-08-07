use crate::validation::{ValidationError, ValidationErrors};

// Utility functions for validation
pub(crate) fn generate_phoneme_suggestions(
    attempted: &str,
    available: &[String],
    max_suggestions: usize,
) -> Vec<String> {
    use crate::phonology::errors::levenshtein_distance;

    let mut suggestions: Vec<(String, usize)> = available
        .iter()
        .map(|phoneme| (phoneme.clone(), levenshtein_distance(attempted, phoneme)))
        .filter(|(_, distance)| *distance <= 3)
        .collect();

    suggestions.sort_by_key(|(_, distance)| *distance);
    suggestions
        .into_iter()
        .take(max_suggestions)
        .map(|(name, _)| name)
        .collect()
}

pub(crate) fn create_phoneme_validation_error(
    attempted_phoneme: &str,
    suggestions: &[String],
    error_prefix: &str,
    phoneme_type: &str,
    context: Option<(&str, usize, usize)>, // (context_name, cluster_idx, phoneme_idx)
) -> ValidationError {
    let base_message = if let Some((context_name, cluster_idx, phoneme_idx)) = context {
        format!(
            "{} '{}' in {} {} an Position {}",
            phoneme_type, attempted_phoneme, context_name, cluster_idx, phoneme_idx
        )
    } else {
        format!("{} '{}'", phoneme_type, attempted_phoneme)
    };

    if suggestions.is_empty() {
        let mut error = ValidationError::new(format!("{}_no_suggestions", error_prefix))
            .add_param("attempted_phoneme", attempted_phoneme)
            .add_param("similarity_threshold", "3")
            .with_message(format!(
                "{}. Keine ähnlichen {} gefunden.",
                base_message,
                if phoneme_type.contains("Vokal") {
                    "Vokale"
                } else {
                    "Konsonanten"
                }
            ));

        if let Some((_, cluster_idx, phoneme_idx)) = context {
            error = error
                .add_param("cluster_index", cluster_idx.to_string())
                .add_param("phoneme_index", phoneme_idx.to_string());
        }
        error
    } else {
        let mut error = ValidationError::new(format!("{}_with_suggestions", error_prefix))
            .add_param("attempted_phoneme", attempted_phoneme)
            .add_param("suggestions", suggestions.join(", "))
            .add_param("suggestion_count", suggestions.len().to_string())
            .add_param("similarity_algorithm", "levenshtein_distance")
            .with_message(format!("{}. Meinten Sie: {:?}", base_message, suggestions));

        if let Some((_, cluster_idx, phoneme_idx)) = context {
            error = error
                .add_param("cluster_index", cluster_idx.to_string())
                .add_param("phoneme_index", phoneme_idx.to_string());
        }
        error
    }
}

// Generic validation utility functions
pub(crate) fn validate_phonemes_against_list(
    phonemes: &[crate::phonology::phonemes::AllowedPhoneme],
    available_phonemes: &[&str],
    phoneme_type: &str, // "Konsonant" or "Vokal"
    error_prefix: &str,
) -> Result<(), ValidationErrors> {
    let mut errors = ValidationErrors::new();

    for phoneme in phonemes {
        if !available_phonemes.iter().any(|&c| c == phoneme.phoneme) {
            let available_strings: Vec<String> =
                available_phonemes.iter().map(|&s| s.to_string()).collect();

            let suggestions = generate_phoneme_suggestions(&phoneme.phoneme, &available_strings, 3);
            let error = create_phoneme_validation_error(
                &phoneme.phoneme,
                &suggestions,
                error_prefix,
                phoneme_type,
                None,
            );
            let error_key = format!("invalid_phoneme_{}", phoneme.phoneme);
            errors.add(error_key, error);
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

pub(crate) fn validate_clusters_against_list(
    clusters: &[crate::phonology::phonemes::AllowedCluster],
    available_phonemes: &[&str],
    phoneme_type: &str, // "Konsonant" or "Vokal"
    error_prefix: &str,
    context_name: &str, // "Cluster", "word_initial_only", etc.
) -> Result<(), ValidationErrors> {
    validate_clusters_against_list_with_error_key(
        clusters,
        available_phonemes,
        phoneme_type,
        error_prefix,
        context_name,
        |cluster_idx, phoneme_idx| {
            format!("invalid_cluster_phoneme_{}_{}", cluster_idx, phoneme_idx)
        },
    )
}

pub(crate) fn validate_clusters_against_list_with_error_key<F>(
    clusters: &[crate::phonology::phonemes::AllowedCluster],
    available_phonemes: &[&str],
    phoneme_type: &str, // "Konsonant" or "Vokal"
    error_prefix: &str,
    context_name: &str, // "Cluster", "word_initial_only", etc.
    error_key_fn: F,
) -> Result<(), ValidationErrors>
where
    F: Fn(usize, usize) -> String,
{
    let mut errors = ValidationErrors::new();

    for (cluster_idx, cluster) in clusters.iter().enumerate() {
        for (phoneme_idx, phoneme) in cluster.phonemes.iter().enumerate() {
            if !available_phonemes.iter().any(|&c| c == phoneme) {
                let available_strings: Vec<String> =
                    available_phonemes.iter().map(|&s| s.to_string()).collect();

                let suggestions = generate_phoneme_suggestions(phoneme, &available_strings, 3);
                let error = create_phoneme_validation_error(
                    phoneme,
                    &suggestions,
                    error_prefix,
                    phoneme_type,
                    Some((context_name, cluster_idx, phoneme_idx)),
                );
                let error_key = error_key_fn(cluster_idx, phoneme_idx);
                errors.add(error_key, error);
            }
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

// Specialized validation utility functions for vowel types
pub(crate) fn validate_diphthongs_against_list(
    diphthongs: &[crate::phonology::phonemes::AllowedDiphthong],
    available_vowels: &[&str],
    phoneme_type: &str, // "Vokal"
    error_prefix: &str,
) -> Result<(), ValidationErrors> {
    let mut errors = ValidationErrors::new();

    for (diphthong_idx, diphthong) in diphthongs.iter().enumerate() {
        // Validate first vowel
        if !available_vowels.iter().any(|&v| v == diphthong.first) {
            let available_strings: Vec<String> =
                available_vowels.iter().map(|&s| s.to_string()).collect();
            let suggestions = generate_phoneme_suggestions(&diphthong.first, &available_strings, 3);
            let error = create_phoneme_validation_error(
                &diphthong.first,
                &suggestions,
                &format!("{}_in_diphthong", error_prefix),
                phoneme_type,
                Some(("allowed_diphthongs", diphthong_idx, 0)),
            );
            let error_key = format!("invalid_diphthong_first_{}_{}", diphthong_idx, 0);
            errors.add(error_key, error);
        }

        // Validate second vowel
        if !available_vowels.iter().any(|&v| v == diphthong.second) {
            let available_strings: Vec<String> =
                available_vowels.iter().map(|&s| s.to_string()).collect();
            let suggestions =
                generate_phoneme_suggestions(&diphthong.second, &available_strings, 3);
            let error = create_phoneme_validation_error(
                &diphthong.second,
                &suggestions,
                &format!("{}_in_diphthong", error_prefix),
                phoneme_type,
                Some(("allowed_diphthongs", diphthong_idx, 1)),
            );
            let error_key = format!("invalid_diphthong_second_{}_{}", diphthong_idx, 1);
            errors.add(error_key, error);
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

pub(crate) fn validate_triphthongs_against_list(
    triphthongs: &[crate::phonology::phonemes::AllowedTriphthong],
    available_vowels: &[&str],
    phoneme_type: &str, // "Vokal"
    error_prefix: &str,
) -> Result<(), ValidationErrors> {
    let mut errors = ValidationErrors::new();

    for (triphthong_idx, triphthong) in triphthongs.iter().enumerate() {
        // Validate first vowel
        if !available_vowels.iter().any(|&v| v == triphthong.first) {
            let available_strings: Vec<String> =
                available_vowels.iter().map(|&s| s.to_string()).collect();
            let suggestions =
                generate_phoneme_suggestions(&triphthong.first, &available_strings, 3);
            let error = create_phoneme_validation_error(
                &triphthong.first,
                &suggestions,
                &format!("{}_in_triphthong", error_prefix),
                phoneme_type,
                Some(("allowed_triphthongs", triphthong_idx, 0)),
            );
            let error_key = format!("invalid_triphthong_first_{}_{}", triphthong_idx, 0);
            errors.add(error_key, error);
        }

        // Validate second vowel
        if !available_vowels.iter().any(|&v| v == triphthong.second) {
            let available_strings: Vec<String> =
                available_vowels.iter().map(|&s| s.to_string()).collect();
            let suggestions =
                generate_phoneme_suggestions(&triphthong.second, &available_strings, 3);
            let error = create_phoneme_validation_error(
                &triphthong.second,
                &suggestions,
                &format!("{}_in_triphthong", error_prefix),
                phoneme_type,
                Some(("allowed_triphthongs", triphthong_idx, 1)),
            );
            let error_key = format!("invalid_triphthong_second_{}_{}", triphthong_idx, 1);
            errors.add(error_key, error);
        }

        // Validate third vowel
        if !available_vowels.iter().any(|&v| v == triphthong.third) {
            let available_strings: Vec<String> =
                available_vowels.iter().map(|&s| s.to_string()).collect();
            let suggestions =
                generate_phoneme_suggestions(&triphthong.third, &available_strings, 3);
            let error = create_phoneme_validation_error(
                &triphthong.third,
                &suggestions,
                &format!("{}_in_triphthong", error_prefix),
                phoneme_type,
                Some(("allowed_triphthongs", triphthong_idx, 2)),
            );
            let error_key = format!("invalid_triphthong_third_{}_{}", triphthong_idx, 2);
            errors.add(error_key, error);
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}
