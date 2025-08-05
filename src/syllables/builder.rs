use crate::{
    syllables::{
        coda::CodaConfiguration, nucleus::NucleusConfiguration, onset::OnsetConfiguration,
        patterns::SyllablePattern,
    },
    validation::{ValidationError, ValidationErrors},
};

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
impl SyllableConfiguration {
    pub fn new(patterns: Vec<SyllablePattern>) -> Self {
        Self {
            patterns: patterns,
            onset: OnsetConfiguration::default(),
            nucleus: NucleusConfiguration::default(),
            coda: CodaConfiguration::default(),
        }
    }
    pub fn set_onset(self, onset: OnsetConfiguration) -> Result<Self, ValidationErrors> {
        let mut errors = ValidationErrors::new();

        if onset.allowed_clusters.is_empty() {
            errors.add("empty_onset_cluster", self.empty_pattern());
        }

        if errors.is_empty() {
            Ok(self)
        } else {
            Err(errors)
        }
    }
}
