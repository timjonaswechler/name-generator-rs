//! The 31 properties on the official IPA diacritics sub-chart.
//! Primary properties have typed fields; remaining modifications use Modifier.
//! This is not an arbitrary-IPA-string parser.
use super::{Modifier, Voicing};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IpaDiacritic {
    Voicing(Voicing),
    Aspirated,
    BreathyVoiced,
    CreakyVoiced,
    Modifier(Modifier),
}
impl IpaDiacritic {
    pub fn ipa(self) -> &'static str {
        match self {
            Self::Voicing(Voicing::Voiceless) => "̥",
            Self::Voicing(Voicing::Voiced) => "̬",
            Self::Aspirated => "ʰ",
            Self::BreathyVoiced => "̤",
            Self::CreakyVoiced => "̰",
            Self::Modifier(m) => match m {
                Modifier::Raised => "̝",
                Modifier::Lowered => "̞",
                Modifier::Centralized => "̈",
                Modifier::Advanced => "̟",
                Modifier::Retracted => "̠",
                Modifier::Nasalized => "̃",
                Modifier::Labialized => "ʷ",
                Modifier::Palatalized => "ʲ",
                Modifier::Velarized => "ˠ",
                Modifier::Pharyngealized => "ˤ",
                Modifier::Syllabic => "̩",
                Modifier::NonSyllabic => "̯",
                Modifier::MoreRounded => "̹",
                Modifier::LessRounded => "̜",
                Modifier::MidCentralized => "̽",
                Modifier::Rhoticity => "˞",
                Modifier::Linguolabial => "̼",
                Modifier::VelarizedOrPharyngealized => "̴",
                Modifier::AdvancedTongueRoot => "̘",
                Modifier::RetractedTongueRoot => "̙",
                Modifier::Dental => "̪",
                Modifier::Apical => "̺",
                Modifier::Laminal => "̻",
                Modifier::NasalRelease => "ⁿ",
                Modifier::LateralRelease => "ˡ",
                Modifier::NoAudibleRelease => "̚",
            },
        }
    }
    pub fn name(self) -> String {
        match self {
            Self::Voicing(v) => v.to_string(),
            Self::Aspirated => "aspirated".to_owned(),
            Self::BreathyVoiced => "breathy voiced".to_owned(),
            Self::CreakyVoiced => "creaky voiced".to_owned(),
            Self::Modifier(m) => m.to_string(),
        }
    }
}
pub const ALL_DIACRITICS: &[IpaDiacritic] = &[
    IpaDiacritic::Voicing(Voicing::Voiceless),
    IpaDiacritic::Voicing(Voicing::Voiced),
    IpaDiacritic::Aspirated,
    IpaDiacritic::BreathyVoiced,
    IpaDiacritic::CreakyVoiced,
    IpaDiacritic::Modifier(Modifier::Raised),
    IpaDiacritic::Modifier(Modifier::Lowered),
    IpaDiacritic::Modifier(Modifier::Centralized),
    IpaDiacritic::Modifier(Modifier::Advanced),
    IpaDiacritic::Modifier(Modifier::Retracted),
    IpaDiacritic::Modifier(Modifier::Nasalized),
    IpaDiacritic::Modifier(Modifier::Labialized),
    IpaDiacritic::Modifier(Modifier::Palatalized),
    IpaDiacritic::Modifier(Modifier::Velarized),
    IpaDiacritic::Modifier(Modifier::Pharyngealized),
    IpaDiacritic::Modifier(Modifier::Syllabic),
    IpaDiacritic::Modifier(Modifier::NonSyllabic),
    IpaDiacritic::Modifier(Modifier::MoreRounded),
    IpaDiacritic::Modifier(Modifier::LessRounded),
    IpaDiacritic::Modifier(Modifier::MidCentralized),
    IpaDiacritic::Modifier(Modifier::Rhoticity),
    IpaDiacritic::Modifier(Modifier::Linguolabial),
    IpaDiacritic::Modifier(Modifier::VelarizedOrPharyngealized),
    IpaDiacritic::Modifier(Modifier::AdvancedTongueRoot),
    IpaDiacritic::Modifier(Modifier::RetractedTongueRoot),
    IpaDiacritic::Modifier(Modifier::Dental),
    IpaDiacritic::Modifier(Modifier::Apical),
    IpaDiacritic::Modifier(Modifier::Laminal),
    IpaDiacritic::Modifier(Modifier::NasalRelease),
    IpaDiacritic::Modifier(Modifier::LateralRelease),
    IpaDiacritic::Modifier(Modifier::NoAudibleRelease),
];
