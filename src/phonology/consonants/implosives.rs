use crate::phonology::macros::define_ipa_phoneme;
use crate::phonology::{AirflowMechanism, ArticulationPlace, Manner, Voicing};
use std::borrow::Cow;

define_ipa_phoneme! {
    name: B_IMPL_BILABIAL,
    id: "b_impl_bilabial",
    ipa: "ɓ",
    consonantal: {
        airflow: AirflowMechanism::Implosive,
        manner: Manner::Plosive,
        place: ArticulationPlace::Bilabial,
        voicing: Voicing::Voiced,
        modifiers: Cow::Borrowed(&[]),
    }
}

define_ipa_phoneme! {
    name: D_IMPL_ALVEOLAR,
    id: "d_impl_alveolar",
    ipa: "ɗ",
    consonantal: {
        airflow: AirflowMechanism::Implosive,
        manner: Manner::Plosive,
        place: ArticulationPlace::Alveolar,
        voicing: Voicing::Voiced,
        modifiers: Cow::Borrowed(&[]),
    }
}

define_ipa_phoneme! {
    name: J_IMPL_PALATAL,
    id: "j_impl_palatal",
    ipa: "ʄ",
    consonantal: {
        airflow: AirflowMechanism::Implosive,
        manner: Manner::Plosive,
        place: ArticulationPlace::Palatal,
        voicing: Voicing::Voiced,
        modifiers: Cow::Borrowed(&[]),
    }
}

define_ipa_phoneme! {
    name: G_IMPL_VELAR,
    id: "g_impl_velar",
    ipa: "ɠ",
    consonantal: {
        airflow: AirflowMechanism::Implosive,
        manner: Manner::Plosive,
        place: ArticulationPlace::Velar,
        voicing: Voicing::Voiced,
        modifiers: Cow::Borrowed(&[]),
    }
}

define_ipa_phoneme! {
    name: G_IMPL_UVULAR,
    id: "g_impl_uvular",
    ipa: "ʛ",
    consonantal: {
        airflow: AirflowMechanism::Implosive,
        manner: Manner::Plosive,
        place: ArticulationPlace::Uvular,
        voicing: Voicing::Voiced,
        modifiers: Cow::Borrowed(&[]),
    }
}

pub const ALL_IMPLOSIVES: &[&crate::phonology::SoundDefinition] = &[
    &B_IMPL_BILABIAL,
    &D_IMPL_ALVEOLAR,
    &J_IMPL_PALATAL,
    &G_IMPL_VELAR,
    &G_IMPL_UVULAR,
];
