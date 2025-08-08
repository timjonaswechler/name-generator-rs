use crate::{
    phonology::{
        all_vowels, consonants::all_consonants, phonemes::AllowedCluster, PhonologyConfiguration,
    },
    syllables::{
        errors::utilities::{
            create_phoneme_suggestions, create_phoneme_validation_error,
            validate_clusters_against_list, validate_diphthongs_against_list,
            validate_phonemes_against_list, validate_triphthongs_against_list,
        },
        nucleus::NucleusConfiguration,
    },
    validation::{ValidationError, ValidationErrors},
};

impl NucleusConfiguration {
    pub fn validate(&mut self) -> Result<NucleusConfiguration, ValidationErrors> {
        let mut errors = ValidationErrors::new();

        let allow_empty_check = [
            self.allowed_phonemes.is_empty(),
            self.allowed_diphthongs.is_empty(),
            self.allowed_triphthongs.is_empty(),
        ];

        let mask = allow_empty_check
            .iter()
            .enumerate()
            .fold(0u8, |acc, (i, &b)| acc | ((b as u8) << i));
        
        let word_empty_check = [
            self.word_initial_only.is_empty(),
            self.word_final_only.is_empty(),
        ];
        let word_mask = word_empty_check
            .iter()
            .enumerate()
            .fold(0u8, |acc, (i, &b)| acc | ((b as u8) << i));

        match mask {
            // empty_nucleus_check
            0b111 => {
                errors.add(
                    "empty_nucleus",
                    ValidationError::new("empty_nucleus")
                        .with_message("Nucleus must contain at least one phoneme or cluster"),
                );
            }
            // allowed_phonemes_only
            0b011 => {
                if let Err(e) = validate_phonemes_against_list(
                    &self.allowed_phonemes,
                    &all_vowels(),
                    "Vokal",
                    "unknown_vowel",
                ) {
                    errors.merge(e);
                }

                if let Err(e) = self.validate_word_mask(word_mask) {
                    errors.merge(e);
                }

                match word_mask {
                    0b11 => {
                        // Da wir die gleichen Daten zu zwei Listen hinzufügen,
                        // erstellen wir sie einmal und klonen sie dann.
                        let clusters: Vec<AllowedCluster> = self
                            .allowed_phonemes
                            .iter()
                            .map(|element| AllowedCluster {
                                phonemes: vec![element.phoneme.clone()],
                                weight: element.weight,
                            })
                            .collect();

                        self.word_initial_only.extend(clusters.clone());
                        self.word_final_only.extend(clusters);
                    }

                    0b01 => {
                        self.word_final_only
                            .extend(self.allowed_phonemes.iter().map(|element| AllowedCluster {
                                phonemes: vec![element.phoneme.clone()],
                                weight: element.weight,
                            }));
                    }

                    0b10 => {
                        self.word_initial_only
                            .extend(self.allowed_phonemes.iter().map(|element| AllowedCluster {
                                phonemes: vec![element.phoneme.clone()],
                                weight: element.weight,
                            }));
                    }

                    _ => unreachable!(),
                }
            }
            // allowed_diphthongs_only
            0b101 => {
                if let Err(e) = validate_diphthongs_against_list(
                    &self.allowed_diphthongs,
                    &all_vowels(),
                    "Diphthong",
                    "unknown_diphthong",
                ) {
                    errors.merge(e);
                }

                if let Err(e) = self.validate_word_mask(word_mask) {
                    errors.merge(e);
                }

                match word_mask {
                    0b11 => {
                        let clusters: Vec<AllowedCluster> = self
                            .allowed_diphthongs
                            .iter()
                            .map(|element| AllowedCluster {
                                phonemes: vec![element.first.clone(), element.second.clone()],
                                weight: element.weight,
                            })
                            .collect();

                        self.word_initial_only.extend(clusters.clone());
                        self.word_final_only.extend(clusters);
                    }

                    0b01 => {
                        self.word_final_only
                            .extend(
                                self.allowed_diphthongs
                                    .iter()
                                    .map(|element| AllowedCluster {
                                        phonemes: vec![
                                            element.first.clone(),
                                            element.second.clone(),
                                        ],
                                        weight: element.weight,
                                    }),
                            );
                    }

                    0b10 => {
                        self.word_initial_only
                            .extend(
                                self.allowed_diphthongs
                                    .iter()
                                    .map(|element| AllowedCluster {
                                        phonemes: vec![
                                            element.first.clone(),
                                            element.second.clone(),
                                        ],
                                        weight: element.weight,
                                    }),
                            );
                    }

                    _ => unreachable!(),
                }
            }
            // allowed_triphthongs_only
            0b110 => {
                if let Err(e) = validate_triphthongs_against_list(
                    &self.allowed_triphthongs,
                    &all_vowels(),
                    "Triphthongs",
                    "unknown_triphthong",
                ) {
                    errors.merge(e);
                }

                if let Err(e) = self.validate_word_mask(word_mask) {
                    errors.merge(e);
                }

                match word_mask {
                    0b11 => {
                        let clusters: Vec<AllowedCluster> = self
                            .allowed_triphthongs
                            .iter()
                            .map(|element| AllowedCluster {
                                phonemes: vec![
                                    element.first.clone(),
                                    element.second.clone(),
                                    element.third.clone(),
                                ],
                                weight: element.weight,
                            })
                            .collect();

                        self.word_initial_only.extend(clusters.clone());
                        self.word_final_only.extend(clusters);
                    }

                    0b01 => {
                        self.word_final_only
                            .extend(self.allowed_triphthongs.iter().map(|element| {
                                AllowedCluster {
                                    phonemes: vec![
                                        element.first.clone(),
                                        element.second.clone(),
                                        element.third.clone(),
                                    ],
                                    weight: element.weight,
                                }
                            }));
                    }

                    0b10 => {
                        self.word_initial_only
                            .extend(self.allowed_triphthongs.iter().map(|element| {
                                AllowedCluster {
                                    phonemes: vec![
                                        element.first.clone(),
                                        element.second.clone(),
                                        element.third.clone(),
                                    ],
                                    weight: element.weight,
                                }
                            }));
                    }

                    _ => unreachable!(),
                }
            }
            // allowed_phonemes_and_diphthongs
            0b001 => {
                if let Err(e) = validate_phonemes_against_list(
                    &self.allowed_phonemes,
                    &all_vowels(),
                    "Vokal",
                    "unknown_vowel",
                ) {
                    errors.merge(e);
                }
                if let Err(e) = validate_diphthongs_against_list(
                    &self.allowed_diphthongs,
                    &all_vowels(),
                    "Diphthong",
                    "unknown_diphthong",
                ) {
                    errors.merge(e);
                }

                if let Err(e) = self.validate_word_mask(word_mask) {
                    errors.merge(e);
                }

                match word_mask {
                    0b11 => {
                        let clusters: Vec<AllowedCluster> = self
                            .allowed_phonemes
                            .iter()
                            .map(|element| AllowedCluster {
                                phonemes: vec![element.phoneme.clone()],
                                weight: element.weight,
                            })
                            .collect();

                        self.word_initial_only.extend(clusters.clone());
                        self.word_final_only.extend(clusters);

                        let clusters: Vec<AllowedCluster> = self
                            .allowed_diphthongs
                            .iter()
                            .map(|element| AllowedCluster {
                                phonemes: vec![element.first.clone(), element.second.clone()],
                                weight: element.weight,
                            })
                            .collect();

                        self.word_initial_only.extend(clusters.clone());
                        self.word_final_only.extend(clusters);
                    }

                    0b01 => {
                        self.word_final_only
                            .extend(self.allowed_phonemes.iter().map(|element| AllowedCluster {
                                phonemes: vec![element.phoneme.clone()],
                                weight: element.weight,
                            }));
                        self.word_final_only
                            .extend(
                                self.allowed_diphthongs
                                    .iter()
                                    .map(|element| AllowedCluster {
                                        phonemes: vec![
                                            element.first.clone(),
                                            element.second.clone(),
                                        ],
                                        weight: element.weight,
                                    }),
                            );
                    }

                    0b10 => {
                        self.word_initial_only
                            .extend(self.allowed_phonemes.iter().map(|element| AllowedCluster {
                                phonemes: vec![element.phoneme.clone()],
                                weight: element.weight,
                            }));
                        self.word_initial_only
                            .extend(
                                self.allowed_diphthongs
                                    .iter()
                                    .map(|element| AllowedCluster {
                                        phonemes: vec![
                                            element.first.clone(),
                                            element.second.clone(),
                                        ],
                                        weight: element.weight,
                                    }),
                            );
                    }

                    _ => unreachable!(),
                }
            }
            // allowed_phonemes_and_triphthongs
            0b010 => {
                if let Err(e) = validate_phonemes_against_list(
                    &self.allowed_phonemes,
                    &all_vowels(),
                    "Vokal",
                    "unknown_vowel",
                ) {
                    errors.merge(e);
                }
                if let Err(e) = validate_triphthongs_against_list(
                    &self.allowed_triphthongs,
                    &all_vowels(),
                    "Triphthongs",
                    "unknown_triphthong",
                ) {
                    errors.merge(e);
                }

                if let Err(e) = self.validate_word_mask(word_mask) {
                    errors.merge(e);
                }

                match word_mask {
                    0b11 => {
                        let clusters: Vec<AllowedCluster> = self
                            .allowed_phonemes
                            .iter()
                            .map(|element| AllowedCluster {
                                phonemes: vec![element.phoneme.clone()],
                                weight: element.weight,
                            })
                            .collect();

                        self.word_initial_only.extend(clusters.clone());
                        self.word_final_only.extend(clusters);

                        let clusters: Vec<AllowedCluster> = self
                            .allowed_triphthongs
                            .iter()
                            .map(|element| AllowedCluster {
                                phonemes: vec![
                                    element.first.clone(),
                                    element.second.clone(),
                                    element.third.clone(),
                                ],
                                weight: element.weight,
                            })
                            .collect();

                        self.word_initial_only.extend(clusters.clone());
                        self.word_final_only.extend(clusters);
                    }

                    0b01 => {
                        self.word_final_only
                            .extend(self.allowed_phonemes.iter().map(|element| AllowedCluster {
                                phonemes: vec![element.phoneme.clone()],
                                weight: element.weight,
                            }));
                        self.word_final_only
                            .extend(self.allowed_triphthongs.iter().map(|element| {
                                AllowedCluster {
                                    phonemes: vec![
                                        element.first.clone(),
                                        element.second.clone(),
                                        element.third.clone(),
                                    ],
                                    weight: element.weight,
                                }
                            }));
                    }

                    0b10 => {
                        self.word_initial_only
                            .extend(self.allowed_phonemes.iter().map(|element| AllowedCluster {
                                phonemes: vec![element.phoneme.clone()],
                                weight: element.weight,
                            }));
                        self.word_initial_only
                            .extend(self.allowed_triphthongs.iter().map(|element| {
                                AllowedCluster {
                                    phonemes: vec![
                                        element.first.clone(),
                                        element.second.clone(),
                                        element.third.clone(),
                                    ],
                                    weight: element.weight,
                                }
                            }));
                    }

                    _ => unreachable!(),
                }
            }
            // allowed_diphthongs_and_triphthongs
            0b100 => {
                if let Err(e) = validate_diphthongs_against_list(
                    &self.allowed_diphthongs,
                    &all_vowels(),
                    "Diphthong",
                    "unknown_diphthong",
                ) {
                    errors.merge(e);
                }
                if let Err(e) = validate_triphthongs_against_list(
                    &self.allowed_triphthongs,
                    &all_vowels(),
                    "Triphthongs",
                    "unknown_triphthong",
                ) {
                    errors.merge(e);
                }
                if let Err(e) = self.validate_word_mask(word_mask) {
                    errors.merge(e);
                }
                match word_mask {
                    0b11 => {
                        let clusters: Vec<AllowedCluster> = self
                            .allowed_diphthongs
                            .iter()
                            .map(|element| AllowedCluster {
                                phonemes: vec![element.first.clone(), element.second.clone()],
                                weight: element.weight,
                            })
                            .collect();

                        self.word_initial_only.extend(clusters.clone());
                        self.word_final_only.extend(clusters);

                        let clusters: Vec<AllowedCluster> = self
                            .allowed_triphthongs
                            .iter()
                            .map(|element| AllowedCluster {
                                phonemes: vec![
                                    element.first.clone(),
                                    element.second.clone(),
                                    element.third.clone(),
                                ],
                                weight: element.weight,
                            })
                            .collect();

                        self.word_initial_only.extend(clusters.clone());
                        self.word_final_only.extend(clusters);
                    }

                    0b01 => {
                        self.word_final_only
                            .extend(
                                self.allowed_diphthongs
                                    .iter()
                                    .map(|element| AllowedCluster {
                                        phonemes: vec![
                                            element.first.clone(),
                                            element.second.clone(),
                                        ],
                                        weight: element.weight,
                                    }),
                            );
                        self.word_final_only
                            .extend(self.allowed_triphthongs.iter().map(|element| {
                                AllowedCluster {
                                    phonemes: vec![
                                        element.first.clone(),
                                        element.second.clone(),
                                        element.third.clone(),
                                    ],
                                    weight: element.weight,
                                }
                            }));
                    }

                    0b10 => {
                        self.word_initial_only
                            .extend(
                                self.allowed_diphthongs
                                    .iter()
                                    .map(|element| AllowedCluster {
                                        phonemes: vec![
                                            element.first.clone(),
                                            element.second.clone(),
                                        ],
                                        weight: element.weight,
                                    }),
                            );
                        self.word_initial_only
                            .extend(self.allowed_triphthongs.iter().map(|element| {
                                AllowedCluster {
                                    phonemes: vec![
                                        element.first.clone(),
                                        element.second.clone(),
                                        element.third.clone(),
                                    ],
                                    weight: element.weight,
                                }
                            }));
                    }

                    _ => unreachable!(),
                }
            }
            // allowed_phonemes_and_diphthongs_and_triphthongs
            0b000 => {
                if let Err(e) = validate_phonemes_against_list(
                    &self.allowed_phonemes,
                    &all_vowels(),
                    "Vokal",
                    "unknown_vowel",
                ) {
                    errors.merge(e);
                }
                if let Err(e) = validate_diphthongs_against_list(
                    &self.allowed_diphthongs,
                    &all_vowels(),
                    "Diphthong",
                    "unknown_diphthong",
                ) {
                    errors.merge(e);
                }
                if let Err(e) = validate_triphthongs_against_list(
                    &self.allowed_triphthongs,
                    &all_vowels(),
                    "Triphthongs",
                    "unknown_triphthong",
                ) {
                    errors.merge(e);
                }

                if let Err(e) = self.validate_word_mask(word_mask) {
                    errors.merge(e);
                }

                match word_mask {
                    0b11 => {
                        let clusters: Vec<AllowedCluster> = self
                            .allowed_phonemes
                            .iter()
                            .map(|element| AllowedCluster {
                                phonemes: vec![element.phoneme.clone()],
                                weight: element.weight,
                            })
                            .collect();

                        self.word_initial_only.extend(clusters.clone());
                        self.word_final_only.extend(clusters);

                        let clusters: Vec<AllowedCluster> = self
                            .allowed_diphthongs
                            .iter()
                            .map(|element| AllowedCluster {
                                phonemes: vec![element.first.clone(), element.second.clone()],
                                weight: element.weight,
                            })
                            .collect();

                        self.word_initial_only.extend(clusters.clone());
                        self.word_final_only.extend(clusters);

                        let clusters: Vec<AllowedCluster> = self
                            .allowed_triphthongs
                            .iter()
                            .map(|element| AllowedCluster {
                                phonemes: vec![
                                    element.first.clone(),
                                    element.second.clone(),
                                    element.third.clone(),
                                ],
                                weight: element.weight,
                            })
                            .collect();

                        self.word_initial_only.extend(clusters.clone());
                        self.word_final_only.extend(clusters);
                    }

                    0b01 => {
                        self.word_final_only
                            .extend(self.allowed_phonemes.iter().map(|element| AllowedCluster {
                                phonemes: vec![element.phoneme.clone()],
                                weight: element.weight,
                            }));
                        self.word_final_only
                            .extend(
                                self.allowed_diphthongs
                                    .iter()
                                    .map(|element| AllowedCluster {
                                        phonemes: vec![
                                            element.first.clone(),
                                            element.second.clone(),
                                        ],
                                        weight: element.weight,
                                    }),
                            );
                        self.word_final_only
                            .extend(self.allowed_triphthongs.iter().map(|element| {
                                AllowedCluster {
                                    phonemes: vec![
                                        element.first.clone(),
                                        element.second.clone(),
                                        element.third.clone(),
                                    ],
                                    weight: element.weight,
                                }
                            }));
                    }

                    0b10 => {
                        self.word_initial_only
                            .extend(self.allowed_phonemes.iter().map(|element| AllowedCluster {
                                phonemes: vec![element.phoneme.clone()],
                                weight: element.weight,
                            }));
                        self.word_initial_only
                            .extend(
                                self.allowed_diphthongs
                                    .iter()
                                    .map(|element| AllowedCluster {
                                        phonemes: vec![
                                            element.first.clone(),
                                            element.second.clone(),
                                        ],
                                        weight: element.weight,
                                    }),
                            );
                        self.word_initial_only
                            .extend(self.allowed_triphthongs.iter().map(|element| {
                                AllowedCluster {
                                    phonemes: vec![
                                        element.first.clone(),
                                        element.second.clone(),
                                        element.third.clone(),
                                    ],
                                    weight: element.weight,
                                }
                            }));
                    }

                    _ => unreachable!(),
                }
            }
            8..=u8::MAX => unreachable!(),
        }

        if errors.is_empty() {
            Ok(self.clone())
        } else {
            Err(errors)
        }
    }

    pub fn validate_against_phonology_vowels(
        &self,
        phonology: &PhonologyConfiguration,
    ) -> Result<(), ValidationErrors> {
        let mut errors = ValidationErrors::new();
        // Get available vowels from phonology
        let available_vowels: Vec<String> = phonology
            .vowels
            .iter()
            .map(|c| c.name.to_string())
            .collect();

        // Validate allowed_phonemes against phonology
        for phoneme in &self.allowed_phonemes {
            if !available_vowels.iter().any(|c| c == &phoneme.phoneme) {
                let suggestions =
                    create_phoneme_suggestions(&phoneme.phoneme, &available_vowels, 3);
                let error = create_phoneme_validation_error(
                    &phoneme.phoneme,
                    &suggestions,
                    "vowel_not_in_phonology",
                    "Vokal",
                    None,
                );
                let error_key = format!("phonology_missing_vowels_{}", phoneme.phoneme);
                errors.add(error_key, error);
            }
        }

        for (cluster_idx, cluster) in self.allowed_diphthongs.iter().enumerate() {
            if !available_vowels.iter().any(|c| c == &cluster.first) {
                let suggestions = create_phoneme_suggestions(&cluster.first, &available_vowels, 3);
                let error = create_phoneme_validation_error(
                    &cluster.first,
                    &suggestions,
                    "diphthong_not_in_phonology",
                    "Diphthong",
                    Some(("allowed_diphthongs", cluster_idx, 0)), // First position of diphthong
                );
                let error_key = format!("phonology_missing_diphthong_first_{}", cluster_idx);
                errors.add(error_key, error);
            }

            if !available_vowels.iter().any(|c| c == &cluster.second) {
                let suggestions = create_phoneme_suggestions(&cluster.second, &available_vowels, 3);
                let error = create_phoneme_validation_error(
                    &cluster.second,
                    &suggestions,
                    "diphthong_not_in_phonology",
                    "Diphthong",
                    Some(("allowed_diphthongs", cluster_idx, 1)), // Second position of diphthong
                );
                let error_key = format!("phonology_missing_diphthong_second_{}", cluster_idx);
                errors.add(error_key, error);
            }
        }
        for (cluster_idx, cluster) in self.allowed_triphthongs.iter().enumerate() {
            if !available_vowels.iter().any(|c| c == &cluster.first) {
                let suggestions = create_phoneme_suggestions(&cluster.first, &available_vowels, 3);
                let error = create_phoneme_validation_error(
                    &cluster.first,
                    &suggestions,
                    "triphthong_not_in_phonology",
                    "Triphthong",
                    Some(("allowed_triphthongs", cluster_idx, 0)), // First position
                );
                let error_key = format!("phonology_missing_triphthong_first_{}", cluster_idx);
                errors.add(error_key, error);
            }

            if !available_vowels.iter().any(|c| c == &cluster.second) {
                let suggestions = create_phoneme_suggestions(&cluster.second, &available_vowels, 3);
                let error = create_phoneme_validation_error(
                    &cluster.second,
                    &suggestions,
                    "triphthong_not_in_phonology",
                    "Triphthong",
                    Some(("allowed_triphthongs", cluster_idx, 1)), // Second position
                );
                let error_key = format!("phonology_missing_triphthong_second_{}", cluster_idx);
                errors.add(error_key, error);
            }

            if !available_vowels.iter().any(|c| c == &cluster.third) {
                let suggestions = create_phoneme_suggestions(&cluster.third, &available_vowels, 3);
                let error = create_phoneme_validation_error(
                    &cluster.third,
                    &suggestions,
                    "triphthong_not_in_phonology",
                    "Triphthong",
                    Some(("allowed_triphthongs", cluster_idx, 2)), // Third position
                );
                let error_key = format!("phonology_missing_triphthong_third_{}", cluster_idx);
                errors.add(error_key, error);
            }
        }

        // Validate word_initial_only against phonology
        for (cluster_idx, cluster) in self.word_initial_only.iter().enumerate() {
            for (phoneme_idx, phoneme) in cluster.phonemes.iter().enumerate() {
                if !available_vowels.iter().any(|c| c == phoneme) {
                    let suggestions = create_phoneme_suggestions(phoneme, &available_vowels, 3);
                    let error = create_phoneme_validation_error(
                        phoneme,
                        &suggestions,
                        "vowel_in_word_initial_not_in_phonology",
                        "Vokal",
                        Some(("word_initial_only", cluster_idx, phoneme_idx)),
                    );
                    let error_key = format!(
                        "phonology_missing_word_initial_vowel_{}_{}",
                        cluster_idx, phoneme_idx
                    );
                    errors.add(error_key, error);
                }
            }
        }

        for (cluster_idx, cluster) in self.word_final_only.iter().enumerate() {
            for (phoneme_idx, phoneme) in cluster.phonemes.iter().enumerate() {
                if !available_vowels.iter().any(|c| c == phoneme) {
                    let suggestions = create_phoneme_suggestions(phoneme, &available_vowels, 3);
                    let error = create_phoneme_validation_error(
                        phoneme,
                        &suggestions,
                        "vowel_in_word_final_not_in_phonology",
                        "Vokal",
                        Some(("word_final_only", cluster_idx, phoneme_idx)),
                    );
                    let error_key = format!(
                        "phonology_missing_word_final_vowel_{}_{}",
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

    fn validate_word_mask(&self, word_mask: u8) -> Result<(), ValidationErrors> {
        let mut errors = ValidationErrors::new();
        match &word_mask {
            0b00 => {
                if let Err(e) = validate_clusters_against_list(
                    &self.word_initial_only,
                    &all_vowels(),
                    "Vokal",
                    "unknown_vowel_in_word_initial_only",
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
                if let Err(e) = validate_clusters_against_list(
                    &self.word_final_only,
                    &all_vowels(),
                    "Vokal",
                    "unknown_vowel_in_word_final_only",
                    "word_final_only",
                    Some(&|cluster_idx, phoneme_idx| {
                        format!("invalid_word_final_phoneme_{}_{}", cluster_idx, phoneme_idx)
                    }),
                ) {
                    errors.merge(e);
                }
            }
            0b01 => {
                if let Err(e) = validate_clusters_against_list(
                    &self.word_initial_only,
                    &all_vowels(),
                    "Vokal",
                    "unknown_vowel_in_word_initial_only",
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
            }
            0b10 => {
                if let Err(e) = validate_clusters_against_list(
                    &self.word_final_only,
                    &all_vowels(),
                    "Vokal",
                    "unknown_vowel_in_word_final_only",
                    "word_final_only",
                    Some(&|cluster_idx, phoneme_idx| {
                        format!("invalid_word_final_phoneme_{}_{}", cluster_idx, phoneme_idx)
                    }),
                ) {
                    errors.merge(e);
                }
            }
            _ => {
                // do nothing, no word initial or final clusters allowed
            }
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}
