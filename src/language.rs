mod builder;
use crate::anatomy::{speaker::SpeakerAnatomy, AnatomyReport};
use crate::orthography::Orthography;
use crate::phonology::{LanguageInventory, SoundCatalog, SoundId};
use crate::prosody::ProsodyRules;
use crate::syllables::Phonotactics;
use serde::{Deserialize, Serialize};

/// Editable and deserializable input. Build performs all cross-domain checks.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageDraft {
    pub name: String,
    pub anatomy: SpeakerAnatomy,
    pub inventory: Vec<SoundId>,
    pub phonotactics: Phonotactics,
    #[serde(default)]
    pub prosody: ProsodyRules,
    pub orthography: Option<Orthography>,
}
/// No Deserialize or mutable getters: a finished profile cannot bypass validation.
#[derive(Debug, Clone)]
pub struct LanguageProfile {
    name: String,
    anatomy: SpeakerAnatomy,
    anatomy_report: AnatomyReport,
    catalog: SoundCatalog,
    inventory: LanguageInventory,
    phonotactics: Phonotactics,
    prosody: ProsodyRules,
    orthography: Option<Orthography>,
}
