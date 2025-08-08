mod errors;

use serde::{Deserialize, Serialize};

use crate::anatomy::speaker::SpeakerAnatomy;
use crate::phonology::PhonologyConfiguration;

use crate::syllables::{
    self, NoCoda, NoNucleus, NoOnset, SyllableConfiguration, WithCoda, WithNucleus, WithOnset,
};
use crate::validation::{ValidationError, ValidationErrors};

use std::marker::PhantomData;

pub struct NotInitialized;
#[derive(Debug)]
pub struct Initializing;
pub struct Initialized;

#[derive(Serialize, Deserialize, Debug)]
pub struct LanguageConfiguration<
    OnsetState = NoOnset,
    NucleusState = NoNucleus,
    CodaState = NoCoda,
    State = NotInitialized,
> {
    pub name: String,
    speaker_anatomy: SpeakerAnatomy,
    phonology: PhonologyConfiguration,
    syllables: SyllableConfiguration<OnsetState, NucleusState, CodaState>,
    _state: PhantomData<State>,
}

impl LanguageConfiguration<NoOnset, NoNucleus, NoCoda, NotInitialized> {
    pub fn new(
        name: impl Into<String>,
    ) -> LanguageConfiguration<NoOnset, NoNucleus, NoCoda, Initializing> {
        LanguageConfiguration {
            name: name.into(),
            speaker_anatomy: SpeakerAnatomy::default(),
            phonology: PhonologyConfiguration::new(),
            syllables: SyllableConfiguration::default(),
            _state: PhantomData,
        }
    }
}

impl<OnsetState, NucleusState, CodaState>
    LanguageConfiguration<OnsetState, NucleusState, CodaState, Initializing>
{
    #[must_use]
    pub fn set_anatomy(
        mut self,
        anatomy: SpeakerAnatomy,
    ) -> Result<
        LanguageConfiguration<OnsetState, NucleusState, CodaState, Initializing>,
        ValidationErrors,
    > {
        //Validate it the anatomy is possible
        let mut errors = ValidationErrors::new();

        if let Err(e) = anatomy.validate_anatomical_consistency() {
            errors.merge(e);
        }

        self.speaker_anatomy = anatomy;

        if errors.is_empty() {
            Ok(LanguageConfiguration {
                name: self.name,
                speaker_anatomy: self.speaker_anatomy,
                phonology: self.phonology,
                syllables: self.syllables,
                _state: PhantomData,
            })
        } else {
            Err(errors)
        }
    }

    #[must_use]
    pub fn set_phonology(
        mut self,
        phonology: PhonologyConfiguration,
    ) -> Result<
        LanguageConfiguration<OnsetState, NucleusState, CodaState, Initializing>,
        ValidationErrors,
    > {
        // Validate the phonology is possible with the anatomy
        let mut errors = ValidationErrors::new();

        if let Err(e) = phonology.validate_against_anatomy(&self.speaker_anatomy) {
            errors.merge(e);
        }

        if phonology.vowels().len() < 3 {
            errors.add(
                "no_enough_vowels",
                ValidationError::new("phonology_not_enough_vowels")
                    .with_message("The phonology must have at least three vowels."),
            );
        }

        if errors.is_empty() {
            self.phonology = phonology;
            Ok(LanguageConfiguration {
                name: self.name,
                speaker_anatomy: self.speaker_anatomy,
                phonology: self.phonology,
                syllables: self.syllables,
                _state: PhantomData,
            })
        } else {
            Err(errors)
        }
    }

    #[must_use]
    pub fn set_syllables(
        self,
        syllables: SyllableConfiguration<WithOnset, WithNucleus, WithCoda>,
    ) -> Result<
        LanguageConfiguration<WithOnset, WithNucleus, WithCoda, Initializing>,
        ValidationErrors,
    > {
        let mut errors = ValidationErrors::new();

        match syllables.validate_against_phonology(&self.phonology) {
            Ok(_) => Ok(LanguageConfiguration {
                name: self.name,
                speaker_anatomy: self.speaker_anatomy,
                phonology: self.phonology,
                syllables: syllables,
                _state: PhantomData,
            }),
            Err(e) => {
                errors.merge(e);
                Err(errors)
            }
        }
    }
}

impl LanguageConfiguration<WithOnset, WithNucleus, WithCoda, Initialized> {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn anatomy(&self) -> &SpeakerAnatomy {
        &self.speaker_anatomy
    }

    pub fn phonology(&self) -> &PhonologyConfiguration {
        &self.phonology
    }

    pub fn syllables(&self) -> &SyllableConfiguration<WithOnset, WithNucleus, WithCoda> {
        &self.syllables
    }
}
