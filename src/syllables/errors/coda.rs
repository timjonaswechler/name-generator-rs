use crate::{
    phonology::{consonants::all_consonants, phonemes::AllowedCluster, PhonologyConfiguration},
    syllables::{
        coda::CodaConfiguration,
        errors::utilities::{
            create_phoneme_validation_error, generate_phoneme_suggestions,
            validate_clusters_against_list, validate_clusters_against_list_with_error_key,
            validate_phonemes_against_list,
        },
    },
    validation::{ValidationError, ValidationErrors},
};

impl CodaConfiguration {
    pub fn validate(&mut self) -> Result<CodaConfiguration, ValidationErrors> {
        let mut errors = ValidationErrors::new();
        // validate allowed phonemes and clusters
        // - empty vector in cluster or phonemes is allowed not both
        // - if word_initial_only is not empty, it must be a subset of allowed_clusters and allowed_phonemes
        // - check if conntent is a valid phoneme string

        // checke with is empty
        let empty_check = [
            self.allowed_phonemes.is_empty(),
            self.allowed_clusters.is_empty(),
            self.word_final_only.is_empty(),
        ];
        match empty_check {
            [true, true, true] | [true, true, false] => {
                errors.add(
                    "empty_onset",
                    ValidationError::new("empty_onset")
                        .with_message("Onset must contain at least one phoneme or cluster"),
                );
            }
            [false, true, true] => {
                if let Err(e) = validate_phonemes_against_list(
                    &self.allowed_phonemes,
                    &all_consonants(),
                    "Konsonant",
                    "unknown_consonant",
                ) {
                    errors.merge(e);
                }
                for element in &self.allowed_phonemes {
                    let temp = AllowedCluster {
                        phonemes: vec![element.phoneme.clone()],
                        weight: element.weight,
                    };
                    self.word_final_only.push(temp);
                }
            }
            [true, false, true] => {
                if let Err(e) = validate_clusters_against_list(
                    &self.allowed_clusters,
                    &all_consonants(),
                    "Konsonant",
                    "unknown_consonant_in_cluster",
                    "Cluster",
                ) {
                    errors.merge(e);
                }
                self.word_final_only
                    .append(&mut self.allowed_clusters.clone());
            }
            [false, false, true] => {
                if let Err(e) = validate_phonemes_against_list(
                    &self.allowed_phonemes,
                    &all_consonants(),
                    "Konsonant",
                    "unknown_consonant",
                ) {
                    errors.merge(e);
                }
                if let Err(e) = validate_clusters_against_list(
                    &self.allowed_clusters,
                    &all_consonants(),
                    "Konsonant",
                    "unknown_consonant_in_cluster",
                    "Cluster",
                ) {
                    errors.merge(e);
                }
                self.word_final_only
                    .append(&mut self.allowed_clusters.clone());
                for element in &self.allowed_phonemes {
                    let temp = AllowedCluster {
                        phonemes: vec![element.phoneme.clone()],
                        weight: element.weight,
                    };
                    self.word_final_only.push(temp);
                }
            }
            [false, false, false] | [false, true, false] | [true, false, false] => {
                // all three vectors are not empty, no error
                if let Err(e) = validate_phonemes_against_list(
                    &self.allowed_phonemes,
                    &all_consonants(),
                    "Konsonant",
                    "unknown_consonant",
                ) {
                    errors.merge(e);
                }
                if let Err(e) = validate_clusters_against_list(
                    &self.allowed_clusters,
                    &all_consonants(),
                    "Konsonant",
                    "unknown_consonant_in_cluster",
                    "Cluster",
                ) {
                    errors.merge(e);
                }
                if let Err(e) = validate_clusters_against_list_with_error_key(
                    &self.word_final_only,
                    &all_consonants(),
                    "Konsonant",
                    "unknown_consonant_in_word_final_only",
                    "word_final_only",
                    |cluster_idx, phoneme_idx| {
                        format!("invalid_word_final_phoneme_{}_{}", cluster_idx, phoneme_idx)
                    },
                ) {
                    errors.merge(e);
                }
            }
        }

        if errors.is_empty() {
            Ok(self.clone())
        } else {
            Err(errors)
        }
    }

    pub fn validate_against_phonology_consonants(
        &self,
        phonology: &PhonologyConfiguration,
    ) -> Result<(), ValidationErrors> {
        let mut errors = ValidationErrors::new();

        // Get available consonants from phonology
        let available_consonants: Vec<String> = phonology
            .consonants
            .iter()
            .map(|c| c.name.to_string())
            .collect();

        // Validate allowed_phonemes against phonology
        for phoneme in &self.allowed_phonemes {
            if !available_consonants.iter().any(|c| c == &phoneme.phoneme) {
                let suggestions =
                    generate_phoneme_suggestions(&phoneme.phoneme, &available_consonants, 3);
                let error = create_phoneme_validation_error(
                    &phoneme.phoneme,
                    &suggestions,
                    "consonant_not_in_phonology",
                    "Konsonant",
                    None,
                );
                let error_key = format!("phonology_missing_consonant_{}", phoneme.phoneme);
                errors.add(error_key, error);
            }
        }

        // Validate allowed_clusters against phonology
        for (cluster_idx, cluster) in self.allowed_clusters.iter().enumerate() {
            for (phoneme_idx, phoneme) in cluster.phonemes.iter().enumerate() {
                if !available_consonants.iter().any(|c| c == phoneme) {
                    let suggestions =
                        generate_phoneme_suggestions(phoneme, &available_consonants, 3);
                    let error = create_phoneme_validation_error(
                        phoneme,
                        &suggestions,
                        "consonant_in_cluster_not_in_phonology",
                        "Konsonant",
                        Some(("Cluster", cluster_idx, phoneme_idx)),
                    );
                    let error_key = format!(
                        "phonology_missing_cluster_consonant_{}_{}",
                        cluster_idx, phoneme_idx
                    );
                    errors.add(error_key, error);
                }
            }
        }

        // Validate word_final_only against phonology
        for (cluster_idx, cluster) in self.word_final_only.iter().enumerate() {
            for (phoneme_idx, phoneme) in cluster.phonemes.iter().enumerate() {
                if !available_consonants.iter().any(|c| c == phoneme) {
                    let suggestions =
                        generate_phoneme_suggestions(phoneme, &available_consonants, 3);
                    let error = create_phoneme_validation_error(
                        phoneme,
                        &suggestions,
                        "consonant_in_word_final_not_in_phonology",
                        "Konsonant",
                        Some(("word_final_only", cluster_idx, phoneme_idx)),
                    );
                    let error_key = format!(
                        "phonology_missing_word_final_consonant_{}_{}",
                        cluster_idx, phoneme_idx
                    );
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
}
