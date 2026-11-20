use super::*;
use std::fmt;

macro_rules! names {
    ($ty:ty, $($variant:ident => $name:literal),+ $(,)?) => {
        impl fmt::Display for $ty {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(match self { $(Self::$variant => $name,)+ })
            }
        }
    }
}
names!(AirflowMechanism, Pulmonic => "pulmonic", Click => "click", Ejective => "ejective", Implosive => "implosive");
names!(Voicing, Voiced => "voiced", Voiceless => "voiceless");
names!(Phonation, Modal => "modal", Breathy => "breathy voiced", Creaky => "creaky voiced");
names!(TongueHeight, Close => "close", NearClose => "near-close", CloseMid => "close-mid", Mid => "mid", OpenMid => "open-mid", NearOpen => "near-open", Open => "open");
names!(TonguePosition, Front => "front", NearFront => "near-front", Central => "central", NearBack => "near-back", Back => "back");
names!(Roundness, Rounded => "rounded", Unrounded => "unrounded");
names!(ArticulationPlace,
    Bilabial => "bilabial", Labiodental => "labiodental", Linguolabial => "linguolabial",
    Dental => "dental", Alveolar => "alveolar", Postalveolar => "postalveolar",
    AlveoloPalatal => "alveolo-palatal", Retroflex => "retroflex", Palatal => "palatal",
    Velar => "velar", Uvular => "uvular", Pharyngeal => "pharyngeal", Epiglottal => "epiglottal",
    Glottal => "glottal", LabialVelar => "labial-velar", LabialPalatal => "labial-palatal",
    PostalveolarVelar => "simultaneous postalveolar-velar");
names!(Manner,
    Nasal => "nasal", Plosive => "plosive", SibilantFricative => "sibilant fricative",
    NonSibilantFricative => "non-sibilant fricative", Fricative => "fricative",
    Approximant => "approximant", TapFlap => "tap or flap", Trill => "trill",
    LateralApproximant => "lateral approximant", LateralFricative => "lateral fricative",
    LateralTapFlap => "lateral flap", LateralPlosive => "lateral plosive", Affricate => "affricate");
names!(Modifier,
    Raised => "raised", Lowered => "lowered", Centralized => "centralized", Advanced => "advanced",
    Retracted => "retracted", Nasalized => "nasalized", Labialized => "labialized",
    Palatalized => "palatalized", Velarized => "velarized", Pharyngealized => "pharyngealized",
    Syllabic => "syllabic", NonSyllabic => "non-syllabic", MoreRounded => "more rounded",
    LessRounded => "less rounded", MidCentralized => "mid-centralized", Rhoticity => "rhotic",
    Linguolabial => "linguolabial", VelarizedOrPharyngealized => "velarized or pharyngealized",
    AdvancedTongueRoot => "advanced tongue root", RetractedTongueRoot => "retracted tongue root",
    Dental => "dental", Apical => "apical", Laminal => "laminal", NasalRelease => "nasal release",
    LateralRelease => "lateral release", NoAudibleRelease => "no audible release");

impl SoundDefinition {
    /// Human-readable, derived from the authoritative features rather than Rust identifiers.
    pub fn name(&self) -> String {
        let mut name = match self.features() {
            SoundFeatures::Consonant(f) => {
                let manner = if f.airflow == AirflowMechanism::Click {
                    if f.manner == Manner::LateralPlosive {
                        "lateral click".to_owned()
                    } else {
                        "click".to_owned()
                    }
                } else {
                    f.manner.to_string()
                };
                let mut s = format!(
                    "{} {} {}",
                    f.voicing,
                    f.affricate_release.unwrap_or(f.place),
                    manner
                );
                if f.airflow == AirflowMechanism::Ejective
                    || f.airflow == AirflowMechanism::Implosive
                {
                    s.push_str(&format!(" ({})", f.airflow));
                }
                if f.aspiration == Aspiration::Aspirated {
                    s.push_str(", aspirated");
                }
                if f.phonation != Phonation::Modal {
                    s.push_str(&format!(", {}", f.phonation));
                }
                s
            }
            SoundFeatures::Vowel(f) => {
                let mut s = format!(
                    "{} {} {} {} vowel",
                    f.voicing, f.height, f.position, f.roundness
                );
                if f.phonation != Phonation::Modal {
                    s.push_str(&format!(", {}", f.phonation));
                }
                s
            }
        };
        for modifier in self.modifiers() {
            name.push_str(&format!(", {modifier}"));
        }
        name
    }
    pub fn description(&self) -> String {
        format!("{} — {}", self.ipa(), self.name())
    }
}
