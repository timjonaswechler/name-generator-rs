macro_rules! define_ipa_phoneme {
    (@release) => { None };
    (@release $place:expr) => { Some($place) };
    (name: $name:ident, id: $id:literal, ipa: $ipa:literal,
     consonantal: { airflow: $airflow:expr, manner: $manner:expr,
       $(release: $release:expr,)?
       place: $place:expr, voicing: $voicing:expr, modifiers: $modifiers:expr $(,)? }) => {
        pub static $name: $crate::phonology::SoundDefinition =
            $crate::phonology::SoundDefinition::builtin(
                $id,
                $ipa,
                $crate::phonology::SoundFeatures::Consonant(
                    $crate::phonology::phonemes::ConsonantFeatures {
                        airflow: $airflow,
                        manner: $manner,
                        affricate_release: $crate::phonology::macros::define_ipa_phoneme!(@release $($release)?),
                        place: $place,
                        voicing: $voicing,
                        phonation: $crate::phonology::Phonation::Modal,
                        aspiration: $crate::phonology::Aspiration::Unaspirated,
                        modifiers: $modifiers,
                    },
                ),
            );
    };
    (name: $name:ident, id: $id:literal, ipa: $ipa:literal,
     vocalic: { position: $position:expr, height: $height:expr,
       roundness: $roundness:expr, modifiers: $modifiers:expr $(,)? }) => {
        pub static $name: $crate::phonology::SoundDefinition =
            $crate::phonology::SoundDefinition::builtin(
                $id,
                $ipa,
                $crate::phonology::SoundFeatures::Vowel(
                    $crate::phonology::phonemes::VowelFeatures {
                        position: $position,
                        height: $height,
                        roundness: $roundness,
                        voicing: $crate::phonology::Voicing::Voiced,
                        phonation: $crate::phonology::Phonation::Modal,
                        modifiers: $modifiers,
                    },
                ),
            );
    };
}
pub(crate) use define_ipa_phoneme;

#[cfg(test)]
mod tests {
    use crate::phonology::{consonants::pulmonics::B_BILABIAL, vowels::A, SoundFeatures, Voicing};
    #[test]
    fn catalogue_macro_retains_explicit_identity_and_voicing() {
        assert_eq!(A.id().as_str(), "a");
        assert_eq!(A.ipa(), "a");
        let SoundFeatures::Consonant(features) = B_BILABIAL.features() else {
            panic!("expected consonant")
        };
        assert_eq!(features.voicing, Voicing::Voiced);
        assert_eq!(B_BILABIAL.ipa(), "b");
    }
}
