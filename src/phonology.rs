//! IPA sound data and language-specific inventories.
mod builder;
pub mod catalog;
pub mod consonants;
mod descriptions;
pub mod diacritics;
pub(crate) mod macros;
pub mod phonemes;
pub mod vowels;

pub use builder::LanguageInventory;
pub use catalog::SoundCatalog;
pub use phonemes::{SoundDefinition, SoundFeatures, SoundId};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AirflowMechanism {
    Pulmonic,
    Click,
    Ejective,
    Implosive,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Manner {
    Nasal,
    Plosive,
    SibilantFricative,
    NonSibilantFricative,
    Fricative,
    Approximant,
    TapFlap,
    Trill,
    LateralApproximant,
    LateralFricative,
    LateralTapFlap,
    LateralPlosive,
    Affricate,
}

/// The precise place is authoritative; no independently editable parent group.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ArticulationPlace {
    Bilabial,
    Labiodental,
    Linguolabial,
    Dental,
    Alveolar,
    Postalveolar,
    AlveoloPalatal,
    Retroflex,
    Palatal,
    Velar,
    Uvular,
    Pharyngeal,
    Epiglottal,
    Glottal,
    LabialVelar,
    LabialPalatal,
    PostalveolarVelar,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Voicing {
    Voiceless,
    Voiced,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Phonation {
    Modal,
    Breathy,
    Creaky,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Aspiration {
    Unaspirated,
    Aspirated,
}

/// Modifiers whose production support must be checked explicitly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Modifier {
    Raised,
    Lowered,
    Centralized,
    Advanced,
    Retracted,
    Nasalized,
    Labialized,
    Palatalized,
    Velarized,
    Pharyngealized,
    Syllabic,
    NonSyllabic,
    MoreRounded,
    LessRounded,
    MidCentralized,
    Rhoticity,
    Linguolabial,
    VelarizedOrPharyngealized,
    AdvancedTongueRoot,
    RetractedTongueRoot,
    Dental,
    Apical,
    Laminal,
    NasalRelease,
    LateralRelease,
    NoAudibleRelease,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TonguePosition {
    Front,
    NearFront,
    Central,
    NearBack,
    Back,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TongueHeight {
    Close,
    NearClose,
    CloseMid,
    Mid,
    OpenMid,
    NearOpen,
    Open,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Roundness {
    Rounded,
    Unrounded,
}
