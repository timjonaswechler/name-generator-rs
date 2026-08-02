mod builder;
mod errors;

use crate::anatomy::speaker::SpeakerAnatomy;
use crate::phonology::PhonologyConfiguration;
use crate::syllables::{NoCoda, NoNucleus, NoOnset, SyllableConfiguration};
use serde::{Deserialize, Serialize};
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
