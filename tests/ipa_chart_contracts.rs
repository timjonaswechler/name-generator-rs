//! Reference fixtures transcribed independently from the official IPA sub-charts:
//! https://www.internationalphoneticassociation.org/content/ipa-chart
//! Chart marked 2026, revised to 2015/2005. These check notation/feature coverage,
//! not empirical physiological accuracy. Blank and shaded cells are not invented sounds.
use name_generator::anatomy::{
    production::{ProductionAssessment, Requirement},
    speaker::{ContactCapability, SpeakerAnatomy, TeethConfiguration},
};
use name_generator::phonology::{diacritics::ALL_DIACRITICS, *};
use std::borrow::Cow;

// Columns are place, chart row, voiceless symbol, voiced symbol.
const PULMONIC_CELLS: &[(ArticulationPlace, Manner, &str, &str)] = &[
    (ArticulationPlace::Bilabial, Manner::Plosive, "p", "b"),
    (ArticulationPlace::Alveolar, Manner::Plosive, "t", "d"),
    (ArticulationPlace::Retroflex, Manner::Plosive, "ʈ", "ɖ"),
    (ArticulationPlace::Palatal, Manner::Plosive, "c", "ɟ"),
    (ArticulationPlace::Velar, Manner::Plosive, "k", "ɡ"),
    (ArticulationPlace::Uvular, Manner::Plosive, "q", "ɢ"),
    (ArticulationPlace::Glottal, Manner::Plosive, "ʔ", ""),
    (ArticulationPlace::Bilabial, Manner::Nasal, "", "m"),
    (ArticulationPlace::Labiodental, Manner::Nasal, "", "ɱ"),
    (ArticulationPlace::Alveolar, Manner::Nasal, "", "n"),
    (ArticulationPlace::Retroflex, Manner::Nasal, "", "ɳ"),
    (ArticulationPlace::Palatal, Manner::Nasal, "", "ɲ"),
    (ArticulationPlace::Velar, Manner::Nasal, "", "ŋ"),
    (ArticulationPlace::Uvular, Manner::Nasal, "", "ɴ"),
    (ArticulationPlace::Bilabial, Manner::Trill, "", "ʙ"),
    (ArticulationPlace::Alveolar, Manner::Trill, "", "r"),
    (ArticulationPlace::Uvular, Manner::Trill, "", "ʀ"),
    (ArticulationPlace::Labiodental, Manner::TapFlap, "", "ⱱ"),
    (ArticulationPlace::Alveolar, Manner::TapFlap, "", "ɾ"),
    (ArticulationPlace::Retroflex, Manner::TapFlap, "", "ɽ"),
    (ArticulationPlace::Bilabial, Manner::Fricative, "ɸ", "β"),
    (ArticulationPlace::Labiodental, Manner::Fricative, "f", "v"),
    (ArticulationPlace::Dental, Manner::Fricative, "θ", "ð"),
    (ArticulationPlace::Alveolar, Manner::Fricative, "s", "z"),
    (ArticulationPlace::Postalveolar, Manner::Fricative, "ʃ", "ʒ"),
    (ArticulationPlace::Retroflex, Manner::Fricative, "ʂ", "ʐ"),
    (ArticulationPlace::Palatal, Manner::Fricative, "ç", "ʝ"),
    (ArticulationPlace::Velar, Manner::Fricative, "x", "ɣ"),
    (ArticulationPlace::Uvular, Manner::Fricative, "χ", "ʁ"),
    (ArticulationPlace::Pharyngeal, Manner::Fricative, "ħ", "ʕ"),
    (ArticulationPlace::Glottal, Manner::Fricative, "h", "ɦ"),
    (
        ArticulationPlace::Alveolar,
        Manner::LateralFricative,
        "ɬ",
        "ɮ",
    ),
    (ArticulationPlace::Labiodental, Manner::Approximant, "", "ʋ"),
    (ArticulationPlace::Alveolar, Manner::Approximant, "", "ɹ"),
    (ArticulationPlace::Retroflex, Manner::Approximant, "", "ɻ"),
    (ArticulationPlace::Palatal, Manner::Approximant, "", "j"),
    (ArticulationPlace::Velar, Manner::Approximant, "", "ɰ"),
    (
        ArticulationPlace::Alveolar,
        Manner::LateralApproximant,
        "",
        "l",
    ),
    (
        ArticulationPlace::Retroflex,
        Manner::LateralApproximant,
        "",
        "ɭ",
    ),
    (
        ArticulationPlace::Palatal,
        Manner::LateralApproximant,
        "",
        "ʎ",
    ),
    (
        ArticulationPlace::Velar,
        Manner::LateralApproximant,
        "",
        "ʟ",
    ),
];

#[test]
fn every_printed_pulmonic_symbol_has_the_chart_place_manner_and_voicing() {
    let catalog = SoundCatalog::builtin();
    let mut checked = 0;
    for &(place, manner, voiceless, voiced) in PULMONIC_CELLS {
        for (ipa, voice) in [(voiceless, Voicing::Voiceless), (voiced, Voicing::Voiced)] {
            if ipa.is_empty() {
                continue;
            }
            let sound = catalog
                .find_ipa(ipa)
                .unwrap_or_else(|| panic!("missing chart symbol {ipa}"));
            let SoundFeatures::Consonant(f) = sound.features() else {
                panic!("not a consonant: {ipa}")
            };
            assert_eq!(f.place, place, "{ipa}");
            assert_eq!(f.voicing, voice, "{ipa}");
            assert_eq!(f.airflow, AirflowMechanism::Pulmonic, "{ipa}");
            if manner == Manner::Fricative {
                assert!(
                    matches!(
                        f.manner,
                        Manner::Fricative
                            | Manner::SibilantFricative
                            | Manner::NonSibilantFricative
                    ),
                    "{ipa}"
                );
            } else {
                assert_eq!(f.manner, manner, "{ipa}");
            }
            checked += 1;
        }
    }
    assert_eq!(checked, 59);
}

#[test]
fn every_printed_vowel_has_the_chart_quality() {
    use Roundness::{Rounded as R, Unrounded as U};
    use TongueHeight::*;
    use TonguePosition::*;
    let reference = [
        ("i", Close, Front, U),
        ("y", Close, Front, R),
        ("ɨ", Close, Central, U),
        ("ʉ", Close, Central, R),
        ("ɯ", Close, Back, U),
        ("u", Close, Back, R),
        ("ɪ", NearClose, NearFront, U),
        ("ʏ", NearClose, NearFront, R),
        ("ʊ", NearClose, NearBack, R),
        ("e", CloseMid, Front, U),
        ("ø", CloseMid, Front, R),
        ("ɘ", CloseMid, Central, U),
        ("ɵ", CloseMid, Central, R),
        ("ɤ", CloseMid, Back, U),
        ("o", CloseMid, Back, R),
        ("ə", Mid, Central, U),
        ("ɛ", OpenMid, Front, U),
        ("œ", OpenMid, Front, R),
        ("ɜ", OpenMid, Central, U),
        ("ɞ", OpenMid, Central, R),
        ("ʌ", OpenMid, Back, U),
        ("ɔ", OpenMid, Back, R),
        ("æ", NearOpen, Front, U),
        ("ɐ", NearOpen, Central, U),
        ("a", Open, Front, U),
        ("ɶ", Open, Front, R),
        ("ɑ", Open, Back, U),
        ("ɒ", Open, Back, R),
    ];
    let catalog = SoundCatalog::builtin();
    for (ipa, height, position, roundness) in reference {
        let sound = catalog
            .find_ipa(ipa)
            .unwrap_or_else(|| panic!("missing vowel {ipa}"));
        let SoundFeatures::Vowel(f) = sound.features() else {
            panic!("not a vowel: {ipa}")
        };
        assert_eq!(
            (f.height, f.position, f.roundness),
            (height, position, roundness),
            "{ipa}"
        );
    }
}

#[test]
fn non_pulmonic_other_symbols_and_chart_compositions_are_available() {
    let catalog = SoundCatalog::builtin();
    for (ipa, airflow, place) in [
        ("ʘ", AirflowMechanism::Click, ArticulationPlace::Bilabial),
        ("ǀ", AirflowMechanism::Click, ArticulationPlace::Dental),
        ("ǃ", AirflowMechanism::Click, ArticulationPlace::Alveolar),
        (
            "ǂ",
            AirflowMechanism::Click,
            ArticulationPlace::Postalveolar,
        ),
        ("ǁ", AirflowMechanism::Click, ArticulationPlace::Alveolar),
        (
            "ɓ",
            AirflowMechanism::Implosive,
            ArticulationPlace::Bilabial,
        ),
        (
            "ɗ",
            AirflowMechanism::Implosive,
            ArticulationPlace::Alveolar,
        ),
        ("ʄ", AirflowMechanism::Implosive, ArticulationPlace::Palatal),
        ("ɠ", AirflowMechanism::Implosive, ArticulationPlace::Velar),
        ("ʛ", AirflowMechanism::Implosive, ArticulationPlace::Uvular),
        (
            "pʼ",
            AirflowMechanism::Ejective,
            ArticulationPlace::Bilabial,
        ),
        (
            "tʼ",
            AirflowMechanism::Ejective,
            ArticulationPlace::Alveolar,
        ),
        ("kʼ", AirflowMechanism::Ejective, ArticulationPlace::Velar),
        (
            "sʼ",
            AirflowMechanism::Ejective,
            ArticulationPlace::Alveolar,
        ),
    ] {
        let SoundFeatures::Consonant(f) = catalog.find_ipa(ipa).unwrap().features() else {
            panic!("{ipa}")
        };
        assert_eq!((f.airflow, f.place), (airflow, place), "{ipa}");
        if airflow == AirflowMechanism::Implosive {
            assert_eq!(f.voicing, Voicing::Voiced);
        }
    }
    for (ipa, place) in [
        ("ʍ", ArticulationPlace::LabialVelar),
        ("w", ArticulationPlace::LabialVelar),
        ("ɥ", ArticulationPlace::LabialPalatal),
        ("ʜ", ArticulationPlace::Epiglottal),
        ("ʢ", ArticulationPlace::Epiglottal),
        ("ʡ", ArticulationPlace::Epiglottal),
        ("ɕ", ArticulationPlace::AlveoloPalatal),
        ("ʑ", ArticulationPlace::AlveoloPalatal),
        ("ɺ", ArticulationPlace::Alveolar),
        ("ɧ", ArticulationPlace::PostalveolarVelar),
    ] {
        let SoundFeatures::Consonant(f) = catalog.find_ipa(ipa).unwrap().features() else {
            panic!("{ipa}")
        };
        assert_eq!(f.place, place, "{ipa}");
    }
    let SoundFeatures::Consonant(affricate) = catalog.find_ipa("t͡s").unwrap().features() else {
        unreachable!()
    };
    assert_eq!(affricate.manner, Manner::Affricate);
    assert_eq!(
        affricate.affricate_release,
        Some(ArticulationPlace::Alveolar)
    );
    let SoundFeatures::Consonant(double) = catalog.find_ipa("k͡p").unwrap().features() else {
        unreachable!()
    };
    assert_eq!(double.place, ArticulationPlace::LabialVelar);
    assert_eq!(double.manner, Manner::Plosive);
}

#[test]
fn readable_names_follow_features_not_legacy_constant_spelling() {
    let catalog = SoundCatalog::builtin();
    assert_eq!(
        catalog.find_ipa("p").unwrap().name(),
        "voiceless bilabial plosive"
    );
    assert_eq!(
        catalog.find_ipa("a").unwrap().name(),
        "voiced open front unrounded vowel"
    );
    assert_eq!(
        catalog.find_ipa("ʡ").unwrap().name(),
        "voiceless epiglottal plosive"
    );
    for ipa in ["e̞", "ø̞", "ɤ̞", "o̞", "ʔ̞"] {
        let sound = catalog.find_ipa(ipa).unwrap();
        assert!(sound.modifiers().contains(&Modifier::Lowered), "{ipa}");
        assert!(!sound.modifiers().contains(&Modifier::Raised), "{ipa}");
        assert!(sound.description().contains(ipa));
    }
}

#[test]
fn every_chart_diacritic_has_a_named_typed_representation() {
    let expected = [
        "̥", "̬", "ʰ", "̹", "̜", "̟", "̠", "̈", "̽", "̩", "̯", "˞", "̤", "̰", "̼", "ʷ", "ʲ", "ˠ", "ˤ", "̴", "̝",
        "̞", "̘", "̙", "̪", "̺", "̻", "̃", "ⁿ", "ˡ", "̚",
    ];
    assert_eq!(ALL_DIACRITICS.len(), expected.len());
    for symbol in expected {
        let matches: Vec<_> = ALL_DIACRITICS
            .iter()
            .filter(|d| d.ipa() == symbol)
            .collect();
        assert_eq!(matches.len(), 1, "{symbol}");
        assert!(!matches[0].name().is_empty());
    }
}

#[test]
fn every_builtin_sound_has_resolved_requirements_for_the_reference_human_model() {
    let human = SpeakerAnatomy::human();
    for sound in SoundCatalog::builtin().iter() {
        assert_eq!(
            human.assess_sound(sound).unwrap(),
            ProductionAssessment::Possible,
            "{}",
            sound.description()
        );
    }
}

#[test]
fn specialized_articulations_require_their_own_capabilities() {
    let catalog = SoundCatalog::builtin();
    let mut no_trills = SpeakerAnatomy::human();
    no_trills.articulation.trills.clear();
    let mut no_taps = SpeakerAnatomy::human();
    no_taps.articulation.taps.clear();
    let mut no_affricates = SpeakerAnatomy::human();
    no_affricates.articulation.can_coordinate_affricates = false;
    let mut no_double = SpeakerAnatomy::human();
    no_double.articulation.can_double_articulate = false;
    let mut no_epiglottals = SpeakerAnatomy::human();
    no_epiglottals.articulation.can_articulate_epiglottals = false;
    let mut no_laterals = SpeakerAnatomy::human();
    no_laterals.tongue.can_perform_lateral_release = false;
    for (anatomy, symbols) in [
        (no_trills, vec!["ʙ", "r", "ʀ"]),
        (no_taps, vec!["ⱱ", "ɾ", "ɽ", "ɺ"]),
        (no_affricates, vec!["t͡s", "d͡ʒ"]),
        (no_double, vec!["w", "ɥ", "ɧ", "k͡p"]),
        (no_epiglottals, vec!["ʜ", "ʢ", "ʡ"]),
        (no_laterals, vec!["ɬ", "ɮ", "l", "ɺ", "ǁ"]),
    ] {
        for symbol in symbols {
            assert!(
                matches!(
                    anatomy
                        .assess_sound(catalog.find_ipa(symbol).unwrap())
                        .unwrap(),
                    ProductionAssessment::Impossible(_)
                ),
                "{symbol}"
            );
        }
    }
}

#[test]
fn affricate_release_requirements_and_contact_overrides_are_not_skipped() {
    let catalog = SoundCatalog::builtin();
    let mut anatomy = SpeakerAnatomy::human();
    anatomy.oral_cavity.has_hard_palate = false;
    assert_eq!(
        anatomy
            .assess_sound(catalog.find_ipa("d͡ʒ").unwrap())
            .unwrap(),
        ProductionAssessment::Impossible(vec![Requirement::HardPalate])
    );
    let anatomy = SpeakerAnatomy::human()
        .teeth(TeethConfiguration::Fangs)
        .labiodental_contact(ContactCapability::Available)
        .dental_contact(ContactCapability::Unavailable);
    assert_eq!(
        anatomy
            .assess_sound(catalog.find_ipa("f").unwrap())
            .unwrap(),
        ProductionAssessment::Possible
    );
    assert_eq!(
        anatomy
            .assess_sound(catalog.find_ipa("θ").unwrap())
            .unwrap(),
        ProductionAssessment::Impossible(vec![Requirement::DentalContact])
    );
}

#[test]
fn known_impossible_and_conflicting_feature_combinations_are_rejected() {
    let catalog = SoundCatalog::builtin();
    for (place, manner, voicing) in [
        (ArticulationPlace::Glottal, Manner::Nasal, Voicing::Voiced),
        (ArticulationPlace::Glottal, Manner::Plosive, Voicing::Voiced),
        (ArticulationPlace::Palatal, Manner::Trill, Voicing::Voiced),
        (
            ArticulationPlace::Bilabial,
            Manner::LateralFricative,
            Voicing::Voiceless,
        ),
    ] {
        let SoundFeatures::Consonant(mut f) = catalog.find_ipa("p").unwrap().features().clone()
        else {
            unreachable!()
        };
        f.place = place;
        f.manner = manner;
        f.voicing = voicing;
        assert!(SoundDefinition::new(
            SoundId::from_static("invalid"),
            "test".into(),
            SoundFeatures::Consonant(f)
        )
        .is_err());
    }
    let SoundFeatures::Consonant(mut f) = catalog.find_ipa("t").unwrap().features().clone() else {
        unreachable!()
    };
    f.manner = Manner::Affricate;
    assert!(SoundDefinition::new(
        SoundId::from_static("invalid"),
        "test".into(),
        SoundFeatures::Consonant(f.clone())
    )
    .is_err());
    f.manner = Manner::Plosive;
    f.modifiers = Cow::Borrowed(&[Modifier::Raised, Modifier::Lowered]);
    assert!(SoundDefinition::new(
        SoundId::from_static("invalid"),
        "test".into(),
        SoundFeatures::Consonant(f)
    )
    .is_err());
}
