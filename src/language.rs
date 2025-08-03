use serde::{Deserialize, Serialize};

use crate::anatomy::speaker::SpeakerAnatomy;
use crate::phonology::phonemes::{Consonant, Vowel};
use crate::phonology::PhonologyConfiguration;
use crate::syllables::SyllableConfiguration;
use std::borrow::Cow;
use std::marker::PhantomData;

pub struct NotInitialized;
pub struct Initialized;
pub struct HasPhonology;
pub struct HasAnatomy;

#[derive(Serialize, Deserialize)]
pub struct LanguageConfiguration<State = NotInitialized> {
    pub name: String,
    speaker_anatomy: SpeakerAnatomy,
    phonology: PhonologyConfiguration,
    syllables: SyllableConfiguration,
    _state: PhantomData<State>,
}

impl LanguageConfiguration<NotInitialized> {
    pub fn new(name: impl Into<String>) -> LanguageConfiguration<Initialized> {
        LanguageConfiguration {
            name: name.into(),
            speaker_anatomy: SpeakerAnatomy::default(),
            phonology: PhonologyConfiguration::new(),
            syllables: SyllableConfiguration::default(),
            _state: PhantomData,
        }
    }
}
impl LanguageConfiguration<Initialized> {
    pub fn set_anatomy(mut self, anatomy: SpeakerAnatomy) -> LanguageConfiguration<HasAnatomy> {
        //Validate it the anatomy is possible
        self.speaker_anatomy = anatomy;
        LanguageConfiguration {
            name: self.name,
            speaker_anatomy: self.speaker_anatomy,
            phonology: self.phonology,
            syllables: self.syllables,
            _state: PhantomData,
        }
    }
}
impl LanguageConfiguration<HasAnatomy> {
    pub fn set_phonology(
        mut self,
        phonology: PhonologyConfiguration,
    ) -> Result<LanguageConfiguration<HasPhonology>, Box<dyn std::error::Error>> {
        // Validate the phonology is possible with the anatomy
        phonology.validate_against_anatomy(&self.speaker_anatomy)?;
        self.phonology = phonology;
        Ok(LanguageConfiguration {
            name: self.name,
            speaker_anatomy: self.speaker_anatomy,
            phonology: self.phonology,
            syllables: self.syllables,
            _state: PhantomData,
        })
    }
    pub fn add_consonant(mut self, consonant: &'static Consonant) -> Self {
        self.phonology.consonants.push(Cow::Borrowed(consonant));
        self
    }

    pub fn add_vowel(mut self, vowel: &'static Vowel) -> Self {
        self.phonology.vowels.push(Cow::Borrowed(vowel));
        self
    }
}
impl<State> LanguageConfiguration<State> {
    pub fn set_syllables(mut self, syllables: SyllableConfiguration) -> Self {
        // Validate the syllable structure is possible with the phonology
        // is there any phoneme missing
        // is the syllable config valid (e.g. no empty syllables, consonant not in nucleus, etc.)

        self.syllables = syllables;
        LanguageConfiguration {
            name: self.name,
            speaker_anatomy: self.speaker_anatomy,
            phonology: self.phonology,
            syllables: self.syllables,
            _state: PhantomData,
        }
    }

    pub fn phonology(&self) -> &PhonologyConfiguration {
        &self.phonology
    }
    pub fn anatomy(&self) -> &SpeakerAnatomy {
        &self.speaker_anatomy
    }
    pub fn syllables(&self) -> &SyllableConfiguration {
        &self.syllables
    }
}

pub(crate) enum Format {
    Lua,
    Json,
    Unknown,
}
