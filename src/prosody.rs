//! Segment timing, word stress and syllable tone are independent language choices.
//! IPA notation follows the Association's suprasegmental and tone sub-charts.
mod rules;
pub mod spelling;
pub use rules::{DurationRules, ProsodyRules, StressRule, ToneRules, WordEdge};

use crate::phonology::SoundId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Duration {
    ExtraShort,
    #[default]
    Normal,
    HalfLong,
    Long,
}
impl Duration {
    pub fn ipa(self) -> &'static str {
        match self {
            Self::ExtraShort => "\u{0306}",
            Self::Normal => "",
            Self::HalfLong => "ˑ",
            Self::Long => "ː",
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Segment {
    pub sound: SoundId,
    #[serde(default)]
    pub duration: Duration,
}
impl Segment {
    pub fn new(sound: SoundId, duration: Duration) -> Self {
        Self { sound, duration }
    }
}
impl From<SoundId> for Segment {
    fn from(sound: SoundId) -> Self {
        Self::new(sound, Duration::Normal)
    }
}
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Stress {
    #[default]
    Unstressed,
    Primary,
    Secondary,
}
impl Stress {
    pub fn ipa(self) -> &'static str {
        match self {
            Self::Unstressed => "",
            Self::Primary => "ˈ",
            Self::Secondary => "ˌ",
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum PitchLevel {
    ExtraLow,
    Low,
    Mid,
    High,
    ExtraHigh,
}
impl PitchLevel {
    pub fn ipa(self) -> &'static str {
        match self {
            Self::ExtraLow => "˩",
            Self::Low => "˨",
            Self::Mid => "˧",
            Self::High => "˦",
            Self::ExtraHigh => "˥",
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(try_from = "Vec<PitchLevel>", into = "Vec<PitchLevel>")]
pub struct ToneContour(Vec<PitchLevel>);
impl ToneContour {
    pub fn levels(&self) -> &[PitchLevel] {
        &self.0
    }
}
impl TryFrom<Vec<PitchLevel>> for ToneContour {
    type Error = &'static str;
    fn try_from(levels: Vec<PitchLevel>) -> Result<Self, Self::Error> {
        if levels.is_empty() {
            Err("a tone requires at least one pitch level")
        } else if levels.windows(2).any(|p| p[0] == p[1]) {
            Err("repeated pitch levels do not define a contour change")
        } else {
            Ok(Self(levels))
        }
    }
}
impl From<ToneContour> for Vec<PitchLevel> {
    fn from(contour: ToneContour) -> Self {
        contour.0
    }
}
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ToneRegister {
    #[default]
    Normal,
    Downstep,
    Upstep,
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Tone {
    pub contour: ToneContour,
    #[serde(default)]
    pub register: ToneRegister,
}
impl Tone {
    pub fn level(level: PitchLevel) -> Self {
        Self {
            contour: ToneContour(vec![level]),
            register: ToneRegister::Normal,
        }
    }
    pub fn ipa(&self) -> String {
        let mut result = match self.register {
            ToneRegister::Normal => "",
            ToneRegister::Downstep => "ꜜ",
            ToneRegister::Upstep => "ꜛ",
        }
        .to_owned();
        for level in self.contour.levels() {
            result.push_str(level.ipa());
        }
        result
    }
}
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SyllableProsody {
    #[serde(default)]
    pub stress: Stress,
    pub tone: Option<Tone>,
}

/// Explicit capabilities; enabling a language feature does not grant anatomy that ability.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProsodicCapabilities {
    pub durations: Vec<Duration>,
    pub can_stress: bool,
    pub pitch_levels: Vec<PitchLevel>,
    pub can_contour_tones: bool,
    pub can_shift_register: bool,
}
impl ProsodicCapabilities {
    pub fn human() -> Self {
        Self {
            durations: vec![
                Duration::ExtraShort,
                Duration::Normal,
                Duration::HalfLong,
                Duration::Long,
            ],
            can_stress: true,
            pitch_levels: vec![
                PitchLevel::ExtraLow,
                PitchLevel::Low,
                PitchLevel::Mid,
                PitchLevel::High,
                PitchLevel::ExtraHigh,
            ],
            can_contour_tones: true,
            can_shift_register: true,
        }
    }
}
