use crate::phonology::macros::define_ipa_phoneme;
use crate::phonology::{AirflowMechanism, ArticulationPlace, Manner, Voicing};
use std::borrow::Cow;

define_ipa_phoneme! {
    name: P_EJECTIVE,
    id: "p_ejective",
    ipa: "pʼ",
    consonantal: {
        airflow: AirflowMechanism::Ejective,
        manner: Manner::Plosive,
        place: ArticulationPlace::Bilabial,
        voicing: Voicing::Voiceless,
        modifiers: Cow::Borrowed(&[]),
    }
}

define_ipa_phoneme! {
    name: T_EJECTIVE,
    id: "t_ejective",
    ipa: "tʼ",
    consonantal: {
        airflow: AirflowMechanism::Ejective,
        manner: Manner::Plosive,
        place: ArticulationPlace::Alveolar,
        voicing: Voicing::Voiceless,
        modifiers: Cow::Borrowed(&[]),
    }
}

define_ipa_phoneme! {
    name: K_EJECTIVE,
    id: "k_ejective",
    ipa: "kʼ",
    consonantal: {
        airflow: AirflowMechanism::Ejective,
        manner: Manner::Plosive,
        place: ArticulationPlace::Velar,
        voicing: Voicing::Voiceless,
        modifiers: Cow::Borrowed(&[]),
    }
}

define_ipa_phoneme! {
    name: S_EJECTIVE,
    id: "s_ejective",
    ipa: "sʼ",
    consonantal: {
        airflow: AirflowMechanism::Ejective,
        manner: Manner::SibilantFricative,
        place: ArticulationPlace::Alveolar,
        voicing: Voicing::Voiceless,
        modifiers: Cow::Borrowed(&[]),
    }
}

pub const ALL_EJECTIVES: &[&crate::phonology::SoundDefinition] =
    &[&P_EJECTIVE, &T_EJECTIVE, &K_EJECTIVE, &S_EJECTIVE];
