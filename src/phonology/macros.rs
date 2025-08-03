#[macro_export]
macro_rules! define_ipa_phoneme {
    // === Vocalic ===
    (
        name: $name:ident,
        ipa: $ipa:literal,
        vocalic: {
            position: $pos:expr,
            height: $height:expr,
            roundness: $round:expr,
            diacritics: $diacritics:expr $(,)?
        }
    ) => {
        #[allow(dead_code)]
        pub static $name: $crate::phonology::phonemes::Vowel =
            $crate::phonology::phonemes::Phoneme {
                ipa: std::borrow::Cow::Borrowed($ipa),
                phoneme: $crate::phonology::phonemes::VowelConfiguration {
                    position: $pos,
                    height: $height,
                    roundness: $round,
                    diacritics: $diacritics,
                },
            };
    };

    // === Consonantal ===
    (
        name: $name:ident,
        ipa: $ipa:literal,
        consonantal: {
            airflow: $airflow:expr,
            manner: $manner:expr,
            place: $place:expr,
            subplace: $subplace:expr,
            diacritics: $diacritics:expr,
            suprasegmentals: $supras:expr $(,)?
        }
    ) => {
        #[allow(dead_code)]
        pub static $name: $crate::phonology::phonemes::Consonant =
            $crate::phonology::phonemes::Phoneme {
                ipa: std::borrow::Cow::Borrowed($ipa),
                phoneme: $crate::phonology::phonemes::ConsonantConfiguration {
                    airflow: $airflow,
                    manner: $manner,
                    place: $place,
                    subplace: $subplace,
                    diacritics: $diacritics,
                    suprasegmentals: $supras,
                },
            };
    };
}
#[macro_export]
macro_rules! define_allowed_phoneme {
    ($phoneme:ident, $weight:expr) => {
        #[allow(dead_code)]
        $crate::syllables::AllowedConsonant {
            phoneme: $phoneme,
            weight: $weight,
        }
    };

    // Default weight 1.0
    ($phoneme:ident) => {
        #[allow(dead_code)]
        define_allowed_phoneme!($phoneme, 1.0)
    };
}
#[macro_export]
macro_rules! define_allowed_cluster {
    ([$($phoneme:ident),* $(,)?], $weight:expr) => {
        #[allow(dead_code)]
        $crate::syllables::AllowedConsonantCluster {
            phonemes: vec![$($phoneme),*],
            weight: $weight,
        }
    };

    // Default weight 1.0
    ([$($phoneme:ident),* $(,)?]) => {
        #[allow(dead_code)]
        define_allowed_cluster!([$($phoneme),*], 1.0)
    };
}

#[macro_export]
macro_rules! define_consonant_cluster {
    ([$($phoneme:ident),* $(,)?]) => {
        #[allow(dead_code)]
        $crate::syllables::PhonemeCluster::<$crate::phonology::phonemes::ConsonantConfiguration> {
            phonemes: vec![$($phoneme),*],
        }
    };
}

#[macro_export]
macro_rules! define_vowel_cluster {
    ([$($phoneme:ident),* $(,)?]) => {
        #[allow(dead_code)]
        $crate::syllables::PhonemeCluster::<$crate::phonology::phonemes::VowelConfiguration> {
            phonemes: vec![$($phoneme),*],
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::phonology::{
        AirflowMechanism, Manner, Place, Roundness, Subplace, TongueHeight, TonguePosition,
        EMPTY_DIACRITICS, EMPTY_SUPRASEGEMENTALS,
    };

    // Der Aufruf des Makros
    define_ipa_phoneme! {
        name: A,
        ipa: "a",
        vocalic: {
            position: TonguePosition::Front,
            height: TongueHeight::Open,
            roundness: Roundness::Unrounded,
            diacritics: EMPTY_DIACRITICS,
        }
    }

    define_ipa_phoneme! {
        name: B,
        ipa: "b",
        consonantal: {
            airflow: AirflowMechanism::Pulmonic,
            manner: Manner::Plosive,
            place: Place::Labial,
            subplace: Subplace::Bilabial,
            diacritics: EMPTY_DIACRITICS,
            suprasegmentals: EMPTY_SUPRASEGEMENTALS
        }
    }

    #[test]
    fn test_define_ipa_phoneme() {
        // Teste, ob die Phoneme korrekt definiert wurden
        assert_eq!(A.ipa, "a");
        assert_eq!(B.ipa, "b");

        // Teste die Eigenschaften der Phoneme - jetzt direkt über das phoneme Feld
        let vocalic = &A.phoneme;
        assert_eq!(vocalic.position, TonguePosition::Front);
        assert_eq!(vocalic.height, TongueHeight::Open);
        assert_eq!(vocalic.roundness, Roundness::Unrounded);
        assert!(vocalic.diacritics.is_empty());

        let consonantal = &B.phoneme;
        assert_eq!(consonantal.airflow, AirflowMechanism::Pulmonic);
        assert_eq!(consonantal.manner, Manner::Plosive);
        assert_eq!(consonantal.place, Place::Labial);
        assert_eq!(consonantal.subplace, Subplace::Bilabial);
        assert!(consonantal.diacritics.is_empty());
        assert!(consonantal.suprasegmentals.is_empty());
    }
}
