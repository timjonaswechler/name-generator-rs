use crate::phonology::phonemes::{
    AllowedCluster, AllowedDiphthong, AllowedPhoneme, AllowedTriphthong,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NucleusConfiguration {
    pub allowed_phonemes: Vec<AllowedPhoneme>,
    pub allowed_diphthongs: Vec<AllowedDiphthong>,
    pub allowed_triphthongs: Vec<AllowedTriphthong>,

    // Word-position specific
    pub word_initial_only: Vec<AllowedCluster>, // Nur am Wortanfang erlaubt
    pub word_final_only: Vec<AllowedCluster>,   // Nur am Wortanfang erlaubt
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
impl NucleusConfiguration {
    pub fn new(
        allowed_phonemes: Vec<AllowedPhoneme>,
        allowed_diphthongs: Vec<AllowedDiphthong>,
        allowed_triphthongs: Vec<AllowedTriphthong>,
        word_initial_only: Vec<AllowedCluster>,
        word_final_only: Vec<AllowedCluster>,
    ) -> Self {
        Self {
            allowed_phonemes,
            allowed_diphthongs,
            allowed_triphthongs,
            word_initial_only,
            word_final_only,
        }
    }
}
