use crate::phonology::phonemes::{AllowedCluster, AllowedPhoneme};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OnsetConfiguration {
    pub allowed_phonemes: Vec<AllowedPhoneme>,
    pub allowed_clusters: Vec<AllowedCluster>,

    // Word-position specific
    pub word_initial_only: Vec<AllowedCluster>, // Nur am Wortanfang erlaubt
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
impl OnsetConfiguration {
    pub fn new(
        allowed_phonemes: Vec<AllowedPhoneme>,
        allowed_clusters: Vec<AllowedCluster>,
        word_initial_only: Vec<AllowedCluster>,
    ) -> Self {
        Self {
            allowed_phonemes,
            allowed_clusters,
            word_initial_only,
        }
    }
}
