use name_generator::anatomy::speaker::SpeakerAnatomy;
use name_generator::language::LanguageDraft;
use name_generator::phonology::consonants::pulmonics::{
    B_BILABIAL, M_BILABIAL_VOICELESS, P_BILABIAL,
};
use name_generator::phonology::vowels::{A, O, U};
use name_generator::phonology::SoundCatalog;
use name_generator::prosody::{
    Duration, DurationRules, PitchLevel, ProsodyRules, Stress, StressRule, Tone, ToneRules,
    WordEdge,
};
use name_generator::syllables::{
    patterns::SyllablePattern, Phonotactics, PositionRules, Presence, SequenceRule, Syllable,
    WeightedPattern,
};
use name_generator::validation::FormattedValidationErrors;
use name_generator::weights::Weight;

fn main() -> Result<(), FormattedValidationErrors> {
    let catalog = SoundCatalog::builtin();
    let draft = LanguageDraft {
        name: "Test Language".to_owned(),
        anatomy: SpeakerAnatomy::human(),
        inventory: [&P_BILABIAL, &B_BILABIAL, &M_BILABIAL_VOICELESS, &A, &O, &U]
            .iter()
            .map(|sound| sound.id().clone())
            .collect(),
        phonotactics: Phonotactics {
            onset: PositionRules::required(
                [&P_BILABIAL, &B_BILABIAL, &M_BILABIAL_VOICELESS]
                    .iter()
                    .map(|sound| SequenceRule::new(vec![sound.id().clone()]))
                    .collect(),
            ),
            nucleus: PositionRules::required(
                [&A, &O]
                    .iter()
                    .map(|sound| SequenceRule::new(vec![sound.id().clone()]))
                    .collect(),
            ),
            coda: PositionRules::forbidden(),
            patterns: vec![WeightedPattern {
                pattern: SyllablePattern::parse("CV").expect("literal CV pattern"),
                weight: Weight::ONE,
            }],
            forbidden_boundaries: vec![],
        },
        prosody: ProsodyRules {
            duration: DurationRules {
                vowels: vec![Duration::Normal, Duration::Long],
                consonants: vec![Duration::Normal],
            },
            stress: StressRule::Fixed {
                edge: WordEdge::Start,
                offset: 0,
                allow_secondary: false,
            },
            tone: ToneRules {
                presence: Presence::Optional,
                inventory: vec![Tone::level(PitchLevel::ExtraHigh)],
            },
        },
        orthography: None,
    };
    let language = draft.build(&catalog)?;
    println!("Validated language: {}", language.name());
    for id in language.inventory().iter() {
        println!(
            "  {}: {}",
            id,
            catalog.get(id).expect("validated sound").description()
        );
    }
    // A supplied word form demonstrates rendering; no random name is generated.
    let mut supplied = Syllable::new(vec![P_BILABIAL.id().clone()], vec![A.id().clone()], vec![]);
    supplied.nucleus[0].duration = Duration::Long;
    supplied.prosody.stress = Stress::Primary;
    supplied.prosody.tone = Some(Tone::level(PitchLevel::ExtraHigh));
    let word = language.validate_word(vec![supplied])?;
    println!("Supplied example IPA: {}", language.ipa(&word)?);
    Ok(())
}
