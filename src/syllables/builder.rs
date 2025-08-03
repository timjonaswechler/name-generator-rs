use crate::syllables::{
    coda::CodaConfiguration, nucleus::NucleusConfiguration, onset::OnsetConfiguration,
    patterns::SyllablePattern,
};

pub struct SyllableConfiguration {
    patterns: Vec<SyllablePattern>,
    onset: OnsetConfiguration,
    nucleus: NucleusConfiguration,
    coda: CodaConfiguration,
}
