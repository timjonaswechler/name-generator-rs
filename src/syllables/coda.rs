use crate::phonology::phonemes::{AllowedConsonant, AllowedConsonantCluster};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CodaConfiguration {
    pub allowed_phonemes: Vec<AllowedConsonant>,
    pub allowed_clusters: Vec<AllowedConsonantCluster>,

    // Word-position specific
    pub word_final_only: Vec<AllowedConsonant>, // Nur am Wortanfang erlaubt
}

impl Default for CodaConfiguration {
    fn default() -> Self {
        Self {
            allowed_phonemes: Vec::new(),
            allowed_clusters: Vec::new(),
            word_final_only: Vec::new(),
        }
    }
}
