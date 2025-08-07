use crate::{
    phonology::phonemes::{AllowedCluster, AllowedDiphthong, AllowedPhoneme, AllowedTriphthong},
    syllables::{
        coda::CodaConfiguration, nucleus::NucleusConfiguration, onset::OnsetConfiguration,
        patterns::SyllablePattern, NoCoda, NoNucleus, NoOnset, SyllableConfiguration, WithCoda,
        WithNucleus, WithOnset,
    },
    validation::ValidationErrors,
};

impl SyllableConfiguration<NoOnset, NoNucleus, NoCoda> {
    pub fn new() -> Self {
        Self {
            patterns: Vec::new(),
            onset: OnsetConfiguration::default(),
            nucleus: NucleusConfiguration::default(),
            coda: CodaConfiguration::default(),
            _onset_state: std::marker::PhantomData,
            _nucleus_state: std::marker::PhantomData,
            _coda_state: std::marker::PhantomData,
        }
    }
}

// Onset nur wenn noch nicht gesetzt
impl<N, C> SyllableConfiguration<NoOnset, N, C> {
    pub fn set_onset(
        self,
        allowed_phonemes: Vec<AllowedPhoneme>,
        allowed_clusters: Vec<AllowedCluster>,
        word_initial_only: Vec<AllowedCluster>,
    ) -> Result<SyllableConfiguration<WithOnset, N, C>, ValidationErrors> {
        let mut errors = ValidationErrors::new();
        let mut onset =
            OnsetConfiguration::new(allowed_phonemes, allowed_clusters, word_initial_only);

        match onset.validate() {
            Ok(onset) => Ok(SyllableConfiguration {
                patterns: self.patterns,
                onset: onset,
                nucleus: self.nucleus,
                coda: self.coda,
                _onset_state: std::marker::PhantomData,
                _nucleus_state: std::marker::PhantomData,
                _coda_state: std::marker::PhantomData,
            }),
            Err(e) => {
                errors.merge(e);
                Err(errors)
            }
        }
    }
}

// Nucleus nur wenn noch nicht gesetzt
impl<O, C> SyllableConfiguration<O, NoNucleus, C> {
    pub fn set_nucleus(
        self,
        allowed_phonemes: Vec<AllowedPhoneme>,
        allowed_diphthongs: Vec<AllowedDiphthong>,
        allowed_triphthongs: Vec<AllowedTriphthong>,
        word_initial_only: Vec<AllowedCluster>, // Nur am Wortanfang erlaubt
        word_final_only: Vec<AllowedCluster>,
    ) -> Result<SyllableConfiguration<O, WithNucleus, C>, ValidationErrors> {
        let mut errors = ValidationErrors::new();
        let mut nucleus = NucleusConfiguration::new(
            allowed_phonemes,
            allowed_diphthongs,
            allowed_triphthongs,
            word_initial_only,
            word_final_only,
        );
        match nucleus.validate() {
            Ok(nucleus) => Ok(SyllableConfiguration {
                patterns: self.patterns,
                onset: self.onset,
                nucleus,
                coda: self.coda,
                _onset_state: std::marker::PhantomData,
                _nucleus_state: std::marker::PhantomData,
                _coda_state: std::marker::PhantomData,
            }),
            Err(e) => {
                errors.merge(e);
                Err(errors)
            }
        }
    }
}

// Coda nur wenn noch nicht gesetzt
impl<O, N> SyllableConfiguration<O, N, NoCoda> {
    pub fn set_coda(
        self,
        allowed_phonemes: Vec<AllowedPhoneme>,
        allowed_clusters: Vec<AllowedCluster>,
        word_final_only: Vec<AllowedCluster>,
    ) -> Result<SyllableConfiguration<O, N, WithCoda>, ValidationErrors> {
        let mut errors = ValidationErrors::new();
        let mut coda = CodaConfiguration::new(allowed_phonemes, allowed_clusters, word_final_only);

        match coda.validate() {
            Ok(coda) => Ok(SyllableConfiguration {
                patterns: self.patterns,
                onset: self.onset,
                nucleus: self.nucleus,
                coda,
                _onset_state: std::marker::PhantomData,
                _nucleus_state: std::marker::PhantomData,
                _coda_state: std::marker::PhantomData,
            }),
            Err(e) => {
                errors.merge(e);
                Err(errors)
            }
        }
    }
}

// add_pattern ist für alle States verfügbar
impl<O, N, C> SyllableConfiguration<O, N, C> {
    pub fn add_pattern(mut self, pattern: &str, weight: f32) -> Result<Self, ValidationErrors> {
        let mut errors = ValidationErrors::new();

        let mut new_pattern = SyllablePattern::new(pattern, weight);

        if let Err(e) = new_pattern.validate_pattern() {
            errors.merge(e);
        }

        if errors.is_empty() {
            self.patterns.push(new_pattern.parse());
            Ok(self)
        } else {
            Err(errors)
        }
    }
}

impl Default for SyllableConfiguration<NoOnset, NoNucleus, NoCoda> {
    fn default() -> Self {
        Self::new()
    }
}
