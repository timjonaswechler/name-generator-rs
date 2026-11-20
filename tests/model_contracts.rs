use name_generator::anatomy::{
    production::{
        PossibleSoundSet, ProductionAssessment, ProductionRequirements, Requirement, SupportGap,
    },
    speaker::{
        LipControl, SpeakerAnatomy, TeethConfiguration, TonguePartControl, VelicPortControl,
    },
};
use name_generator::language::{LanguageDraft, LanguageProfile};
use name_generator::orthography::Orthography;
use name_generator::phonology::{
    consonants::pulmonics::{
        B_BILABIAL, F, H, M_BILABIAL, M_BILABIAL_VOICELESS, N_ALVEOLAR, P_BILABIAL, S_POSTALVEOLAR,
        T_ALVEOLAR,
    },
    phonemes::ConsonantFeatures,
    vowels::{A, I, U},
    *,
};
use name_generator::syllables::{patterns::SyllablePattern, *};
use name_generator::weights::Weight;
use std::{borrow::Cow, collections::BTreeMap};

fn id(sound: &SoundDefinition) -> SoundId {
    sound.id().clone()
}
fn sequence(sound: &SoundDefinition) -> SequenceRule {
    SequenceRule::new(vec![id(sound)])
}
fn pattern(text: &str) -> WeightedPattern {
    WeightedPattern {
        pattern: SyllablePattern::parse(text).unwrap(),
        weight: Weight::ONE,
    }
}
fn cv(onsets: &[&SoundDefinition]) -> LanguageDraft {
    LanguageDraft {
        name: "Example".into(),
        anatomy: SpeakerAnatomy::human(),
        inventory: onsets.iter().map(|s| id(s)).chain([id(&A)]).collect(),
        phonotactics: Phonotactics {
            onset: PositionRules::required(onsets.iter().map(|s| sequence(s)).collect()),
            nucleus: PositionRules::required(vec![sequence(&A)]),
            coda: PositionRules::forbidden(),
            patterns: vec![pattern("CV")],
            forbidden_boundaries: vec![],
        },
        prosody: Default::default(),
        orthography: None,
    }
}
fn syllable(onset: &SoundDefinition, coda: Option<&SoundDefinition>) -> Syllable {
    Syllable::new(
        vec![id(onset)],
        vec![id(&A)],
        coda.into_iter().map(id).collect(),
    )
}
fn custom_consonant(
    key: &'static str,
    edit: impl FnOnce(&mut ConsonantFeatures),
) -> SoundDefinition {
    let SoundFeatures::Consonant(mut features) = P_BILABIAL.features().clone() else {
        unreachable!()
    };
    edit(&mut features);
    SoundDefinition::new(
        SoundId::from_static(key),
        key.into(),
        SoundFeatures::Consonant(features),
    )
    .unwrap()
}
fn assessment(anatomy: &SpeakerAnatomy, sound: &SoundDefinition) -> ProductionAssessment {
    anatomy.assess_sound(sound).unwrap()
}

#[test]
fn catalogue_is_structured_and_ids_are_explicit_and_unique() {
    let catalog = SoundCatalog::builtin();
    assert_eq!(P_BILABIAL.id().as_str(), "p_bilabial");
    for sound in catalog.iter() {
        assert_eq!(catalog.get(sound.id()), Some(sound));
    }
    assert!(catalog.get(&SoundId::from_static("missing")).is_none());
    assert!(SoundCatalog::new([P_BILABIAL.clone(), P_BILABIAL.clone()]).is_err());
}

#[test]
fn tooth_and_lip_restrictions_remove_only_dependent_sounds() {
    let human = SpeakerAnatomy::human();
    for sound in [&P_BILABIAL, &B_BILABIAL, &M_BILABIAL, &F, &A, &U] {
        assert_eq!(assessment(&human, sound), ProductionAssessment::Possible);
    }
    let toothless = human.clone().teeth(TeethConfiguration::None);
    assert!(!toothless
        .validate_anatomical_consistency()
        .unwrap()
        .warnings
        .is_empty());
    assert_eq!(
        assessment(&toothless, &P_BILABIAL),
        ProductionAssessment::Possible
    );
    assert_eq!(
        assessment(&toothless, &F),
        ProductionAssessment::Impossible(vec![Requirement::LabiodentalContact])
    );
    let lipless = human.lips(LipControl::None);
    assert_eq!(assessment(&lipless, &A), ProductionAssessment::Possible);
    assert_eq!(
        assessment(&lipless, &N_ALVEOLAR),
        ProductionAssessment::Possible
    );
    for sound in [&P_BILABIAL, &B_BILABIAL, &M_BILABIAL, &U] {
        assert!(matches!(
            assessment(&lipless, sound),
            ProductionAssessment::Impossible(_)
        ));
    }
}

#[test]
fn nasal_and_oral_airflow_are_independent_capabilities() {
    let mut anatomy = SpeakerAnatomy::human();
    anatomy.airflow.velic_port = VelicPortControl::OralOnly;
    assert_eq!(
        assessment(&anatomy, &P_BILABIAL),
        ProductionAssessment::Possible
    );
    assert_eq!(
        assessment(&anatomy, &M_BILABIAL),
        ProductionAssessment::Impossible(vec![Requirement::NasalAirflow])
    );
    anatomy.airflow.velic_port = VelicPortControl::NasalOnly;
    assert_eq!(
        assessment(&anatomy, &M_BILABIAL),
        ProductionAssessment::Possible
    );
    assert_eq!(
        assessment(&anatomy, &P_BILABIAL),
        ProductionAssessment::Impossible(vec![Requirement::OralAirflow])
    );
}

#[test]
fn fixed_voicing_does_not_mean_both_voice_types_are_impossible() {
    let mut anatomy = SpeakerAnatomy::human();
    anatomy.larynx.voicing.voiceless = false;
    assert_eq!(
        assessment(&anatomy, &B_BILABIAL),
        ProductionAssessment::Possible
    );
    assert_eq!(
        assessment(&anatomy, &P_BILABIAL),
        ProductionAssessment::Impossible(vec![Requirement::Voicing(Voicing::Voiceless)])
    );
    anatomy.larynx.voicing.voiceless = true;
    anatomy.larynx.voicing.voiced = false;
    assert_eq!(
        assessment(&anatomy, &P_BILABIAL),
        ProductionAssessment::Possible
    );
    assert_eq!(
        assessment(&anatomy, &B_BILABIAL),
        ProductionAssessment::Impossible(vec![Requirement::Voicing(Voicing::Voiced)])
    );
}

#[test]
fn aspiration_does_not_replace_voicing_checks() {
    let aspirated = custom_consonant("p_aspirated", |f| f.aspiration = Aspiration::Aspirated);
    let mut anatomy = SpeakerAnatomy::human();
    anatomy.larynx.can_aspirate = false;
    assert_eq!(
        assessment(&anatomy, &aspirated),
        ProductionAssessment::Impossible(vec![Requirement::Aspiration])
    );
    anatomy.larynx.can_aspirate = true;
    anatomy.larynx.voicing.voiceless = false;
    assert_eq!(
        assessment(&anatomy, &aspirated),
        ProductionAssessment::Impossible(vec![Requirement::Voicing(Voicing::Voiceless)])
    );
}

#[test]
fn all_modifier_requirements_are_checked_regardless_of_order() {
    let mut anatomy = SpeakerAnatomy::human().lips(LipControl::None);
    anatomy.oral_cavity.has_hard_palate = false;
    for modifiers in [
        vec![Modifier::Labialized, Modifier::Palatalized],
        vec![Modifier::Palatalized, Modifier::Labialized],
    ] {
        let sound = custom_consonant("modified_h", |f| {
            f.place = ArticulationPlace::Glottal;
            f.manner = Manner::NonSibilantFricative;
            f.modifiers = Cow::Owned(modifiers);
        });
        let ProductionAssessment::Impossible(missing) = assessment(&anatomy, &sound) else {
            panic!("must reject both missing capabilities")
        };
        assert_eq!(missing.len(), 2);
        assert!(missing.contains(&Requirement::LipRounding));
        assert!(missing.contains(&Requirement::HardPalate));
    }
}

#[test]
fn vowel_ranges_and_glottal_or_pharyngeal_requirements_are_used() {
    let mut anatomy = SpeakerAnatomy::human();
    anatomy.tongue.vowel_heights = vec![TongueHeight::Open];
    assert_eq!(assessment(&anatomy, &A), ProductionAssessment::Possible);
    assert_eq!(
        assessment(&anatomy, &I),
        ProductionAssessment::Impossible(vec![Requirement::VowelHeight(TongueHeight::Close)])
    );
    anatomy.tongue.root = TonguePartControl::None;
    let pharyngeal = custom_consonant("pharyngeal", |f| f.place = ArticulationPlace::Pharyngeal);
    assert_eq!(
        assessment(&anatomy, &pharyngeal),
        ProductionAssessment::Impossible(vec![Requirement::TongueRoot])
    );
    assert_eq!(assessment(&anatomy, &H), ProductionAssessment::Possible);
}

#[test]
fn unsupported_is_distinct_from_possible_and_from_missing_capabilities() {
    let unsupported = F.clone();
    let human = SpeakerAnatomy::human().teeth(TeethConfiguration::Fangs);
    assert_eq!(
        assessment(&human, &unsupported),
        ProductionAssessment::Unsupported(vec![SupportGap::UnspecifiedContact(
            Requirement::LabiodentalContact
        )])
    );
    assert!(ProductionRequirements::for_sound(&unsupported)
        .all_of()
        .contains(&Requirement::LabiodentalContact));
    let catalog = SoundCatalog::new([P_BILABIAL.clone(), A.clone(), unsupported.clone()]).unwrap();
    let possible = PossibleSoundSet::derive(&catalog, &human).unwrap();
    let ids: Vec<_> = possible.possible_ids().cloned().collect();
    assert_eq!(ids.len(), 2);
    assert!(ids.contains(P_BILABIAL.id()) && ids.contains(A.id()));
    assert!(!ids.contains(unsupported.id()));
    assert!(LanguageInventory::new([id(&unsupported)], &catalog, &possible).is_err());
    let trill = custom_consonant("trill", |f| f.manner = Manner::Trill);
    assert_eq!(assessment(&human, &trill), ProductionAssessment::Possible);
    let mut no_trills = human.clone();
    no_trills.articulation.trills.clear();
    assert_eq!(
        assessment(&no_trills, &trill),
        ProductionAssessment::Impossible(vec![Requirement::Trill(ArticulationPlace::Bilabial)])
    );
    let confirmed = human
        .clone()
        .labiodental_contact(name_generator::anatomy::speaker::ContactCapability::Available);
    assert_eq!(
        assessment(&confirmed, &unsupported),
        ProductionAssessment::Possible
    );
    let lipless = human.lips(LipControl::None);
    assert!(matches!(
        assessment(&lipless, &unsupported),
        ProductionAssessment::Impossible(_)
    ));
}

#[test]
fn inventory_rejects_duplicates_unknowns_and_stale_catalogue_assessments() {
    let catalog = SoundCatalog::new([P_BILABIAL.clone(), A.clone()]).unwrap();
    let possible = PossibleSoundSet::derive(&catalog, &SpeakerAnatomy::human()).unwrap();
    assert!(
        LanguageInventory::new([id(&P_BILABIAL), id(&P_BILABIAL)], &catalog, &possible).is_err()
    );
    assert!(
        LanguageInventory::new([SoundId::from_static("unknown")], &catalog, &possible).is_err()
    );
    assert!(LanguageInventory::new([], &catalog, &possible).is_err());
    let SoundFeatures::Consonant(mut changed) = P_BILABIAL.features().clone() else {
        unreachable!()
    };
    changed.voicing = Voicing::Voiced;
    let redefined = SoundDefinition::new(
        id(&P_BILABIAL),
        "p".into(),
        SoundFeatures::Consonant(changed),
    )
    .unwrap();
    let altered = SoundCatalog::new([redefined, A.clone()]).unwrap();
    assert!(LanguageInventory::new([id(&A)], &altered, &possible).is_err());
}

#[test]
fn the_same_anatomy_supports_distinct_inventories_and_one_vowel() {
    let catalog = SoundCatalog::builtin();
    let first = cv(&[&P_BILABIAL]).build(&catalog).unwrap();
    let second = cv(&[&N_ALVEOLAR]).build(&catalog).unwrap();
    assert!(first.inventory().contains(P_BILABIAL.id()));
    assert!(!second.inventory().contains(P_BILABIAL.id()));
    assert!(second.inventory().contains(N_ALVEOLAR.id()));
    assert_eq!(first.anatomy(), second.anatomy());
    assert_eq!(first.inventory().len(), 2);
}

#[test]
fn changing_anatomy_or_deserialized_input_requires_full_revalidation() {
    let catalog = SoundCatalog::builtin();
    let profile = cv(&[&P_BILABIAL]).build(&catalog).unwrap();
    let mut changed = profile.to_draft();
    changed.anatomy.lips = LipControl::None;
    assert!(changed.build(&catalog).is_err());
    let json = serde_json::to_string(&profile.to_draft()).unwrap();
    let mut loaded: LanguageDraft = serde_json::from_str(&json).unwrap();
    loaded.inventory.clear();
    assert!(loaded.build(&catalog).is_err());
    let mut inconsistent = SpeakerAnatomy::human();
    inconsistent.oral_cavity.has_soft_palate = false;
    assert!(inconsistent.validate_anatomical_consistency().is_err());
}

#[test]
fn forbidden_and_optional_positions_are_explicit_and_checked() {
    let catalog = SoundCatalog::builtin();
    let profile = cv(&[&P_BILABIAL]).build(&catalog).unwrap();
    assert!(profile
        .validate_word(vec![syllable(&P_BILABIAL, None)])
        .is_ok());
    assert!(profile
        .validate_word(vec![syllable(&P_BILABIAL, Some(&P_BILABIAL))])
        .is_err());
    let mut invalid = profile.to_draft();
    invalid.phonotactics.patterns.push(pattern("CVC"));
    assert!(invalid.build(&catalog).is_err());
    let mut invalid = profile.to_draft();
    invalid
        .phonotactics
        .onset
        .choices
        .push(sequence(&T_ALVEOLAR));
    assert!(invalid.build(&catalog).is_err());
    let mut impossible_context = profile.to_draft();
    impossible_context.phonotactics.nucleus.choices[0].position = WordPosition::InitialOnly;
    // A required onset means the nucleus can never be word-initial.
    assert!(impossible_context.build(&catalog).is_err());
}

fn boundary_language() -> LanguageDraft {
    let mut draft = cv(&[&P_BILABIAL, &T_ALVEOLAR]);
    draft.phonotactics.coda = PositionRules::optional(vec![sequence(&T_ALVEOLAR)]);
    draft.phonotactics.patterns.push(pattern("CVC"));
    draft
        .phonotactics
        .forbidden_boundaries
        .push(ForbiddenBoundaryPair {
            left: id(&T_ALVEOLAR),
            right: id(&T_ALVEOLAR),
        });
    draft
}
#[test]
fn legal_syllables_cannot_bypass_cross_syllable_constraints() {
    let profile = boundary_language().build(&SoundCatalog::builtin()).unwrap();
    assert!(profile
        .validate_word(vec![syllable(&P_BILABIAL, Some(&T_ALVEOLAR))])
        .is_ok());
    assert!(profile
        .validate_word(vec![syllable(&T_ALVEOLAR, None)])
        .is_ok());
    let error = profile
        .validate_word(vec![
            syllable(&P_BILABIAL, Some(&T_ALVEOLAR)),
            syllable(&T_ALVEOLAR, None),
        ])
        .unwrap_err();
    assert!(error.has_error("boundaries"));
    assert!(profile
        .validate_word(vec![
            syllable(&P_BILABIAL, None),
            syllable(&T_ALVEOLAR, None)
        ])
        .is_ok());
}

#[test]
fn word_initial_only_rules_do_not_leak_into_medial_positions() {
    let mut draft = cv(&[&P_BILABIAL, &T_ALVEOLAR]);
    draft.phonotactics.onset.choices[0].position = WordPosition::InitialOnly;
    let profile = draft.build(&SoundCatalog::builtin()).unwrap();
    assert!(profile
        .validate_word(vec![
            syllable(&P_BILABIAL, None),
            syllable(&T_ALVEOLAR, None)
        ])
        .is_ok());
    assert!(profile
        .validate_word(vec![
            syllable(&T_ALVEOLAR, None),
            syllable(&P_BILABIAL, None)
        ])
        .is_err());
}

#[test]
fn spelling_is_separate_from_ipa_and_never_silently_drops_sounds() {
    let catalog = SoundCatalog::builtin();
    let mut draft = cv(&[&S_POSTALVEOLAR]);
    draft.orthography = Some(Orthography {
        mappings: BTreeMap::from([(id(&S_POSTALVEOLAR), "sh".into()), (id(&A), "a".into())]),
        ..Default::default()
    });
    let profile = draft.build(&catalog).unwrap();
    let word = profile
        .validate_word(vec![syllable(&S_POSTALVEOLAR, None)])
        .unwrap();
    assert_eq!(profile.ipa(&word).unwrap(), "ʃa");
    assert_eq!(profile.spell(&word).unwrap(), "sha");
    let incomplete = Orthography {
        mappings: BTreeMap::from([(id(&A), "a".into())]),
        ..Default::default()
    };
    assert!(incomplete.render(&word).is_err());
    let mut draft = profile.to_draft();
    draft.orthography = Some(incomplete);
    assert!(draft.build(&catalog).is_err());
}

#[test]
fn ipa_codepoints_are_not_counted_as_sound_segments() {
    let profile = cv(&[&M_BILABIAL_VOICELESS])
        .build(&SoundCatalog::builtin())
        .unwrap();
    let word = profile
        .validate_word(vec![syllable(&M_BILABIAL_VOICELESS, None)])
        .unwrap();
    assert_eq!(word.sounds().count(), 2);
    assert_eq!(profile.ipa(&word).unwrap(), "m̥a");
}

#[test]
fn syllabic_consonants_are_nuclei_not_an_arbitrary_three_vowel_requirement() {
    let sound = custom_consonant("syllabic_m", |f| {
        f.manner = Manner::Nasal;
        f.voicing = Voicing::Voiced;
        f.modifiers = Cow::Borrowed(&[Modifier::Syllabic]);
    });
    let catalog = SoundCatalog::new([sound.clone()]).unwrap();
    let draft = LanguageDraft {
        name: "Syllabic".into(),
        anatomy: SpeakerAnatomy::human(),
        inventory: vec![id(&sound)],
        phonotactics: Phonotactics {
            onset: PositionRules::forbidden(),
            nucleus: PositionRules::required(vec![sequence(&sound)]),
            coda: PositionRules::forbidden(),
            patterns: vec![pattern("V")],
            forbidden_boundaries: vec![],
        },
        prosody: Default::default(),
        orthography: None,
    };
    let profile: LanguageProfile = draft.build(&catalog).unwrap();
    assert!(profile
        .validate_word(vec![Syllable::new(vec![], vec![id(&sound)], vec![])])
        .is_ok());
}

#[test]
fn patterns_have_one_consistent_parse_and_validate_on_deserialization() {
    assert_eq!(
        SyllablePattern::parse("(CC)(VV)(C)").unwrap(),
        SyllablePattern::parse("ccvvc").unwrap()
    );
    for invalid in [
        "", "CCC", "CVCV", "(C)(V", "(CV)", "C(V)", "(C)()(V)", "C💬V",
    ] {
        assert!(SyllablePattern::parse(invalid).is_err(), "{invalid}");
    }
    assert!(serde_json::from_str::<SyllablePattern>("\"CCC\"").is_err());
    assert!(serde_json::from_str::<SoundId>("\" \"").is_err());
    assert!(std::panic::catch_unwind(|| SoundId::from_static(" ")).is_err());
}

#[test]
fn airflow_capabilities_are_not_silently_accepted_when_missing() {
    for (airflow, expected) in [
        (AirflowMechanism::Pulmonic, Requirement::PulmonicAirflow),
        (AirflowMechanism::Click, Requirement::ClickAirflow),
        (AirflowMechanism::Ejective, Requirement::EjectiveAirflow),
        (AirflowMechanism::Implosive, Requirement::ImplosiveAirflow),
    ] {
        let sound = custom_consonant("airflow_case", |f| f.airflow = airflow);
        let mut anatomy = SpeakerAnatomy::human();
        assert_eq!(assessment(&anatomy, &sound), ProductionAssessment::Possible);
        match airflow {
            AirflowMechanism::Pulmonic => {
                anatomy.airflow.pulmonic = name_generator::anatomy::speaker::PulmonicControl::None
            }
            AirflowMechanism::Click => anatomy.airflow.can_produce_clicks = false,
            AirflowMechanism::Ejective => anatomy.larynx.can_produce_ejectives = false,
            AirflowMechanism::Implosive => anatomy.larynx.can_produce_implosives = false,
        }
        assert_eq!(
            assessment(&anatomy, &sound),
            ProductionAssessment::Impossible(vec![expected])
        );
    }
}

#[test]
fn phonation_and_glottal_closure_have_their_own_requirements() {
    for phonation in [Phonation::Breathy, Phonation::Creaky] {
        let sound = custom_consonant("phonation_case", |f| {
            f.voicing = Voicing::Voiced;
            f.phonation = phonation;
        });
        let mut anatomy = SpeakerAnatomy::human();
        assert_eq!(assessment(&anatomy, &sound), ProductionAssessment::Possible);
        anatomy.larynx.can_produce_breathy_voice = false;
        anatomy.larynx.can_produce_creaky_voice = false;
        assert_eq!(
            assessment(&anatomy, &sound),
            ProductionAssessment::Impossible(vec![Requirement::Phonation(phonation)])
        );
    }
    let stop = custom_consonant("glottal_stop", |f| f.place = ArticulationPlace::Glottal);
    let mut anatomy = SpeakerAnatomy::human();
    anatomy.larynx.can_produce_ejectives = false;
    anatomy.larynx.can_produce_implosives = false;
    anatomy.larynx.can_close_glottis = false;
    assert_eq!(
        assessment(&anatomy, &stop),
        ProductionAssessment::Impossible(vec![Requirement::GlottalClosure])
    );
}

#[test]
fn final_only_and_disabled_choices_cannot_be_used_in_other_positions() {
    let mut draft = boundary_language();
    draft.phonotactics.coda.choices[0].position = WordPosition::FinalOnly;
    let profile = draft.build(&SoundCatalog::builtin()).unwrap();
    assert!(profile
        .validate_word(vec![
            syllable(&P_BILABIAL, None),
            syllable(&P_BILABIAL, Some(&T_ALVEOLAR))
        ])
        .is_ok());
    assert!(profile
        .validate_word(vec![
            syllable(&P_BILABIAL, Some(&T_ALVEOLAR)),
            syllable(&P_BILABIAL, None)
        ])
        .is_err());
    assert!(profile.validate_word(vec![]).is_err());

    let mut draft = cv(&[&P_BILABIAL, &T_ALVEOLAR]);
    draft.phonotactics.onset.choices[0].weight = Weight::try_from(0.0).unwrap();
    let profile = draft.build(&SoundCatalog::builtin()).unwrap();
    assert!(profile
        .validate_word(vec![syllable(&T_ALVEOLAR, None)])
        .is_ok());
    assert!(profile
        .validate_word(vec![syllable(&P_BILABIAL, None)])
        .is_err());

    let mut json = serde_json::to_value(profile.to_draft()).unwrap();
    json["phonotactics"]["onset"]["choices"][0]["weight"] = serde_json::json!(-1);
    assert!(serde_json::from_value::<LanguageDraft>(json).is_err());
}

#[test]
fn invalid_and_all_zero_weights_are_rejected_without_clamping() {
    for invalid in [-1.0, f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        assert!(Weight::try_from(invalid).is_err());
    }
    assert_eq!(Weight::try_from(2.5).unwrap().value(), 2.5);
    assert!(serde_json::from_str::<Weight>("-1").is_err());
    let mut draft = cv(&[&P_BILABIAL]);
    draft.phonotactics.onset.choices[0].weight = Weight::try_from(0.0).unwrap();
    assert!(draft.build(&SoundCatalog::builtin()).is_err());
}
