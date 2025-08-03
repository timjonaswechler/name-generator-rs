pub mod builder;
pub(crate) mod coda;
pub(crate) mod errors;
pub(crate) mod nucleus;
pub(crate) mod onset;
pub(crate) mod patterns;

use crate::phonology::phonemes::{ConsonantCluster, VowelCluster};

/// Represents a complete syllable with all its components
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Syllable {
    /// Optional onset cluster
    pub onset: Option<ConsonantCluster>,
    /// Required nucleus (vowel core)
    pub nucleus: VowelCluster,
    /// Optional coda cluster
    pub coda: Option<ConsonantCluster>,
}

use crate::syllables::{
    coda::CodaConfiguration, nucleus::NucleusConfiguration, onset::OnsetConfiguration,
    patterns::SyllablePattern,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SyllableConfiguration {
    patterns: Vec<SyllablePattern>,
    onset: OnsetConfiguration,
    nucleus: NucleusConfiguration,
    coda: CodaConfiguration,
}
impl Default for SyllableConfiguration {
    fn default() -> Self {
        Self {
            patterns: Vec::new(),
            onset: OnsetConfiguration::default(),
            nucleus: NucleusConfiguration::default(),
            coda: CodaConfiguration::default(),
        }
    }
}
