use crate::phonology::{
    AirflowMechanism, Diacritics, Manner, Place, Roundness, Subplace, Suprasegmentals,
    TongueHeight, TonguePosition,
};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// This is the canonical representation used throughout the system
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Phoneme<T> {
    pub ipa: Cow<'static, str>,  // IPA representation of the phoneme
    pub name: Cow<'static, str>, // Human-readable name of the phoneme
    pub phoneme: T,              // Phoneme structure with all features
}

// Type aliases for specific IPA phoneme types
pub type Consonant = Phoneme<ConsonantConfiguration>;
pub type Vowel = Phoneme<VowelConfiguration>;
pub type Diphthong = Phoneme<DiphthongConfiguration>;
pub type Triphthong = Phoneme<TriphthongConfiguration>;

/// General phoneme categories for pattern matching and generic operations
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) enum PhonemeCategory {
    Consonantal(ConsonantConfiguration),
    Vocalic(VowelConfiguration),
    Diphthongal(DiphthongConfiguration),
    Triphthongal(TriphthongConfiguration),
}

// /// Trait for types that can be categorized as phonemes
// pub(crate) trait Phonemic {
//     fn to_category(self) -> PhonemeCategory;
// }

// impl Phonemic for ConsonantConfiguration {
//     fn to_category(self) -> PhonemeCategory {
//         PhonemeCategory::Consonantal(self)
//     }
// }

// impl Phonemic for VowelConfiguration {
//     fn to_category(self) -> PhonemeCategory {
//         PhonemeCategory::Vocalic(self)
//     }
// }
// impl Phonemic for DiphthongConfiguration {
//     fn to_category(self) -> PhonemeCategory {
//         PhonemeCategory::Vocalic(self.first) // Diphthongs are treated as vocalic
//     }
// }
// impl Phonemic for TriphthongConfiguration {
//     fn to_category(self) -> PhonemeCategory {
//         PhonemeCategory::Vocalic(self.first) // Triphthongs are treated as vocalic
//     }
// }

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConsonantConfiguration {
    pub airflow: AirflowMechanism, // Pulmonic, NonPulmonic, Other, etc.
    pub manner: Manner,            // Nasal, Plosive, etc.
    pub place: Place,              // Labial, Alveolar, etc.
    pub subplace: Subplace,        // bilabial, labiodental, etc.
    pub diacritics: Cow<'static, [Diacritics]>, // Voiced, aspirated, etc.
    pub suprasegmentals: Cow<'static, [Suprasegmentals]>, // Stress, tone, etc.
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VowelConfiguration {
    pub position: TonguePosition,               // Front, Central, Back
    pub height: TongueHeight,                   // Close, Open, etc.
    pub roundness: Roundness,                   // Rounded, Unrounded
    pub diacritics: Cow<'static, [Diacritics]>, // Nasalized, etc.
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DiphthongConfiguration {
    pub first: VowelConfiguration,  // First part of the diphthong
    pub second: VowelConfiguration, // Second part of the diphthong
    pub diacritics: Cow<'static, [Diacritics]>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TriphthongConfiguration {
    pub first: VowelConfiguration,  // First part of the triphthong
    pub second: VowelConfiguration, // Second part of the triphthong
    pub third: VowelConfiguration,  // Third part of the triphthong
    pub diacritics: Cow<'static, [Diacritics]>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AllowedPhoneme {
    pub phoneme: String,
    pub weight: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AllowedCluster {
    pub phonemes: Vec<String>,
    pub weight: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AllowedDiphthong {
    pub first: String,
    pub second: String,
    pub weight: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AllowedTriphthong {
    pub first: String,
    pub second: String,
    pub third: String,
    pub weight: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PhonemeCluster {
    pub phonemes: Vec<String>,
}
