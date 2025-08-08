use crate::{
    phonology::{consonants::all_consonants, phonemes::AllowedCluster, PhonologyConfiguration},
    syllables::{
        errors::utilities::{
            create_phoneme_suggestions, create_phoneme_validation_error,
            validate_clusters_against_list, validate_phonemes_against_list,
        },
        onset::OnsetConfiguration,
    },
    validation::{ValidationError, ValidationErrors},
};

impl OnsetConfiguration {
    pub fn validate(&mut self) -> Result<OnsetConfiguration, ValidationErrors> {
        let mut errors = ValidationErrors::new();

        // checke with is empty
        let allow_empty_check = [
            self.allowed_phonemes.is_empty(),
            self.allowed_clusters.is_empty(),
        ];

        let mask = allow_empty_check
            .iter()
            .enumerate()
            .fold(0u8, |acc, (i, &b)| acc | ((b as u8) << i));

        

        match mask {
            0b11 => {
                errors.add(
                    "empty_onset",
                    ValidationError::new("empty_onset")
                        .with_message("Onset must contain at least one phoneme or cluster"),
                );
            }
            0b01 => {
                if let Err(e) = validate_phonemes_against_list(
                    &self.allowed_phonemes,
                    &all_consonants(),
                    "Konsonant",
                    "unknown_consonant",
                ) {
                    errors.merge(e);
                }

                if self.word_initial_only.is_empty() {
                    if let Err(e) = validate_clusters_against_list(
                        &self.word_initial_only,
                        &all_consonants(),
                        "Konsonant",
                        "unknown_consonant_in_word_initial_only",
                        "word_initial_only",
                        Some(&|cluster_idx, phoneme_idx| {
                            format!(
                                "invalid_word_initial_phoneme_{}_{}",
                                cluster_idx, phoneme_idx
                            )
                        }),
                    ) {
                        errors.merge(e);
                    }
                    for element in &self.allowed_phonemes {
                        let temp = AllowedCluster {
                            phonemes: vec![element.phoneme.clone()],
                            weight: element.weight,
                        };
                        self.word_initial_only.push(temp);
                    }
                }
            }
            0b10 => {
                if let Err(e) = validate_clusters_against_list(
                    &self.allowed_clusters,
                    &all_consonants(),
                    "Konsonant",
                    "unknown_consonant_in_cluster",
                    "Cluster",
                    None,
                ) {
                    errors.merge(e);
                }

                if self.word_initial_only.is_empty() {
                    if let Err(e) = validate_clusters_against_list(
                        &self.word_initial_only,
                        &all_consonants(),
                        "Konsonant",
                        "unknown_consonant_in_word_initial_only",
                        "word_initial_only",
                        Some(&|cluster_idx, phoneme_idx| {
                            format!(
                                "invalid_word_initial_phoneme_{}_{}",
                                cluster_idx, phoneme_idx
                            )
                        }),
                    ) {
                        errors.merge(e);
                    }
                    self.word_initial_only
                        .append(&mut self.allowed_clusters.clone());
                }
            }
            0b00 => {
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
                    None,
                ) {
                    errors.merge(e);
                }

                if self.word_initial_only.is_empty() {
                    if let Err(e) = validate_clusters_against_list(
                        &self.word_initial_only,
                        &all_consonants(),
                        "Konsonant",
                        "unknown_consonant_in_word_initial_only",
                        "word_initial_only",
                        Some(&|cluster_idx, phoneme_idx| {
                            format!(
                                "invalid_word_initial_phoneme_{}_{}",
                                cluster_idx, phoneme_idx
                            )
                        }),
                    ) {
                        errors.merge(e);
                    }

                    self.word_initial_only
                        .append(&mut self.allowed_clusters.clone());

                    for element in &self.allowed_phonemes {
                        let temp = AllowedCluster {
                            phonemes: vec![element.phoneme.clone()],
                            weight: element.weight,
                        };
                        self.word_initial_only.push(temp);
                    }
                }
            }
            _ => unreachable!(),
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
                    create_phoneme_suggestions(&phoneme.phoneme, &available_consonants, 3);
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
                    let suggestions = create_phoneme_suggestions(phoneme, &available_consonants, 3);
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

        // Validate word_initial_only against phonology
        for (cluster_idx, cluster) in self.word_initial_only.iter().enumerate() {
            for (phoneme_idx, phoneme) in cluster.phonemes.iter().enumerate() {
                if !available_consonants.iter().any(|c| c == phoneme) {
                    let suggestions = create_phoneme_suggestions(phoneme, &available_consonants, 3);
                    let error = create_phoneme_validation_error(
                        phoneme,
                        &suggestions,
                        "consonant_in_word_initial_not_in_phonology",
                        "Konsonant",
                        Some(("word_initial_only", cluster_idx, phoneme_idx)),
                    );
                    let error_key = format!(
                        "phonology_missing_word_initial_consonant_{}_{}",
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
