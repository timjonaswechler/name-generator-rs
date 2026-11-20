use super::*;
use crate::validation::{ValidationError, ValidationErrors};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct SoundId(Cow<'static, str>);

impl SoundId {
    pub const fn from_static(value: &'static str) -> Self {
        assert!(
            valid_id(value),
            "sound IDs require ASCII letters, digits, underscores or hyphens"
        );
        Self(Cow::Borrowed(value))
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
const fn valid_id(value: &str) -> bool {
    let bytes = value.as_bytes();
    if bytes.is_empty() {
        return false;
    }
    let mut i = 0;
    while i < bytes.len() {
        let b = bytes[i];
        if !((b >= b'a' && b <= b'z')
            || (b >= b'A' && b <= b'Z')
            || (b >= b'0' && b <= b'9')
            || b == b'_'
            || b == b'-')
        {
            return false;
        }
        i += 1;
    }
    true
}
impl TryFrom<String> for SoundId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if !valid_id(&value) {
            Err("sound IDs require ASCII letters, digits, underscores or hyphens")
        } else {
            Ok(Self(Cow::Owned(value)))
        }
    }
}
impl From<SoundId> for String {
    fn from(value: SoundId) -> Self {
        value.0.into_owned()
    }
}
impl std::fmt::Display for SoundId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConsonantFeatures {
    pub airflow: AirflowMechanism,
    pub manner: Manner,
    /// The release phase of an affricate; its stop closure uses `place`.
    pub affricate_release: Option<ArticulationPlace>,
    pub place: ArticulationPlace,
    pub voicing: Voicing,
    pub phonation: Phonation,
    pub aspiration: Aspiration,
    pub modifiers: Cow<'static, [Modifier]>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VowelFeatures {
    pub position: TonguePosition,
    pub height: TongueHeight,
    pub roundness: Roundness,
    pub voicing: Voicing,
    pub phonation: Phonation,
    pub modifiers: Cow<'static, [Modifier]>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SoundFeatures {
    Consonant(ConsonantFeatures),
    Vowel(VowelFeatures),
}

/// A catalogue sound, not a language-specific phonemic contrast.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct SoundDefinition {
    id: SoundId,
    ipa: Cow<'static, str>,
    features: SoundFeatures,
}
impl SoundDefinition {
    /// Checks structural invariants, not the phonetic accuracy of the supplied IPA.
    /// Catalogue authors remain responsible for auditing notation against features.
    pub fn new(
        id: SoundId,
        ipa: String,
        features: SoundFeatures,
    ) -> Result<Self, ValidationErrors> {
        let sound = Self {
            id,
            ipa: Cow::Owned(ipa),
            features,
        };
        sound.validate()?;
        Ok(sound)
    }
    pub(crate) const fn builtin(
        id: &'static str,
        ipa: &'static str,
        features: SoundFeatures,
    ) -> Self {
        Self {
            id: SoundId::from_static(id),
            ipa: Cow::Borrowed(ipa),
            features,
        }
    }
    pub fn id(&self) -> &SoundId {
        &self.id
    }
    pub fn ipa(&self) -> &str {
        &self.ipa
    }
    pub fn features(&self) -> &SoundFeatures {
        &self.features
    }
    pub fn modifiers(&self) -> &[Modifier] {
        match &self.features {
            SoundFeatures::Consonant(f) => &f.modifiers,
            SoundFeatures::Vowel(f) => &f.modifiers,
        }
    }
    pub fn is_consonant(&self) -> bool {
        matches!(self.features, SoundFeatures::Consonant(_))
    }
    pub fn can_be_margin(&self) -> bool {
        !self.modifiers().contains(&Modifier::Syllabic)
            && (self.is_consonant() || self.modifiers().contains(&Modifier::NonSyllabic))
    }
    pub fn can_be_nucleus(&self) -> bool {
        !self.modifiers().contains(&Modifier::NonSyllabic)
            && (!self.is_consonant() || self.modifiers().contains(&Modifier::Syllabic))
    }
    pub(crate) fn validate(&self) -> Result<(), ValidationErrors> {
        let mut errors = ValidationErrors::new();
        if self.ipa.trim().is_empty() {
            errors.add("ipa", ValidationError::new("empty_ipa"));
        }
        // Canonical prosodic marks belong to segments, syllables or larger units.
        // This guard is intentionally not a general Unicode/IPA parser.
        if self.ipa.chars().any(|c| {
            matches!(
                c,
                'ˈ' | 'ˌ'
                    | 'ː'
                    | 'ˑ'
                    | '\u{0306}'
                    | '˥'
                    | '˦'
                    | '˧'
                    | '˨'
                    | '˩'
                    | 'ꜜ'
                    | 'ꜛ'
                    | '↑'
                    | '↓'
                    | '↗'
                    | '↘'
                    | '.'
                    | '|'
                    | '‖'
                    | '\u{0300}'
                    | '\u{0301}'
                    | '\u{0302}'
                    | '\u{0304}'
                    | '\u{030B}'
                    | '\u{030C}'
                    | '\u{030F}'
            )
        }) {
            errors.add(
                "ipa",
                ValidationError::new("prosody_must_not_be_embedded_in_sound_definition"),
            );
        }
        if let SoundFeatures::Consonant(f) = &self.features {
            // These combinations occupy shaded cells in the official pulmonic chart.
            let impossible = (f.manner == Manner::Nasal
                && matches!(
                    f.place,
                    ArticulationPlace::Pharyngeal
                        | ArticulationPlace::Epiglottal
                        | ArticulationPlace::Glottal
                ))
                || (f.manner == Manner::Plosive
                    && f.place == ArticulationPlace::Glottal
                    && f.voicing == Voicing::Voiced)
                || (f.manner == Manner::Trill
                    && matches!(
                        f.place,
                        ArticulationPlace::Labiodental
                            | ArticulationPlace::Palatal
                            | ArticulationPlace::Velar
                            | ArticulationPlace::Glottal
                    ))
                || (matches!(f.manner, Manner::TapFlap | Manner::LateralTapFlap)
                    && matches!(
                        f.place,
                        ArticulationPlace::Velar | ArticulationPlace::Glottal
                    ))
                || (matches!(
                    f.manner,
                    Manner::LateralFricative | Manner::LateralApproximant | Manner::LateralTapFlap
                ) && matches!(
                    f.place,
                    ArticulationPlace::Bilabial
                        | ArticulationPlace::Labiodental
                        | ArticulationPlace::Pharyngeal
                        | ArticulationPlace::Epiglottal
                        | ArticulationPlace::Glottal
                ));
            if impossible {
                errors.add(
                    "features",
                    ValidationError::new("impossible_articulation_combination"),
                );
            }
            if (f.manner == Manner::Affricate) != f.affricate_release.is_some() {
                errors.add(
                    "features",
                    ValidationError::new("affricate_requires_release_place"),
                );
            }
        }
        let (voicing, phonation) = match &self.features {
            SoundFeatures::Consonant(f) => (f.voicing, f.phonation),
            SoundFeatures::Vowel(f) => (f.voicing, f.phonation),
        };
        if phonation != Phonation::Modal && voicing != Voicing::Voiced {
            errors.add(
                "features",
                ValidationError::new("voiced_phonation_requires_voicing"),
            );
        }
        for pair in [
            (Modifier::Raised, Modifier::Lowered),
            (Modifier::Advanced, Modifier::Retracted),
            (Modifier::MoreRounded, Modifier::LessRounded),
            (Modifier::AdvancedTongueRoot, Modifier::RetractedTongueRoot),
            (Modifier::Apical, Modifier::Laminal),
        ] {
            if self.modifiers().contains(&pair.0) && self.modifiers().contains(&pair.1) {
                errors.add("modifiers", ValidationError::new("conflicting_modifiers"));
            }
        }
        let release_count = [
            Modifier::NasalRelease,
            Modifier::LateralRelease,
            Modifier::NoAudibleRelease,
        ]
        .iter()
        .filter(|m| self.modifiers().contains(m))
        .count();
        if release_count > 1 {
            errors.add(
                "modifiers",
                ValidationError::new("conflicting_release_modifiers"),
            );
        }
        if release_count > 0
            && !matches!(&self.features,
            SoundFeatures::Consonant(f) if matches!(f.manner, Manner::Plosive | Manner::LateralPlosive | Manner::Affricate))
        {
            errors.add(
                "modifiers",
                ValidationError::new("release_modifier_requires_stop"),
            );
        }
        for (i, modifier) in self.modifiers().iter().enumerate() {
            if self.modifiers()[..i].contains(modifier) {
                errors.add("modifiers", ValidationError::new("duplicate_modifier"));
            }
        }
        if self.modifiers().contains(&Modifier::Syllabic)
            && self.modifiers().contains(&Modifier::NonSyllabic)
        {
            errors.add("modifiers", ValidationError::new("conflicting_syllabicity"));
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}
