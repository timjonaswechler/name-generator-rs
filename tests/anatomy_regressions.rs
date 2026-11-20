use name_generator::anatomy::speaker::{LipControl, SpeakerAnatomy, TeethConfiguration};

// Reduced sound capabilities must not make an otherwise coherent anatomy invalid.
#[test]
fn toothless_anatomy_is_valid() {
    let anatomy = SpeakerAnatomy::human().teeth(TeethConfiguration::None);
    assert!(anatomy.validate_anatomical_consistency().is_ok());
}

#[test]
fn missing_lips_do_not_invalidate_remaining_anatomy() {
    let anatomy = SpeakerAnatomy::human().lips(LipControl::None);
    assert!(anatomy.validate_anatomical_consistency().is_ok());
}
