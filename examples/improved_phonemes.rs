//! Improved phonemes.rs with better visibility control

use serde::{Deserialize, Serialize};
use std::borrow::Cow;

use crate::phonology::{
    AirflowMechanism, Diacritics, Manner, Place, Roundness, Subplace, Suprasegmentals,
    TongueHeight, TonguePosition,
};

/// Public interface for phonemes - only what users need to know
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Phoneme<T> {
    pub symbol: Cow<'static, str>,
    pub description: Cow<'static, str>,
    // Internal configuration is private
    config: T,
}

impl<T> Phoneme<T> {
    pub fn symbol(&self) -> &str {
        &self.symbol
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    // Internal constructor - not public
    pub(crate) fn new(
        symbol: Cow<'static, str>,
        description: Cow<'static, str>,
        config: T,
    ) -> Self {
        Self {
            symbol,
            description,
            config,
        }
    }

    // Access to config only within the crate
    pub(crate) fn config(&self) -> &T {
        &self.config
    }
}

/// Internal consonant configuration - not exposed to users
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct ConsonantConfiguration {
    pub(crate) airflow: AirflowMechanism,
    pub(crate) manner: Manner,
    pub(crate) place: Place,
    pub(crate) subplace: Subplace,
    pub(crate) diacritics: Cow<'static, [Diacritics]>,
    pub(crate) suprasegmentals: Cow<'static, [Suprasegmentals]>,
}

/// Internal vowel configuration - not exposed to users
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct VowelConfiguration {
    pub(crate) position: TonguePosition,
    pub(crate) height: TongueHeight,
    pub(crate) roundness: Roundness,
    pub(crate) diacritics: Cow<'static, [Diacritics]>,
}

/// Public type aliases for user convenience
pub type Consonant = Phoneme<ConsonantConfiguration>;
pub type Vowel = Phoneme<VowelConfiguration>;

/// Public structures for user configuration - simplified interface
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AllowedPhoneme {
    pub symbol: Cow<'static, str>,
    pub weight: f32,
}

impl AllowedPhoneme {
    pub fn new(symbol: impl Into<Cow<'static, str>>, weight: f32) -> Self {
        Self {
            symbol: symbol.into(),
            weight: weight.clamp(0.0, 1.0), // Ensure valid range
        }
    }

    pub fn symbol(&self) -> &str {
        &self.symbol
    }

    pub fn weight(&self) -> f32 {
        self.weight
    }
}

/// Public cluster configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AllowedCluster {
    pub phonemes: Vec<Cow<'static, str>>,
    pub weight: f32,
}

impl AllowedCluster {
    pub fn new(phonemes: Vec<impl Into<Cow<'static, str>>>, weight: f32) -> Self {
        Self {
            phonemes: phonemes.into_iter().map(|p| p.into()).collect(),
            weight: weight.clamp(0.0, 1.0),
        }
    }
}

// Internal structures - only visible within module hierarchy
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(super) struct AllowedDiphthong {
    pub(super) first_vowel: Cow<'static, str>,
    pub(super) second_vowel: Cow<'static, str>,
    pub(super) weight: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(super) struct AllowedTriphthong {
    pub(super) first_vowel: Cow<'static, str>,
    pub(super) second_vowel: Cow<'static, str>,
    pub(super) third_vowel: Cow<'static, str>,
    pub(super) weight: f32,
}

/// Internal cluster implementation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct PhonemeCluster {
    phonemes: Vec<Cow<'static, str>>,
    cluster_type: ClusterType,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) enum ClusterType {
    Consonant,
    Vowel,
    Mixed,
}

impl PhonemeCluster {
    pub(crate) fn new(phonemes: Vec<Cow<'static, str>>) -> Self {
        let cluster_type = if phonemes.is_empty() {
            ClusterType::Mixed
        } else {
            // Logic to determine cluster type based on phonemes
            ClusterType::Mixed // Simplified for example
        };

        Self {
            phonemes,
            cluster_type,
        }
    }

    pub(crate) fn phonemes(&self) -> &[Cow<'static, str>] {
        &self.phonemes
    }
}
