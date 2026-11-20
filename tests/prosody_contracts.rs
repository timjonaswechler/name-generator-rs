use name_generator::anatomy::speaker::SpeakerAnatomy;
use name_generator::language::LanguageDraft;
use name_generator::orthography::Orthography;
use name_generator::phonology::{consonants::pulmonics::M_BILABIAL, vowels::A, SoundCatalog};
use name_generator::prosody::{
    spelling::{DurationSpelling, ProsodySpelling, ToneSpelling, WrittenMark},
    *,
};
use name_generator::syllables::{patterns::SyllablePattern, *};
use name_generator::weights::Weight;
use std::collections::BTreeMap;

fn draft() -> LanguageDraft {
    LanguageDraft {
        name: "Prosody example".into(),
        anatomy: SpeakerAnatomy::human(),
        inventory: vec![M_BILABIAL.id().clone(), A.id().clone()],
        phonotactics: Phonotactics {
            onset: PositionRules::required(vec![SequenceRule::new(vec![M_BILABIAL.id().clone()])]),
            nucleus: PositionRules::required(vec![SequenceRule::new(vec![A.id().clone()])]),
            coda: PositionRules::forbidden(),
            patterns: vec![WeightedPattern {
                pattern: SyllablePattern::parse("CV").unwrap(),
                weight: Weight::ONE,
            }],
            forbidden_boundaries: vec![],
        },
        prosody: ProsodyRules::default(),
        orthography: None,
    }
}
fn ma() -> Syllable {
    Syllable::new(vec![M_BILABIAL.id().clone()], vec![A.id().clone()], vec![])
}
fn spelling() -> Orthography {
    Orthography {
        mappings: BTreeMap::from([
            (M_BILABIAL.id().clone(), "m".into()),
            (A.id().clone(), "a".into()),
        ]),
        ..Default::default()
    }
}
fn tonal(tone: Tone) -> LanguageDraft {
    let mut d = draft();
    d.prosody.tone = ToneRules {
        presence: Presence::Required,
        inventory: vec![tone],
    };
    d
}

#[test]
fn all_chart_durations_preserve_one_sound_slot_and_have_distinct_ipa() {
    let mut d = draft();
    d.prosody.duration.vowels = vec![
        Duration::Normal,
        Duration::Long,
        Duration::HalfLong,
        Duration::ExtraShort,
    ];
    d.prosody.duration.consonants = d.prosody.duration.vowels.clone();
    let profile = d.build(&SoundCatalog::builtin()).unwrap();
    for (duration, expected) in [
        (Duration::Normal, "ma"),
        (Duration::Long, "maː"),
        (Duration::HalfLong, "maˑ"),
        (Duration::ExtraShort, "mă"),
    ] {
        let mut syllable = ma();
        syllable.nucleus[0].duration = duration;
        let word = profile.validate_word(vec![syllable]).unwrap();
        assert_eq!(word.sounds().count(), 2);
        assert_eq!(profile.ipa(&word).unwrap(), expected);
    }
    let mut geminate = ma();
    geminate.onset[0].duration = Duration::Long;
    let word = profile.validate_word(vec![geminate]).unwrap();
    assert_eq!(profile.ipa(&word).unwrap(), "mːa");
}

#[test]
fn language_duration_choices_do_not_change_the_inventory_or_grant_anatomical_ability() {
    let catalog = SoundCatalog::builtin();
    let plain = draft().build(&catalog).unwrap();
    let mut syllable = ma();
    syllable.nucleus[0].duration = Duration::Long;
    assert!(plain.validate_word(vec![syllable.clone()]).is_err());
    let mut d = draft();
    d.prosody.duration.vowels.push(Duration::Long);
    let long = d.clone().build(&catalog).unwrap();
    assert_eq!(
        plain.inventory().iter().collect::<Vec<_>>(),
        long.inventory().iter().collect::<Vec<_>>()
    );
    assert!(long.validate_word(vec![syllable]).is_ok());
    d.anatomy.prosody.durations = vec![Duration::Normal];
    assert!(d.build(&catalog).unwrap_err().has_error("duration"));
}

#[test]
fn fixed_stress_is_checked_from_either_word_edge_without_silent_fallback() {
    for (edge, offset, stressed, expected) in [
        (WordEdge::Start, 0, 0, "ˈmama"),
        (WordEdge::End, 0, 1, "maˈma"),
        (WordEdge::End, 1, 0, "ˈmama"),
    ] {
        let mut d = draft();
        d.prosody.stress = StressRule::Fixed {
            edge,
            offset,
            allow_secondary: false,
        };
        let profile = d.build(&SoundCatalog::builtin()).unwrap();
        let mut syllables = vec![ma(), ma()];
        assert!(profile.validate_word(syllables.clone()).is_err());
        syllables[stressed].prosody.stress = Stress::Primary;
        let word = profile.validate_word(syllables.clone()).unwrap();
        assert_eq!(profile.ipa(&word).unwrap(), expected);
        syllables[1 - stressed].prosody.stress = Stress::Primary;
        assert!(profile.validate_word(syllables).is_err());
        if offset == 1 {
            assert!(profile.validate_word(vec![ma()]).is_err());
        }
    }
}

#[test]
fn lexical_primary_and_secondary_stress_are_not_interchangeable() {
    let mut d = draft();
    d.prosody.stress = StressRule::Lexical {
        require_primary: true,
        allow_secondary: true,
    };
    let profile = d.build(&SoundCatalog::builtin()).unwrap();
    let mut syllables = vec![ma(), ma(), ma()];
    syllables[2].prosody.stress = Stress::Secondary;
    assert!(profile.validate_word(syllables.clone()).is_err());
    syllables[0].prosody.stress = Stress::Primary;
    let word = profile.validate_word(syllables.clone()).unwrap();
    assert_eq!(profile.ipa(&word).unwrap(), "ˈmamaˌma");
    assert!(draft()
        .build(&SoundCatalog::builtin())
        .unwrap()
        .validate_word(syllables)
        .is_err());
}

#[test]
fn the_five_pitch_levels_are_rendered_independently() {
    for (level, expected) in [
        (PitchLevel::ExtraLow, "ma˩"),
        (PitchLevel::Low, "ma˨"),
        (PitchLevel::Mid, "ma˧"),
        (PitchLevel::High, "ma˦"),
        (PitchLevel::ExtraHigh, "ma˥"),
    ] {
        let tone = Tone::level(level);
        let profile = tonal(tone.clone()).build(&SoundCatalog::builtin()).unwrap();
        assert!(profile.validate_word(vec![ma()]).is_err());
        let mut syllable = ma();
        syllable.prosody.tone = Some(tone);
        let word = profile.validate_word(vec![syllable]).unwrap();
        assert_eq!(profile.ipa(&word).unwrap(), expected);
    }
}

#[test]
fn contours_and_register_steps_survive_word_validation_and_ipa_output() {
    let contour = ToneContour::try_from(vec![PitchLevel::Low, PitchLevel::ExtraHigh]).unwrap();
    for (register, expected) in [
        (ToneRegister::Normal, "ma˨˥"),
        (ToneRegister::Downstep, "maꜜ˨˥"),
        (ToneRegister::Upstep, "maꜛ˨˥"),
    ] {
        let tone = Tone {
            contour: contour.clone(),
            register,
        };
        let profile = tonal(tone.clone()).build(&SoundCatalog::builtin()).unwrap();
        let mut syllable = ma();
        syllable.prosody.tone = Some(tone);
        assert_eq!(
            profile
                .ipa(&profile.validate_word(vec![syllable]).unwrap())
                .unwrap(),
            expected
        );
    }
    assert!(ToneContour::try_from(vec![]).is_err());
    assert!(serde_json::from_str::<ToneContour>("[]").is_err());
    assert!(ToneContour::try_from(vec![PitchLevel::Low, PitchLevel::Low]).is_err());
}

#[test]
fn optional_tones_do_not_turn_into_required_or_unrestricted_tones() {
    let high = Tone::level(PitchLevel::ExtraHigh);
    let mut d = tonal(high.clone());
    d.prosody.tone.presence = Presence::Optional;
    let profile = d.build(&SoundCatalog::builtin()).unwrap();
    assert!(profile.validate_word(vec![ma()]).is_ok());
    let mut syllable = ma();
    syllable.prosody.tone = Some(Tone::level(PitchLevel::Low));
    assert!(profile.validate_word(vec![syllable.clone()]).is_err());
    syllable.prosody.tone = Some(high);
    assert!(profile.validate_word(vec![syllable.clone()]).is_ok());
    assert!(draft()
        .build(&SoundCatalog::builtin())
        .unwrap()
        .validate_word(vec![syllable])
        .is_err());
}

#[test]
fn pitch_and_stress_capabilities_cannot_be_enabled_by_language_rules() {
    let catalog = SoundCatalog::builtin();
    let mut d = draft();
    d.prosody.stress = StressRule::Fixed {
        edge: WordEdge::Start,
        offset: 0,
        allow_secondary: false,
    };
    d.anatomy.prosody.can_stress = false;
    assert!(d.build(&catalog).unwrap_err().has_error("stress"));
    let tone = Tone {
        contour: ToneContour::try_from(vec![PitchLevel::Low, PitchLevel::ExtraHigh]).unwrap(),
        register: ToneRegister::Downstep,
    };
    let base = tonal(tone);
    let mut no_contour = base.clone();
    no_contour.anatomy.prosody.can_contour_tones = false;
    assert!(no_contour.build(&catalog).unwrap_err().has_error("tone"));
    let mut no_register = base.clone();
    no_register.anatomy.prosody.can_shift_register = false;
    assert!(no_register.build(&catalog).unwrap_err().has_error("tone"));
    let mut no_high = base;
    no_high.anatomy.prosody.pitch_levels = vec![PitchLevel::Low];
    assert!(no_high.build(&catalog).unwrap_err().has_error("tone"));
}

#[test]
fn spelling_requires_explicit_prosody_mappings_or_explicit_omission() {
    let catalog = SoundCatalog::builtin();
    let high = Tone::level(PitchLevel::ExtraHigh);
    let mut d = tonal(high.clone());
    d.prosody.duration.vowels.push(Duration::Long);
    d.prosody.stress = StressRule::Fixed {
        edge: WordEdge::Start,
        offset: 0,
        allow_secondary: false,
    };
    d.orthography = Some(spelling());
    assert!(d.clone().build(&catalog).is_err());
    let marks = ProsodySpelling {
        duration: BTreeMap::from([(Duration::Long, DurationSpelling::Repeat)]),
        primary_stress: Some(WrittenMark::Text("'".into())),
        secondary_stress: None,
        tones: vec![ToneSpelling {
            tone: high.clone(),
            mark: WrittenMark::Text("h".into()),
        }],
    };
    d.orthography.as_mut().unwrap().prosody = marks;
    let profile = d.build(&catalog).unwrap();
    let mut syllable = ma();
    syllable.nucleus[0].duration = Duration::Long;
    syllable.prosody.stress = Stress::Primary;
    syllable.prosody.tone = Some(high.clone());
    let word = profile.validate_word(vec![syllable]).unwrap();
    assert_eq!(profile.ipa(&word).unwrap(), "ˈmaː˥");
    assert_eq!(profile.spell(&word).unwrap(), "'maah");
    assert!(spelling().render(&word).is_err());
    let mut omitted = profile.to_draft();
    omitted.orthography.as_mut().unwrap().prosody = ProsodySpelling {
        duration: BTreeMap::from([(Duration::Long, DurationSpelling::Omit)]),
        primary_stress: Some(WrittenMark::Omit),
        secondary_stress: None,
        tones: vec![ToneSpelling {
            tone: high,
            mark: WrittenMark::Omit,
        }],
    };
    let omitted = omitted.build(&catalog).unwrap();
    assert_eq!(omitted.spell(&word).unwrap(), "ma");
    assert_eq!(omitted.ipa(&word).unwrap(), "ˈmaː˥");
    let plain = draft().build(&catalog).unwrap();
    assert!(plain.ipa(&word).is_err());
    assert!(plain.spell(&word).is_err());
}

#[test]
fn required_tones_need_a_voiced_nucleus_not_merely_a_voiced_onset() {
    use name_generator::phonology::{SoundDefinition, SoundFeatures, SoundId, Voicing};
    let SoundFeatures::Vowel(mut features) = A.features().clone() else {
        unreachable!()
    };
    features.voicing = Voicing::Voiceless;
    let vowel = SoundDefinition::new(
        SoundId::from_static("a_voiceless"),
        "ḁ".into(),
        SoundFeatures::Vowel(features),
    )
    .unwrap();
    let catalog = SoundCatalog::new([M_BILABIAL.clone(), vowel.clone()]).unwrap();
    let high = Tone::level(PitchLevel::ExtraHigh);
    let mut d = tonal(high.clone());
    d.inventory = vec![M_BILABIAL.id().clone(), vowel.id().clone()];
    d.phonotactics.nucleus =
        PositionRules::required(vec![SequenceRule::new(vec![vowel.id().clone()])]);
    assert!(d.clone().build(&catalog).unwrap_err().has_error("tone"));
    d.prosody.tone.presence = Presence::Optional;
    let profile = d.build(&catalog).unwrap();
    let mut syllable = Syllable::new(
        vec![M_BILABIAL.id().clone()],
        vec![vowel.id().clone()],
        vec![],
    );
    assert!(profile.validate_word(vec![syllable.clone()]).is_ok());
    syllable.prosody.tone = Some(high);
    assert!(profile
        .validate_word(vec![syllable])
        .unwrap_err()
        .has_error("tone"));
}

#[test]
fn empty_spelling_text_is_not_a_substitute_for_explicit_omission() {
    let catalog = SoundCatalog::builtin();
    let mut d = draft();
    d.prosody.stress = StressRule::Fixed {
        edge: WordEdge::Start,
        offset: 0,
        allow_secondary: false,
    };
    let profile = d.clone().build(&catalog).unwrap();
    let mut syllable = ma();
    syllable.prosody.stress = Stress::Primary;
    let word = profile.validate_word(vec![syllable]).unwrap();
    let mut empty = spelling();
    empty.prosody.primary_stress = Some(WrittenMark::Text(" ".into()));
    assert!(empty.render(&word).is_err());
    d.orthography = Some(empty);
    assert!(d.build(&catalog).is_err());

    let mut d = draft();
    d.prosody.duration.vowels.push(Duration::Long);
    let profile = d.clone().build(&catalog).unwrap();
    let mut syllable = ma();
    syllable.nucleus[0].duration = Duration::Long;
    let word = profile.validate_word(vec![syllable]).unwrap();
    let mut empty = spelling();
    empty.prosody.duration.insert(
        Duration::Long,
        DurationSpelling::Affix {
            prefix: "".into(),
            suffix: "".into(),
        },
    );
    assert!(empty.render(&word).is_err());
    d.orthography = Some(empty);
    assert!(d.build(&catalog).is_err());
}

#[test]
fn nonsyllabic_vowels_can_be_glides_but_cannot_be_the_only_nucleus_core() {
    use name_generator::phonology::{vowels::I, Modifier, SoundDefinition, SoundFeatures, SoundId};
    use std::borrow::Cow;
    let SoundFeatures::Vowel(mut features) = I.features().clone() else {
        unreachable!()
    };
    features.modifiers = Cow::Borrowed(&[Modifier::NonSyllabic]);
    let glide = SoundDefinition::new(
        SoundId::from_static("i_glide"),
        "i̯".into(),
        SoundFeatures::Vowel(features),
    )
    .unwrap();
    let catalog = SoundCatalog::new([A.clone(), glide.clone()]).unwrap();
    let mut d = draft();
    d.inventory = vec![A.id().clone(), glide.id().clone()];
    d.phonotactics.onset = PositionRules::forbidden();
    d.phonotactics.nucleus = PositionRules::required(vec![SequenceRule::new(vec![
        A.id().clone(),
        glide.id().clone(),
    ])]);
    d.phonotactics.patterns[0].pattern = SyllablePattern::parse("VV").unwrap();
    let profile = d.clone().build(&catalog).unwrap();
    let word = profile
        .validate_word(vec![Syllable::new(
            vec![],
            vec![A.id().clone(), glide.id().clone()],
            vec![],
        )])
        .unwrap();
    assert_eq!(profile.ipa(&word).unwrap(), "ai̯");

    let mut invalid = d.clone();
    invalid.phonotactics.nucleus.choices[0].sounds = vec![glide.id().clone()];
    invalid.phonotactics.patterns[0].pattern = SyllablePattern::parse("V").unwrap();
    assert!(invalid.build(&catalog).is_err());

    d.phonotactics.onset =
        PositionRules::required(vec![SequenceRule::new(vec![glide.id().clone()])]);
    d.phonotactics.nucleus = PositionRules::required(vec![SequenceRule::new(vec![A.id().clone()])]);
    d.phonotactics.patterns[0].pattern = SyllablePattern::parse("CV").unwrap();
    let profile = d.build(&catalog).unwrap();
    let word = profile
        .validate_word(vec![Syllable::new(
            vec![glide.id().clone()],
            vec![A.id().clone()],
            vec![],
        )])
        .unwrap();
    assert_eq!(profile.ipa(&word).unwrap(), "i̯a");
}

#[test]
fn catalogue_sounds_cannot_hide_canonical_duration_stress_or_tone_marks() {
    use name_generator::phonology::{SoundDefinition, SoundId};
    for ipa in ["aː", "aˑ", "ă", "ˈa", "ˌa", "a˥", "á", "a.a"] {
        assert!(
            SoundDefinition::new(
                SoundId::from_static("bad_prosody"),
                ipa.into(),
                A.features().clone()
            )
            .is_err(),
            "{ipa}"
        );
    }
}

#[test]
fn serialized_drafts_and_syllables_preserve_annotations_and_revalidate() {
    let mut d = tonal(Tone::level(PitchLevel::ExtraHigh));
    d.prosody.duration.vowels.push(Duration::Long);
    let json = serde_json::to_string(&d).unwrap();
    let loaded: LanguageDraft = serde_json::from_str(&json).unwrap();
    let profile = loaded.build(&SoundCatalog::builtin()).unwrap();
    let mut syllable = ma();
    syllable.nucleus[0].duration = Duration::Long;
    syllable.prosody.tone = Some(Tone::level(PitchLevel::ExtraHigh));
    let loaded: Syllable =
        serde_json::from_str(&serde_json::to_string(&syllable).unwrap()).unwrap();
    assert_eq!(
        profile
            .ipa(&profile.validate_word(vec![loaded]).unwrap())
            .unwrap(),
        "maː˥"
    );
}
