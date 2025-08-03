use crate::phonology::phonemes::{AllowedConsonant, AllowedConsonantCluster};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct OnsetConfiguration {
    pub allowed_phonemes: Vec<AllowedConsonant>,
    pub allowed_clusters: Vec<AllowedConsonantCluster>,

    // Word-position specific
    pub word_initial_only: Vec<AllowedConsonant>, // Nur am Wortanfang erlaubt
}

impl Default for OnsetConfiguration {
    fn default() -> Self {
        Self {
            allowed_phonemes: Vec::new(),
            allowed_clusters: Vec::new(),
            word_initial_only: Vec::new(),
        }
    }
}
