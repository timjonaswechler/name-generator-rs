use crate::{
    phonology::PhonologyConfiguration,
    syllables::{SyllableConfiguration, WithCoda, WithNucleus, WithOnset},
    validation::ValidationErrors,
};

impl<OnsetState, NucleusState, CodaState>
    SyllableConfiguration<OnsetState, NucleusState, CodaState>
{
    pub fn validate_against_phonology(
        &self,
        phonology: &PhonologyConfiguration,
    ) -> Result<SyllableConfiguration<WithOnset, WithNucleus, WithCoda>, ValidationErrors> {
        let mut errors = ValidationErrors::new();

        if let Err(e) = self.onset.validate_against_phonology_consonants(phonology) {
            errors.merge(e);
        }

        if let Err(e) = self.nucleus.validate_against_phonology_vowels(phonology) {
            errors.merge(e);
        }

        if let Err(e) = self.coda.validate_against_phonology_consonants(phonology) {
            errors.merge(e);
        }

        if errors.is_empty() {
            Ok(SyllableConfiguration {
                patterns: self.patterns.clone(),
                onset: self.onset.clone(),
                nucleus: self.nucleus.clone(),
                coda: self.coda.clone(),
                _onset_state: std::marker::PhantomData,
                _nucleus_state: std::marker::PhantomData,
                _coda_state: std::marker::PhantomData,
            })
        } else {
            Err(errors)
        }
    }
}
