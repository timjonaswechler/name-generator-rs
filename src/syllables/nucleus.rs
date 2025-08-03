use crate::phonology::phonemes::{AllowedDiphthong, AllowedTriphthong, AllowedVowel};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct NucleusConfiguration {
    pub allowed_phonemes: Vec<AllowedVowel>,
    pub allowed_diphthongs: Vec<AllowedDiphthong>,
    pub allowed_triphthongs: Vec<AllowedTriphthong>,

    // Word-position specific
    pub word_initial_only: Vec<AllowedVowel>, // Nur am Wortanfang erlaubt
    pub word_final_only: Vec<AllowedVowel>,   // Nur am Wortanfang erlaubt
}

impl Default for NucleusConfiguration {
    fn default() -> Self {
        Self {
            allowed_phonemes: Vec::new(),
            allowed_diphthongs: Vec::new(),
            allowed_triphthongs: Vec::new(),
            word_initial_only: Vec::new(), // Nur am Wortanfang erlaubt
            word_final_only: Vec::new(),
        }
    }
}
