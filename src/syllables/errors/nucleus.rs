use crate::{
    phonology::{
        all_vowels, consonants::all_consonants, phonemes::AllowedCluster, PhonologyConfiguration,
    },
    syllables::{
        errors::utilities::{
            create_phoneme_validation_error, generate_phoneme_suggestions,
            validate_clusters_against_list_with_error_key, validate_diphthongs_against_list,
            validate_phonemes_against_list, validate_triphthongs_against_list,
        },
        nucleus::NucleusConfiguration,
    },
    validation::{ValidationError, ValidationErrors},
};

impl NucleusConfiguration {
    pub fn validate(&mut self) -> Result<NucleusConfiguration, ValidationErrors> {
        let mut errors = ValidationErrors::new();

        // checke with is empty
        let empty_check = [
            self.allowed_phonemes.is_empty(),
            self.allowed_diphthongs.is_empty(),
            self.allowed_triphthongs.is_empty(),
            self.word_initial_only.is_empty(),
            self.word_final_only.is_empty(),
        ];
        match empty_check {
            [true, true, true, true, true]
            | [true, true, true, true, false]
            | [true, true, true, false, true]
            | [true, true, true, false, false] => {
                errors.add(
                    "empty_nucleus",
                    ValidationError::new("empty_nucleus")
                        .with_message("Nucleus must contain at least one phoneme or cluster"),
                );
            }
            [false, true, true, true, true] => {
                if let Err(e) = validate_phonemes_against_list(
                    &self.allowed_phonemes,
                    &all_vowels(),
                    "Vokal",
                    "unknown_vowel",
                ) {
                    errors.merge(e);
                }
                for element in &self.allowed_phonemes {
                    let temp = AllowedCluster {
                        phonemes: vec![element.phoneme.clone()],
                        weight: element.weight,
                    };
                    self.word_initial_only.push(temp.clone());
                    self.word_final_only.push(temp);
                }
            }
            [false, true, true, false, true] => {
                if let Err(e) = validate_phonemes_against_list(
                    &self.allowed_phonemes,
                    &all_vowels(),
                    "Vokal",
                    "unknown_vowel",
                ) {
                    errors.merge(e);
                }
                if let Err(e) = validate_clusters_against_list_with_error_key(
                    &self.word_initial_only,
                    &all_vowels(),
                    "Vokal",
                    "unknown_vowel_in_word_initial_only",
                    "word_initial_only",
                    |cluster_idx, phoneme_idx| {
                        format!(
                            "invalid_word_initial_phoneme_{}_{}",
                            cluster_idx, phoneme_idx
                        )
                    },
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
            [false, true, true, true, false] => {
                if let Err(e) = validate_phonemes_against_list(
                    &self.allowed_phonemes,
                    &all_vowels(),
                    "Vokal",
                    "unknown_vowel",
                ) {
                    errors.merge(e);
                }
                if let Err(e) = validate_clusters_against_list_with_error_key(
                    &self.word_final_only,
                    &all_vowels(),
                    "Vokal",
                    "unknown_vowel_in_word_final_only",
                    "word_final_only",
                    |cluster_idx, phoneme_idx| {
                        format!("invalid_word_final_phoneme_{}_{}", cluster_idx, phoneme_idx)
                    },
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
            [false, false, true, true, true] => {
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
                for element in &self.allowed_phonemes {
                    let temp = AllowedCluster {
                        phonemes: vec![element.phoneme.clone()],
                        weight: element.weight,
                    };
                    self.word_initial_only.push(temp.clone());
                    self.word_final_only.push(temp);
                }
                for element in &self.allowed_diphthongs {
                    let temp = AllowedCluster {
                        phonemes: vec![element.first.clone(), element.second.clone()],
                        weight: element.weight,
                    };
                    self.word_initial_only.push(temp.clone());
                    self.word_final_only.push(temp);
                }
            }
            [false, false, true, false, true] => {
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
                if let Err(e) = validate_clusters_against_list_with_error_key(
                    &self.word_initial_only,
                    &all_vowels(),
                    "Vokal",
                    "unknown_vowel_in_word_initial_only",
                    "word_initial_only",
                    |cluster_idx, phoneme_idx| {
                        format!(
                            "invalid_word_initial_phoneme_{}_{}",
                            cluster_idx, phoneme_idx
                        )
                    },
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
                for element in &self.allowed_diphthongs {
                    let temp = AllowedCluster {
                        phonemes: vec![element.first.clone(), element.second.clone()],
                        weight: element.weight,
                    };
                    self.word_initial_only.push(temp);
                }
            }
            [false, false, true, true, false] => {
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
                if let Err(e) = validate_clusters_against_list_with_error_key(
                    &self.word_final_only,
                    &all_vowels(),
                    "Vokal",
                    "unknown_vowel_in_word_final_only",
                    "word_final_only",
                    |cluster_idx, phoneme_idx| {
                        format!("invalid_word_final_phoneme_{}_{}", cluster_idx, phoneme_idx)
                    },
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
                for element in &self.allowed_diphthongs {
                    let temp = AllowedCluster {
                        phonemes: vec![element.first.clone(), element.second.clone()],
                        weight: element.weight,
                    };
                    self.word_final_only.push(temp);
                }
            }
            [false, true, false, true, true] => {
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
                for element in &self.allowed_phonemes {
                    let temp = AllowedCluster {
                        phonemes: vec![element.phoneme.clone()],
                        weight: element.weight,
                    };
                    self.word_initial_only.push(temp.clone());
                    self.word_final_only.push(temp);
                }
                for element in &self.allowed_triphthongs {
                    let temp = AllowedCluster {
                        phonemes: vec![
                            element.first.clone(),
                            element.second.clone(),
                            element.third.clone(),
                        ],
                        weight: element.weight,
                    };
                    self.word_initial_only.push(temp.clone());
                    self.word_final_only.push(temp);
                }
            }
            [false, true, false, false, true] => {
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
                if let Err(e) = validate_clusters_against_list_with_error_key(
                    &self.word_initial_only,
                    &all_vowels(),
                    "Vokal",
                    "unknown_vowel_in_word_initial_only",
                    "word_initial_only",
                    |cluster_idx, phoneme_idx| {
                        format!(
                            "invalid_word_initial_phoneme_{}_{}",
                            cluster_idx, phoneme_idx
                        )
                    },
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
                for element in &self.allowed_triphthongs {
                    let temp = AllowedCluster {
                        phonemes: vec![
                            element.first.clone(),
                            element.second.clone(),
                            element.third.clone(),
                        ],
                        weight: element.weight,
                    };
                    self.word_initial_only.push(temp);
                }
            }
            [false, true, false, true, false] => {
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
                if let Err(e) = validate_clusters_against_list_with_error_key(
                    &self.word_final_only,
                    &all_vowels(),
                    "Vokal",
                    "unknown_vowel_in_word_final_only",
                    "word_final_only",
                    |cluster_idx, phoneme_idx| {
                        format!("invalid_word_final_phoneme_{}_{}", cluster_idx, phoneme_idx)
                    },
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
                for element in &self.allowed_triphthongs {
                    let temp = AllowedCluster {
                        phonemes: vec![
                            element.first.clone(),
                            element.second.clone(),
                            element.third.clone(),
                        ],
                        weight: element.weight,
                    };
                    self.word_final_only.push(temp);
                }
            }
            [true, false, true, true, true] => {
                if let Err(e) = validate_diphthongs_against_list(
                    &self.allowed_diphthongs,
                    &all_vowels(),
                    "Diphthong",
                    "unknown_diphthong",
                ) {
                    errors.merge(e);
                }
                for element in &self.allowed_diphthongs {
                    let temp = AllowedCluster {
                        phonemes: vec![element.first.clone(), element.second.clone()],
                        weight: element.weight,
                    };
                    self.word_initial_only.push(temp.clone());
                    self.word_final_only.push(temp);
                }
            }
            [true, false, true, false, true] => {
                if let Err(e) = validate_diphthongs_against_list(
                    &self.allowed_diphthongs,
                    &all_vowels(),
                    "Diphthong",
                    "unknown_diphthong",
                ) {
                    errors.merge(e);
                }
                if let Err(e) = validate_clusters_against_list_with_error_key(
                    &self.word_initial_only,
                    &all_vowels(),
                    "Vokal",
                    "unknown_vowel_in_word_initial_only",
                    "word_initial_only",
                    |cluster_idx, phoneme_idx| {
                        format!(
                            "invalid_word_initial_phoneme_{}_{}",
                            cluster_idx, phoneme_idx
                        )
                    },
                ) {
                    errors.merge(e);
                }
                for element in &self.allowed_diphthongs {
                    let temp = AllowedCluster {
                        phonemes: vec![element.first.clone(), element.second.clone()],
                        weight: element.weight,
                    };
                    self.word_initial_only.push(temp);
                }
            }
            [true, false, true, true, false] => {
                if let Err(e) = validate_diphthongs_against_list(
                    &self.allowed_diphthongs,
                    &all_vowels(),
                    "Diphthong",
                    "unknown_diphthong",
                ) {
                    errors.merge(e);
                }
                if let Err(e) = validate_clusters_against_list_with_error_key(
                    &self.word_final_only,
                    &all_vowels(),
                    "Vokal",
                    "unknown_vowel_in_word_final_only",
                    "word_final_only",
                    |cluster_idx, phoneme_idx| {
                        format!("invalid_word_final_phoneme_{}_{}", cluster_idx, phoneme_idx)
                    },
                ) {
                    errors.merge(e);
                }
                for element in &self.allowed_diphthongs {
                    let temp = AllowedCluster {
                        phonemes: vec![element.first.clone(), element.second.clone()],
                        weight: element.weight,
                    };
                    self.word_final_only.push(temp);
                }
            }
            [true, false, false, true, true] => {
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
                for element in &self.allowed_diphthongs {
                    let temp = AllowedCluster {
                        phonemes: vec![element.first.clone(), element.second.clone()],
                        weight: element.weight,
                    };
                    self.word_initial_only.push(temp.clone());
                    self.word_final_only.push(temp);
                }
                for element in &self.allowed_triphthongs {
                    let temp = AllowedCluster {
                        phonemes: vec![
                            element.first.clone(),
                            element.second.clone(),
                            element.third.clone(),
                        ],
                        weight: element.weight,
                    };
                    self.word_initial_only.push(temp.clone());
                    self.word_final_only.push(temp);
                }
            }
            [true, false, false, false, true] => {
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
                if let Err(e) = validate_clusters_against_list_with_error_key(
                    &self.word_initial_only,
                    &all_vowels(),
                    "Vokal",
                    "unknown_vowel_in_word_initial_only",
                    "word_initial_only",
                    |cluster_idx, phoneme_idx| {
                        format!(
                            "invalid_word_initial_phoneme_{}_{}",
                            cluster_idx, phoneme_idx
                        )
                    },
                ) {
                    errors.merge(e);
                }
                for element in &self.allowed_diphthongs {
                    let temp = AllowedCluster {
                        phonemes: vec![element.first.clone(), element.second.clone()],
                        weight: element.weight,
                    };
                    self.word_initial_only.push(temp);
                }
                for element in &self.allowed_triphthongs {
                    let temp = AllowedCluster {
                        phonemes: vec![
                            element.first.clone(),
                            element.second.clone(),
                            element.third.clone(),
                        ],
                        weight: element.weight,
                    };
                    self.word_initial_only.push(temp);
                }
            }
            [true, false, false, true, false] => {
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
                if let Err(e) = validate_clusters_against_list_with_error_key(
                    &self.word_final_only,
                    &all_vowels(),
                    "Vokal",
                    "unknown_vowel_in_word_final_only",
                    "word_final_only",
                    |cluster_idx, phoneme_idx| {
                        format!("invalid_word_final_phoneme_{}_{}", cluster_idx, phoneme_idx)
                    },
                ) {
                    errors.merge(e);
                }
                for element in &self.allowed_diphthongs {
                    let temp = AllowedCluster {
                        phonemes: vec![element.first.clone(), element.second.clone()],
                        weight: element.weight,
                    };
                    self.word_final_only.push(temp);
                }
                for element in &self.allowed_triphthongs {
                    let temp = AllowedCluster {
                        phonemes: vec![
                            element.first.clone(),
                            element.second.clone(),
                            element.third.clone(),
                        ],
                        weight: element.weight,
                    };
                    self.word_final_only.push(temp);
                }
            }
            [true, true, false, true, true] => {
                if let Err(e) = validate_triphthongs_against_list(
                    &self.allowed_triphthongs,
                    &all_vowels(),
                    "Triphthongs",
                    "unknown_triphthong",
                ) {
                    errors.merge(e);
                }
                for element in &self.allowed_triphthongs {
                    let temp = AllowedCluster {
                        phonemes: vec![
                            element.first.clone(),
                            element.second.clone(),
                            element.third.clone(),
                        ],
                        weight: element.weight,
                    };
                    self.word_initial_only.push(temp.clone());
                    self.word_final_only.push(temp);
                }
            }
            [true, true, false, false, true] => {
                if let Err(e) = validate_triphthongs_against_list(
                    &self.allowed_triphthongs,
                    &all_vowels(),
                    "Triphthongs",
                    "unknown_triphthong",
                ) {
                    errors.merge(e);
                }
                if let Err(e) = validate_clusters_against_list_with_error_key(
                    &self.word_initial_only,
                    &all_vowels(),
                    "Vokal",
                    "unknown_vowel_in_word_initial_only",
                    "word_initial_only",
                    |cluster_idx, phoneme_idx| {
                        format!(
                            "invalid_word_initial_phoneme_{}_{}",
                            cluster_idx, phoneme_idx
                        )
                    },
                ) {
                    errors.merge(e);
                }
                for element in &self.allowed_triphthongs {
                    let temp = AllowedCluster {
                        phonemes: vec![
                            element.first.clone(),
                            element.second.clone(),
                            element.third.clone(),
                        ],
                        weight: element.weight,
                    };
                    self.word_initial_only.push(temp);
                }
            }
            [true, true, false, true, false] => {
                if let Err(e) = validate_triphthongs_against_list(
                    &self.allowed_triphthongs,
                    &all_vowels(),
                    "Triphthongs",
                    "unknown_triphthong",
                ) {
                    errors.merge(e);
                }
                if let Err(e) = validate_clusters_against_list_with_error_key(
                    &self.word_final_only,
                    &all_vowels(),
                    "Vokal",
                    "unknown_vowel_in_word_final_only",
                    "word_final_only",
                    |cluster_idx, phoneme_idx| {
                        format!("invalid_word_final_phoneme_{}_{}", cluster_idx, phoneme_idx)
                    },
                ) {
                    errors.merge(e);
                }
                for element in &self.allowed_triphthongs {
                    let temp = AllowedCluster {
                        phonemes: vec![
                            element.first.clone(),
                            element.second.clone(),
                            element.third.clone(),
                        ],
                        weight: element.weight,
                    };
                    self.word_final_only.push(temp);
                }
            }
            [false, false, false, true, true] => {
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
                for element in &self.allowed_phonemes {
                    let temp = AllowedCluster {
                        phonemes: vec![element.phoneme.clone()],
                        weight: element.weight,
                    };
                    self.word_initial_only.push(temp.clone());
                    self.word_final_only.push(temp);
                }
                for element in &self.allowed_diphthongs {
                    let temp = AllowedCluster {
                        phonemes: vec![element.first.clone(), element.second.clone()],
                        weight: element.weight,
                    };
                    self.word_initial_only.push(temp.clone());
                    self.word_final_only.push(temp);
                }
                for element in &self.allowed_triphthongs {
                    let temp = AllowedCluster {
                        phonemes: vec![
                            element.first.clone(),
                            element.second.clone(),
                            element.third.clone(),
                        ],
                        weight: element.weight,
                    };
                    self.word_initial_only.push(temp.clone());
                    self.word_final_only.push(temp);
                }
            }
            [false, false, false, false, true] => {
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
                if let Err(e) = validate_clusters_against_list_with_error_key(
                    &self.word_initial_only,
                    &all_vowels(),
                    "Vokal",
                    "unknown_vowel_in_word_initial_only",
                    "word_initial_only",
                    |cluster_idx, phoneme_idx| {
                        format!(
                            "invalid_word_initial_phoneme_{}_{}",
                            cluster_idx, phoneme_idx
                        )
                    },
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
                for element in &self.allowed_diphthongs {
                    let temp = AllowedCluster {
                        phonemes: vec![element.first.clone(), element.second.clone()],
                        weight: element.weight,
                    };
                    self.word_initial_only.push(temp);
                }
                for element in &self.allowed_triphthongs {
                    let temp = AllowedCluster {
                        phonemes: vec![
                            element.first.clone(),
                            element.second.clone(),
                            element.third.clone(),
                        ],
                        weight: element.weight,
                    };
                    self.word_initial_only.push(temp);
                }
            }
            [false, false, false, true, false] => {
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
                if let Err(e) = validate_clusters_against_list_with_error_key(
                    &self.word_final_only,
                    &all_vowels(),
                    "Vokal",
                    "unknown_vowel_in_word_final_only",
                    "word_final_only",
                    |cluster_idx, phoneme_idx| {
                        format!("invalid_word_final_phoneme_{}_{}", cluster_idx, phoneme_idx)
                    },
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
                for element in &self.allowed_diphthongs {
                    let temp = AllowedCluster {
                        phonemes: vec![element.first.clone(), element.second.clone()],
                        weight: element.weight,
                    };
                    self.word_final_only.push(temp);
                }
                for element in &self.allowed_triphthongs {
                    let temp = AllowedCluster {
                        phonemes: vec![
                            element.first.clone(),
                            element.second.clone(),
                            element.third.clone(),
                        ],
                        weight: element.weight,
                    };
                    self.word_final_only.push(temp);
                }
            }
            [true, false, false, false, false] => {
                if let Err(e) = validate_triphthongs_against_list(
                    &self.allowed_triphthongs,
                    &all_vowels(),
                    "Triphthongs",
                    "unknown_triphthong",
                ) {
                    errors.merge(e);
                }
                if let Err(e) = validate_clusters_against_list_with_error_key(
                    &self.word_initial_only,
                    &all_vowels(),
                    "Vokal",
                    "unknown_vowel_in_word_initial_only",
                    "word_initial_only",
                    |cluster_idx, phoneme_idx| {
                        format!(
                            "invalid_word_initial_phoneme_{}_{}",
                            cluster_idx, phoneme_idx
                        )
                    },
                ) {
                    errors.merge(e);
                }
                if let Err(e) = validate_clusters_against_list_with_error_key(
                    &self.word_final_only,
                    &all_vowels(),
                    "Vokal",
                    "unknown_vowel_in_word_final_only",
                    "word_final_only",
                    |cluster_idx, phoneme_idx| {
                        format!("invalid_word_final_phoneme_{}_{}", cluster_idx, phoneme_idx)
                    },
                ) {
                    errors.merge(e);
                }
            }
            [false, true, false, false, false] => {
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
                if let Err(e) = validate_clusters_against_list_with_error_key(
                    &self.word_initial_only,
                    &all_vowels(),
                    "Vokal",
                    "unknown_vowel_in_word_initial_only",
                    "word_initial_only",
                    |cluster_idx, phoneme_idx| {
                        format!(
                            "invalid_word_initial_phoneme_{}_{}",
                            cluster_idx, phoneme_idx
                        )
                    },
                ) {
                    errors.merge(e);
                }
                if let Err(e) = validate_clusters_against_list_with_error_key(
                    &self.word_final_only,
                    &all_vowels(),
                    "Vokal",
                    "unknown_vowel_in_word_final_only",
                    "word_final_only",
                    |cluster_idx, phoneme_idx| {
                        format!("invalid_word_final_phoneme_{}_{}", cluster_idx, phoneme_idx)
                    },
                ) {
                    errors.merge(e);
                }
            }
            [false, false, true, false, false] => {
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

                if let Err(e) = validate_clusters_against_list_with_error_key(
                    &self.word_initial_only,
                    &all_vowels(),
                    "Vokal",
                    "unknown_vowel_in_word_initial_only",
                    "word_initial_only",
                    |cluster_idx, phoneme_idx| {
                        format!(
                            "invalid_word_initial_phoneme_{}_{}",
                            cluster_idx, phoneme_idx
                        )
                    },
                ) {
                    errors.merge(e);
                }
                if let Err(e) = validate_clusters_against_list_with_error_key(
                    &self.word_final_only,
                    &all_vowels(),
                    "Vokal",
                    "unknown_vowel_in_word_final_only",
                    "word_final_only",
                    |cluster_idx, phoneme_idx| {
                        format!("invalid_word_final_phoneme_{}_{}", cluster_idx, phoneme_idx)
                    },
                ) {
                    errors.merge(e);
                }
            }
            [true, true, false, false, false] => {
                if let Err(e) = validate_triphthongs_against_list(
                    &self.allowed_triphthongs,
                    &all_vowels(),
                    "Triphthongs",
                    "unknown_triphthong",
                ) {
                    errors.merge(e);
                }
                if let Err(e) = validate_clusters_against_list_with_error_key(
                    &self.word_initial_only,
                    &all_vowels(),
                    "Vokal",
                    "unknown_vowel_in_word_initial_only",
                    "word_initial_only",
                    |cluster_idx, phoneme_idx| {
                        format!(
                            "invalid_word_initial_phoneme_{}_{}",
                            cluster_idx, phoneme_idx
                        )
                    },
                ) {
                    errors.merge(e);
                }
                if let Err(e) = validate_clusters_against_list_with_error_key(
                    &self.word_final_only,
                    &all_vowels(),
                    "Vokal",
                    "unknown_vowel_in_word_final_only",
                    "word_final_only",
                    |cluster_idx, phoneme_idx| {
                        format!("invalid_word_final_phoneme_{}_{}", cluster_idx, phoneme_idx)
                    },
                ) {
                    errors.merge(e);
                }
            }
            [true, false, true, false, false] => {
                if let Err(e) = validate_diphthongs_against_list(
                    &self.allowed_diphthongs,
                    &all_vowels(),
                    "Diphthong",
                    "unknown_diphthong",
                ) {
                    errors.merge(e);
                }

                if let Err(e) = validate_clusters_against_list_with_error_key(
                    &self.word_initial_only,
                    &all_vowels(),
                    "Vokal",
                    "unknown_vowel_in_word_initial_only",
                    "word_initial_only",
                    |cluster_idx, phoneme_idx| {
                        format!(
                            "invalid_word_initial_phoneme_{}_{}",
                            cluster_idx, phoneme_idx
                        )
                    },
                ) {
                    errors.merge(e);
                }
                if let Err(e) = validate_clusters_against_list_with_error_key(
                    &self.word_final_only,
                    &all_vowels(),
                    "Vokal",
                    "unknown_vowel_in_word_final_only",
                    "word_final_only",
                    |cluster_idx, phoneme_idx| {
                        format!("invalid_word_final_phoneme_{}_{}", cluster_idx, phoneme_idx)
                    },
                ) {
                    errors.merge(e);
                }
            }
            [false, true, true, false, false] => {
                if let Err(e) = validate_phonemes_against_list(
                    &self.allowed_phonemes,
                    &all_vowels(),
                    "Vokal",
                    "unknown_vowel",
                ) {
                    errors.merge(e);
                }
                if let Err(e) = validate_clusters_against_list_with_error_key(
                    &self.word_initial_only,
                    &all_vowels(),
                    "Vokal",
                    "unknown_vowel_in_word_initial_only",
                    "word_initial_only",
                    |cluster_idx, phoneme_idx| {
                        format!(
                            "invalid_word_initial_phoneme_{}_{}",
                            cluster_idx, phoneme_idx
                        )
                    },
                ) {
                    errors.merge(e);
                }
                if let Err(e) = validate_clusters_against_list_with_error_key(
                    &self.word_final_only,
                    &all_vowels(),
                    "Vokal",
                    "unknown_vowel_in_word_final_only",
                    "word_final_only",
                    |cluster_idx, phoneme_idx| {
                        format!("invalid_word_final_phoneme_{}_{}", cluster_idx, phoneme_idx)
                    },
                ) {
                    errors.merge(e);
                }
            }
            [false, false, false, false, false] => {
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
                if let Err(e) = validate_clusters_against_list_with_error_key(
                    &self.word_initial_only,
                    &all_vowels(),
                    "Vokal",
                    "unknown_vowel_in_word_initial_only",
                    "word_initial_only",
                    |cluster_idx, phoneme_idx| {
                        format!(
                            "invalid_word_initial_phoneme_{}_{}",
                            cluster_idx, phoneme_idx
                        )
                    },
                ) {
                    errors.merge(e);
                }
                if let Err(e) = validate_clusters_against_list_with_error_key(
                    &self.word_final_only,
                    &all_vowels(),
                    "Vokal",
                    "unknown_vowel_in_word_final_only",
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
                    generate_phoneme_suggestions(&phoneme.phoneme, &available_vowels, 3);
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
                let suggestions =
                    generate_phoneme_suggestions(&cluster.first, &available_vowels, 3);
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
                let suggestions =
                    generate_phoneme_suggestions(&cluster.second, &available_vowels, 3);
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
                let suggestions =
                    generate_phoneme_suggestions(&cluster.first, &available_vowels, 3);
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
                let suggestions =
                    generate_phoneme_suggestions(&cluster.second, &available_vowels, 3);
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
                let suggestions =
                    generate_phoneme_suggestions(&cluster.third, &available_vowels, 3);
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
                    let suggestions = generate_phoneme_suggestions(phoneme, &available_vowels, 3);
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
                    let suggestions = generate_phoneme_suggestions(phoneme, &available_vowels, 3);
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
}
