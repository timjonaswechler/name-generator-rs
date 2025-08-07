use crate::phonology::phonemes::{AllowedCluster, AllowedPhoneme};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CodaConfiguration {
    pub allowed_phonemes: Vec<AllowedPhoneme>,
    pub allowed_clusters: Vec<AllowedCluster>,

    // Word-position specific
    pub word_final_only: Vec<AllowedCluster>, // Nur am Wortanfang erlaubt
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

impl CodaConfiguration {
    pub fn new(
        allowed_phonemes: Vec<AllowedPhoneme>,
        allowed_clusters: Vec<AllowedCluster>,
        word_final_only: Vec<AllowedCluster>,
    ) -> Self {
        Self {
            allowed_phonemes,
            allowed_clusters,
            word_final_only,
        }
    }
}
