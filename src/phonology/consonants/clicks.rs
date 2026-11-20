use crate::phonology::macros::define_ipa_phoneme;
use crate::phonology::{AirflowMechanism, ArticulationPlace, Manner, Voicing};
use std::borrow::Cow;

define_ipa_phoneme! {
    name: CLICK_BILABIAL,
    id: "click_bilabial",
    ipa: "ʘ",
    consonantal: {
        airflow: AirflowMechanism::Click,
        manner: Manner::Plosive,
        place: ArticulationPlace::Bilabial,
        voicing: Voicing::Voiceless,
        modifiers: Cow::Borrowed(&[]),
    }
}

define_ipa_phoneme! {
    name: CLICK_DENTAL,
    id: "click_dental",
    ipa: "ǀ",
    consonantal: {
        airflow: AirflowMechanism::Click,
        manner: Manner::Plosive,
        place: ArticulationPlace::Dental,
        voicing: Voicing::Voiceless,
        modifiers: Cow::Borrowed(&[]),
    }
}

define_ipa_phoneme! {
    name: CLICK_ALVEOLAR,
    id: "click_alveolar",
    ipa: "ǃ",
    consonantal: {
        airflow: AirflowMechanism::Click,
        manner: Manner::Plosive,
        place: ArticulationPlace::Alveolar,
        voicing: Voicing::Voiceless,
        modifiers: Cow::Borrowed(&[]),
    }
}

define_ipa_phoneme! {
    name: CLICK_PALATOALVEOLAR,
    id: "click_palatoalveolar",
    ipa: "ǂ",
    consonantal: {
        airflow: AirflowMechanism::Click,
        manner: Manner::Plosive,
        place: ArticulationPlace::Postalveolar,
        voicing: Voicing::Voiceless,
        modifiers: Cow::Borrowed(&[]),
    }
}

define_ipa_phoneme! {
    name: CLICK_ALVEOLAR_LATERAL,
    id: "click_alveolar_lateral",
    ipa: "ǁ",
    consonantal: {
        airflow: AirflowMechanism::Click,
        manner: Manner::LateralPlosive,
        place: ArticulationPlace::Alveolar,
        voicing: Voicing::Voiceless,
        modifiers: Cow::Borrowed(&[]),
    }
}

pub const ALL_CLICKS: &[&crate::phonology::SoundDefinition] = &[
    &CLICK_BILABIAL,
    &CLICK_DENTAL,
    &CLICK_ALVEOLAR,
    &CLICK_PALATOALVEOLAR,
    &CLICK_ALVEOLAR_LATERAL,
];
