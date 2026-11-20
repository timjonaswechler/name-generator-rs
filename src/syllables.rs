//! Sound-combination rules, distinct from sound production and written spelling.
mod builder;
pub mod patterns;
use crate::phonology::SoundId;
use crate::prosody::{Segment, SyllableProsody};
use crate::weights::Weight;
use patterns::SyllablePattern;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Presence {
    Forbidden,
    Optional,
    Required,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WordPosition {
    Any,
    InitialOnly,
    FinalOnly,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SequenceRule {
    pub sounds: Vec<SoundId>,
    pub weight: Weight,
    pub position: WordPosition,
}
impl SequenceRule {
    pub fn new(sounds: Vec<SoundId>) -> Self {
        Self {
            sounds,
            weight: Weight::ONE,
            position: WordPosition::Any,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PositionRules {
    pub presence: Presence,
    pub choices: Vec<SequenceRule>,
}
impl PositionRules {
    pub fn forbidden() -> Self {
        Self {
            presence: Presence::Forbidden,
            choices: vec![],
        }
    }
    pub fn required(choices: Vec<SequenceRule>) -> Self {
        Self {
            presence: Presence::Required,
            choices,
        }
    }
    pub fn optional(choices: Vec<SequenceRule>) -> Self {
        Self {
            presence: Presence::Optional,
            choices,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WeightedPattern {
    pub pattern: SyllablePattern,
    pub weight: Weight,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ForbiddenBoundaryPair {
    pub left: SoundId,
    pub right: SoundId,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Phonotactics {
    pub onset: PositionRules,
    pub nucleus: PositionRules,
    pub coda: PositionRules,
    pub patterns: Vec<WeightedPattern>,
    pub forbidden_boundaries: Vec<ForbiddenBoundaryPair>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Syllable {
    pub onset: Vec<Segment>,
    pub nucleus: Vec<Segment>,
    pub coda: Vec<Segment>,
    #[serde(default)]
    pub prosody: SyllableProsody,
}
impl Syllable {
    pub fn new(onset: Vec<SoundId>, nucleus: Vec<SoundId>, coda: Vec<SoundId>) -> Self {
        Self {
            onset: onset.into_iter().map(Segment::from).collect(),
            nucleus: nucleus.into_iter().map(Segment::from).collect(),
            coda: coda.into_iter().map(Segment::from).collect(),
            prosody: SyllableProsody::default(),
        }
    }
    pub fn segments(&self) -> impl DoubleEndedIterator<Item = &Segment> {
        self.onset.iter().chain(&self.nucleus).chain(&self.coda)
    }
    pub fn sounds(&self) -> impl DoubleEndedIterator<Item = &SoundId> {
        self.segments().map(|segment| &segment.sound)
    }
}
/// Constructed only after checking a complete word against a language profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WordForm {
    pub(crate) syllables: Vec<Syllable>,
}
impl WordForm {
    pub fn syllables(&self) -> &[Syllable] {
        &self.syllables
    }
    pub fn sounds(&self) -> impl Iterator<Item = &SoundId> {
        self.syllables.iter().flat_map(Syllable::sounds)
    }
}
