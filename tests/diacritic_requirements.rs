//! These are contracts for the project's bounded capability model, not a claim
//! that the IPA chart specifies universal biological prerequisites.
use name_generator::anatomy::{
    production::{ProductionAssessment, Requirement},
    speaker::{ContactCapability, LipControl, SpeakerAnatomy, TonguePartControl, VelicPortControl},
};
use name_generator::phonology::{diacritics::IpaDiacritic, *};
use std::borrow::Cow;

fn modified(base: &str, modifier: Modifier) -> SoundDefinition {
    let catalog = SoundCatalog::builtin();
    let mut features = catalog.find_ipa(base).unwrap().features().clone();
    match &mut features {
        SoundFeatures::Consonant(f) => {
            f.modifiers = Cow::Owned(vec![modifier]);
            if modifier == Modifier::Dental {
                f.place = ArticulationPlace::Dental;
            }
            if modifier == Modifier::Linguolabial {
                f.place = ArticulationPlace::Linguolabial;
            }
        }
        SoundFeatures::Vowel(f) => {
            f.modifiers = Cow::Owned(vec![modifier]);
        }
    }
    let ipa = format!("{base}{}", IpaDiacritic::Modifier(modifier).ipa());
    SoundDefinition::new(SoundId::from_static("modified_fixture"), ipa, features).unwrap()
}
fn impossible(anatomy: &SpeakerAnatomy, sound: &SoundDefinition, expected: Requirement) {
    let ProductionAssessment::Impossible(missing) = anatomy.assess_sound(sound).unwrap() else {
        panic!("must reject {}", sound.description())
    };
    assert!(
        missing.contains(&expected),
        "{missing:?}: {}",
        sound.description()
    );
}

#[test]
fn relative_diacritics_require_fine_control_not_a_silent_fallback() {
    for modifier in [
        Modifier::Raised,
        Modifier::Lowered,
        Modifier::Advanced,
        Modifier::Retracted,
        Modifier::Centralized,
        Modifier::MidCentralized,
    ] {
        let sound = modified("e", modifier);
        let mut anatomy = SpeakerAnatomy::human();
        assert_eq!(
            anatomy.assess_sound(&sound).unwrap(),
            ProductionAssessment::Possible
        );
        anatomy.articulation.can_fine_adjust = false;
        impossible(&anatomy, &sound, Requirement::VowelPrecision);
    }
    let sound = modified("ɹ", Modifier::Raised);
    let mut anatomy = SpeakerAnatomy::human();
    anatomy.articulation.can_fine_adjust = false;
    impossible(
        &anatomy,
        &sound,
        Requirement::Precision(ArticulationPlace::Alveolar),
    );
}

#[test]
fn rounding_secondary_articulation_and_root_modifications_are_checked() {
    for modifier in [
        Modifier::MoreRounded,
        Modifier::LessRounded,
        Modifier::Labialized,
    ] {
        let sound = modified("t", modifier);
        let anatomy = SpeakerAnatomy::human().lips(LipControl::Rigid);
        impossible(&anatomy, &sound, Requirement::LipRounding);
    }
    let mut anatomy = SpeakerAnatomy::human();
    anatomy.oral_cavity.has_hard_palate = false;
    impossible(
        &anatomy,
        &modified("t", Modifier::Palatalized),
        Requirement::HardPalate,
    );
    anatomy = SpeakerAnatomy::human();
    anatomy.oral_cavity.has_soft_palate = false;
    anatomy.airflow.velic_port = VelicPortControl::OralOnly;
    impossible(
        &anatomy,
        &modified("t", Modifier::Velarized),
        Requirement::SoftPalate,
    );
    anatomy = SpeakerAnatomy::human();
    anatomy.tongue.root = TonguePartControl::None;
    impossible(
        &anatomy,
        &modified("t", Modifier::Pharyngealized),
        Requirement::TongueRoot,
    );
    anatomy.tongue.root = TonguePartControl::Limited;
    for modifier in [Modifier::AdvancedTongueRoot, Modifier::RetractedTongueRoot] {
        impossible(
            &anatomy,
            &modified("e", modifier),
            Requirement::AgileTongueRoot,
        );
    }
    anatomy = SpeakerAnatomy::human();
    anatomy.articulation.can_rhoticize = false;
    impossible(
        &anatomy,
        &modified("ə", Modifier::Rhoticity),
        Requirement::RhoticControl,
    );
}

#[test]
fn velarized_or_pharyngealized_is_an_alternative_not_two_mandatory_requirements() {
    let sound = modified("l", Modifier::VelarizedOrPharyngealized);
    let mut anatomy = SpeakerAnatomy::human();
    anatomy.tongue.root = TonguePartControl::None;
    assert_eq!(
        anatomy.assess_sound(&sound).unwrap(),
        ProductionAssessment::Possible
    );
    anatomy.tongue.body = TonguePartControl::Limited;
    impossible(&anatomy, &sound, Requirement::VelarOrPharyngealization);
    anatomy.tongue.root = TonguePartControl::Limited;
    assert_eq!(
        anatomy.assess_sound(&sound).unwrap(),
        ProductionAssessment::Possible
    );
}

#[test]
fn apical_laminal_dental_and_linguolabial_require_the_right_articulators() {
    let mut anatomy = SpeakerAnatomy::human();
    anatomy.tongue.tip = TonguePartControl::None;
    anatomy.tongue.can_curl_for_retroflex = false;
    impossible(
        &anatomy,
        &modified("t", Modifier::Apical),
        Requirement::TongueTip,
    );
    anatomy = SpeakerAnatomy::human();
    anatomy.tongue.blade = TonguePartControl::None;
    impossible(
        &anatomy,
        &modified("t", Modifier::Laminal),
        Requirement::TongueBlade,
    );
    anatomy = SpeakerAnatomy::human().dental_contact(ContactCapability::Unavailable);
    impossible(
        &anatomy,
        &modified("t", Modifier::Dental),
        Requirement::DentalContact,
    );
    anatomy = SpeakerAnatomy::human().lips(LipControl::None);
    impossible(
        &anatomy,
        &modified("t", Modifier::Linguolabial),
        Requirement::Lips,
    );
}

#[test]
fn nasality_and_release_modifications_do_not_bypass_airflow_or_release_control() {
    let mut anatomy = SpeakerAnatomy::human();
    anatomy.airflow.velic_port = VelicPortControl::OralOnly;
    impossible(
        &anatomy,
        &modified("e", Modifier::Nasalized),
        Requirement::MixedAirflow,
    );
    impossible(
        &anatomy,
        &modified("t", Modifier::NasalRelease),
        Requirement::MixedAirflow,
    );
    anatomy = SpeakerAnatomy::human();
    anatomy.tongue.can_perform_lateral_release = false;
    impossible(
        &anatomy,
        &modified("t", Modifier::LateralRelease),
        Requirement::LateralControl,
    );
    for modifier in [
        Modifier::NasalRelease,
        Modifier::LateralRelease,
        Modifier::NoAudibleRelease,
    ] {
        let mut anatomy = SpeakerAnatomy::human();
        anatomy.articulation.can_control_release = false;
        impossible(
            &anatomy,
            &modified("t", modifier),
            Requirement::ReleaseControl,
        );
    }
    let mut anatomy = SpeakerAnatomy::human();
    anatomy.articulation.syllabic_manners.clear();
    impossible(
        &anatomy,
        &modified("n", Modifier::Syllabic),
        Requirement::SyllabicManner(Manner::Nasal),
    );
}
