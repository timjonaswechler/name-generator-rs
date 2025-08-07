mod builder;
pub mod consonants;
pub(crate) mod errors;
pub(crate) mod macros;
pub mod phonemes;
pub mod vowels;

pub use crate::phonology::vowels::*;

use phonemes::{Consonant, Vowel};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

pub const EMPTY_DIACRITICS: Cow<'static, [Diacritics]> = Cow::Borrowed(&[]);
pub const EMPTY_SUPRASEGEMENTALS: Cow<'static, [Suprasegmentals]> = Cow::Borrowed(&[]);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PhonologyConfiguration {
    pub consonants: Vec<Cow<'static, Consonant>>,
    pub vowels: Vec<Cow<'static, Vowel>>,
    pub symmetrics_score: f64, // Sc
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AirflowMechanism {
    Pulmonic,
    Click,
    Ejective,
    Implosive,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Manner {
    Nasal,                // m, n, ŋ
    Plosive,              // p, b, t, d, k, g
    SibilantFricative,    // s, z, ʃ, ʒ
    NonSibilantFricative, // f, v, θ, ð, x, h
    Approximant,          // w, j, ɹ
    TapFlap,              // ɾ, ⱱ
    Trill,                // r, ʙ
    LateralApproximant,   // l, ɭ, ʎ, ʟ
    Affricate,            // tʃ, dʒ, ts, dz
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Place {
    Labial,    // p, b, f, v, m
    Coronal,   // t, d, s, z, n, l, r
    Dorsal,    // k, g, x, ŋ
    Laryngeal, // ʔ, h
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Subplace {
    // Labial
    Bilabial,     // p, b, m
    Labiodental,  // f, v
    Linguolabial, //

    // Coronal
    Dental,       // θ, ð
    Alveolar,     // t, d, s, z, n, l
    Postalveolar, // ʃ, ʒ, tʃ, dʒ
    Retroflex,    // ʈ, ɖ, ɳ, ɭ

    // Dorsal
    Palatal, // c, ɟ, ç, ʝ, ɲ, ʎ, j
    Velar,   // k, g, x, ɣ, ŋ
    Uvular,  // q, ɢ, χ, ʁ, ɴ, ʀ

    // Laryngeal
    Pharyngeal, // ʕ, ħ
    Glottal,    // ʔ, h, ɦ
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Diacritics {
    // --- From original enum, not in table ---
    Long, // [aː] (ː)

    // --- Airstream diacritics ---
    Ejective, // [kʼ] (◌ʼ)

    // --- Phonation diacritics ---
    Voiced,        // [s̬] (◌̬)
    Voiceless,     // [n̥] (◌̥)
    BreathyVoiced, // [b̤] (◌̤)
    CreakyVoiced,  // [b̰] (◌̰)

    // --- Syllabicity diacritics ---
    Syllabic,    // [n̩] (◌̩)
    NonSyllabic, // [ɪ̯] (◌̯)

    // --- Consonant-release diacritics ---
    Aspirated,                       // [tʰ] (◌ʰ)
    NoAudibleRelease,                // [p̚] (◌̚)
    NasalRelease,                    // [dⁿ] (◌ⁿ)
    LateralRelease,                  // [dˡ] (◌ˡ)
    VoicelessDentalFricativeRelease, // [tᶿ] (◌ᶿ)
    VoicelessVelarFricativeRelease,  // [tˣ] (◌ˣ)
    MidCentralVowelRelease,          // [dᵊ] (◌ᵊ)

    // --- Articulation diacritics ---
    // Place
    Dental,       // [t̪] (◌̪)
    Linguolabial, // [t̼] (◌̼)
    Dentolabial,  // [ɮ͆] (◌͆)
    Apical,       // [t̺] (◌̺)
    Laminal,      // [t̻] (◌̻)
    // Relative Articulation
    Advanced,          // [u̟] (◌̟)
    Retracted,         // [i̠] (◌̠)
    Centralized,       // [ë] (◌̈)
    MiddleCentralized, // [e̽] (◌̽)
    Raised,            // [e̝] (◌̝)
    Lowered,           // [e̞] (◌̞)

    // --- Co-articulation diacritics ---
    MoreRounded,               // [ɔ̹] (◌̹)
    LessRounded,               // [ɔ̜] (◌̜)
    Labialized,                // [tʷ] (◌ʷ)
    Palatalized,               // [tʲ] (◌ʲ)
    Velarized,                 // [tˠ] (◌ˠ)
    Pharyngealized,            // [tˤ] (◌ˤ)
    VelarizedOrPharyngealized, // [ɫ] (◌̴)
    AdvancedTongueRoot,        // [e̘] (◌̘)
    RetractedTongueRoot,       // [e̙] (◌̙)
    Nasalized,                 // [ẽ] (◌̃)
    Rhoticity,                 // [ɚ] (◌˞)
}

// Vowel enums
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum TonguePosition {
    Front,   // i, e, ɛ, a
    Central, // ɨ, ə, ɐ
    Back,    // ɯ, u, o, ɔ, ɑ
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum TongueHeight {
    Close,     // i, u
    NearClose, // ɪ, ʊ
    CloseMid,  // e, o
    Mid,       // ə
    OpenMid,   // ɛ, ɔ, ʌ
    NearOpen,  // æ
    Open,      // a, ɑ
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Roundness {
    Rounded,   // u, o, ɔ
    Unrounded, // i, e, ɛ, a
}

// Suprasegmentals features
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Suprasegmentals {
    Stress,          // Primary stress
    SecondaryStress, // Secondary stress
    Tone,            // Tonal features
    Length,          // Phonemic length
}
