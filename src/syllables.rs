pub mod builder;
pub(crate) mod coda;
mod errors;
pub(crate) mod nucleus;
pub(crate) mod onset;
pub mod patterns;

use crate::phonology::phonemes::PhonemeCluster;
use crate::syllables::patterns::SyllablePattern;
use crate::syllables::{
    coda::CodaConfiguration, nucleus::NucleusConfiguration, onset::OnsetConfiguration,
};
use serde::{Deserialize, Serialize};

// State Marker
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct NoOnset;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct WithOnset;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct NoNucleus;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct WithNucleus;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct NoCoda;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct WithCoda;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SyllableConfiguration<OnsetState = NoOnset, NucleusState = NoNucleus, CodaState = NoCoda>
{
    pub patterns: Vec<SyllablePattern>,
    pub onset: OnsetConfiguration,
    pub nucleus: NucleusConfiguration,
    pub coda: CodaConfiguration,
    _onset_state: std::marker::PhantomData<OnsetState>,
    _nucleus_state: std::marker::PhantomData<NucleusState>,
    _coda_state: std::marker::PhantomData<CodaState>,
}

/// Represents a complete syllable with all its components
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Syllable {
    /// Optional onset cluster
    pub onset: Option<PhonemeCluster>,
    /// Required nucleus (vowel core)
    pub nucleus: PhonemeCluster,
    /// Optional coda cluster
    pub coda: Option<PhonemeCluster>,
}
