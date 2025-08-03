use crate::define_ipa_phoneme;
use crate::phonology::{
    AirflowMechanism, Diacritics, Manner, Place, Subplace, EMPTY_SUPRASEGEMENTALS,
};
use std::borrow::Cow;

// ============================================================================
// NASALE - Systematische Benennung: [BASIS]_[ARTIKULATIONSSTELLE]_[STIMMHAFTIGKEIT]
// ============================================================================

define_ipa_phoneme! {
    name: M_BILABIAL,
    ipa: "m",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::Nasal,
        place: Place::Labial,
        subplace: Subplace::Bilabial,
        diacritics: Cow::Borrowed(&[Diacritics::Voiced]),
        suprasegmentals: EMPTY_SUPRASEGEMENTALS,
    }
}
define_ipa_phoneme! {
    name: M_BILABIAL_VOICELESS,
    ipa: "m̥",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::Nasal,
        place: Place::Labial,
        subplace: Subplace::Bilabial,
        diacritics: Cow::Borrowed(&[Diacritics::Voiceless]),
        suprasegmentals: EMPTY_SUPRASEGEMENTALS,
    }
}
define_ipa_phoneme! {
    name: M_LABIODENTAL,
    ipa: "ɱ",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::Nasal,
        place: Place::Labial,
        subplace: Subplace::Labiodental,
        diacritics: Cow::Borrowed(&[Diacritics::Voiced]),
        suprasegmentals: EMPTY_SUPRASEGEMENTALS,
    }
}
define_ipa_phoneme! {
    name: M_LABIODENTAL_VOICELESS,
    ipa: "ɱ̥",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::Nasal,
        place: Place::Labial,
        subplace: Subplace::Labiodental,
        diacritics: Cow::Borrowed(&[Diacritics::Voiceless]),
        suprasegmentals: EMPTY_SUPRASEGEMENTALS,
    }
}

// Backward Compatibility - Alte Namen als Type Aliases
#[deprecated(since = "0.1.0", note = "Use M_BILABIAL instead")]
pub use M_BILABIAL as M_VOICED;
#[deprecated(since = "0.1.0", note = "Use M_BILABIAL_VOICELESS instead")]
pub use M_BILABIAL_VOICELESS as M_VOICELESS;
#[deprecated(since = "0.1.0", note = "Use M_LABIODENTAL instead")]
pub use M_LABIODENTAL as M_STOP;
#[deprecated(since = "0.1.0", note = "Use M_LABIODENTAL_VOICELESS instead")]
pub use M_LABIODENTAL_VOICELESS as M_STOP_VOICELESS;
define_ipa_phoneme! {
    name: N_LINGUOLABIAL,
    ipa: "n̼",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::Nasal,
        place: Place::Labial,
        subplace: Subplace::Linguolabial,
        diacritics: Cow::Borrowed(&[Diacritics::Voiced]),
        suprasegmentals: EMPTY_SUPRASEGEMENTALS,
    }
}

define_ipa_phoneme! {
    name: N_ALVEOLAR,
    ipa: "n",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::Nasal,
        place: Place::Coronal,
        subplace: Subplace::Alveolar,
        diacritics: Cow::Borrowed(&[Diacritics::Voiced]),
        suprasegmentals: EMPTY_SUPRASEGEMENTALS,
    }
}
define_ipa_phoneme! {
    name: N_ALVEOLAR_VOICELESS,
    ipa: "n̥",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::Nasal,
        place: Place::Coronal,
        subplace: Subplace::Alveolar,
        diacritics: Cow::Borrowed(&[Diacritics::Voiceless]),
        suprasegmentals: EMPTY_SUPRASEGEMENTALS,
    }
}

// Backward Compatibility für N-Laute
#[deprecated(since = "0.1.0", note = "Use N_LINGUOLABIAL instead (typo fixed)")]
pub use N_LINGUOLABIAL as N_LINGOULABIAL;
#[deprecated(since = "0.1.0", note = "Use N_ALVEOLAR instead")]
pub use N_ALVEOLAR as N_VOICED;
#[deprecated(since = "0.1.0", note = "Use N_ALVEOLAR_VOICELESS instead")]
pub use N_ALVEOLAR_VOICELESS as N_VOICELESS;
define_ipa_phoneme! {
    name: N_RETROFLEX,
    ipa: "ɳ",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::Nasal,
        place: Place::Coronal,
        subplace: Subplace::Retroflex,
        diacritics: Cow::Borrowed(&[Diacritics::Voiced]),
        suprasegmentals: EMPTY_SUPRASEGEMENTALS,
    }
}
define_ipa_phoneme! {
    name: N_RETROFLEX_VOICELESS,
    ipa: "ɳ̊",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::Nasal,
        place: Place::Coronal,
        subplace: Subplace::Retroflex,
        diacritics: Cow::Borrowed(&[Diacritics::Voiceless]),
        suprasegmentals: EMPTY_SUPRASEGEMENTALS,
    }
}
define_ipa_phoneme! {
    name: N_PALATAL,
    ipa: "ɲ",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::Nasal,
        place: Place::Coronal,
        subplace: Subplace::Palatal,
        diacritics: Cow::Borrowed(&[Diacritics::Voiced]),
        suprasegmentals: EMPTY_SUPRASEGEMENTALS,
    }
}
define_ipa_phoneme! {
    name: N_PALATAL_VOICELESS,
    ipa: "ɲ̊",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::Nasal,
        place: Place::Coronal,
        subplace: Subplace::Palatal,
        diacritics: Cow::Borrowed(&[Diacritics::Voiceless]),
        suprasegmentals: EMPTY_SUPRASEGEMENTALS,
    }
}
define_ipa_phoneme! {
    name: N_VELAR,
    ipa: "ŋ",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::Nasal,
        place: Place::Dorsal,
        subplace: Subplace::Velar,
        diacritics: Cow::Borrowed(&[Diacritics::Voiced]),
        suprasegmentals: EMPTY_SUPRASEGEMENTALS,
    }
}
define_ipa_phoneme! {
    name: N_VELAR_VOICELESS,
    ipa: "ŋ̊",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::Nasal,
        place: Place::Dorsal,
        subplace: Subplace::Velar,
        diacritics: Cow::Borrowed(&[Diacritics::Voiceless]),
        suprasegmentals: EMPTY_SUPRASEGEMENTALS,
    }
}
define_ipa_phoneme! {
    name: N_UVULAR,
    ipa: "ɴ",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::Nasal,
        place: Place::Dorsal,
        subplace: Subplace::Uvular,
        diacritics: Cow::Borrowed(&[Diacritics::Voiced]),
        suprasegmentals: EMPTY_SUPRASEGEMENTALS,
    }
}
define_ipa_phoneme! {
    name: N_UVULAR_VOICELESS,
    ipa: "ɴ̊",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::Nasal,
        place: Place::Dorsal,
        subplace: Subplace::Uvular,
        diacritics: Cow::Borrowed(&[Diacritics::Voiceless]),
        suprasegmentals: EMPTY_SUPRASEGEMENTALS,
    }
}

// ============================================================================
// PLOSIVE - Systematische Benennung: [BASIS]_[ARTIKULATIONSSTELLE]
// ============================================================================

define_ipa_phoneme! {
    name: P_BILABIAL,
    ipa: "p",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::Plosive,
        place: Place::Labial,
        subplace: Subplace::Bilabial,
        diacritics: Cow::Borrowed(&[Diacritics::Voiceless]),
        suprasegmentals: EMPTY_SUPRASEGEMENTALS,
    }
}
define_ipa_phoneme! {
    name: B_BILABIAL,
    ipa: "b",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::Plosive,
        place: Place::Labial,
        subplace: Subplace::Bilabial,
        diacritics: Cow::Borrowed(&[Diacritics::Voiced]),
        suprasegmentals: EMPTY_SUPRASEGEMENTALS,
    }
}

// Backward Compatibility für einfache Plosive
#[deprecated(since = "0.1.0", note = "Use P_BILABIAL instead")]
pub use P_BILABIAL as P;
#[deprecated(since = "0.1.0", note = "Use B_BILABIAL instead")]
pub use B_BILABIAL as B;
define_ipa_phoneme! {
    name: P_LABIODENTAL,
    ipa: "p̪",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::Plosive,
        place: Place::Labial,
        subplace: Subplace::Labiodental,
        diacritics: Cow::Borrowed(&[Diacritics::Voiceless]),
        suprasegmentals: EMPTY_SUPRASEGEMENTALS,
    }
}
define_ipa_phoneme! {
    name: B_LABIODENTAL,
    ipa: "b̪",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::Plosive,
        place: Place::Labial,
        subplace: Subplace::Labiodental,
        diacritics: Cow::Borrowed(&[Diacritics::Voiced]),
        suprasegmentals: EMPTY_SUPRASEGEMENTALS,
    }
}
define_ipa_phoneme! {
    name: T_LINGUOLABIAL,
    ipa: "t̼",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::Plosive,
        place: Place::Labial,
        subplace: Subplace::Linguolabial,
        diacritics: Cow::Borrowed(&[Diacritics::Voiceless]),
        suprasegmentals: EMPTY_SUPRASEGEMENTALS,
    }
}
define_ipa_phoneme! {
    name: D_LINGUOLABIAL,
    ipa: "d̼",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::Plosive,
        place: Place::Labial,
        subplace: Subplace::Linguolabial,
        diacritics: Cow::Borrowed(&[Diacritics::Voiced]),
        suprasegmentals: EMPTY_SUPRASEGEMENTALS,
    }
}
define_ipa_phoneme! {
    name: T_ALVEOLAR,
    ipa: "t",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::Plosive,
        place: Place::Coronal,
        subplace: Subplace::Alveolar,
        diacritics: Cow::Borrowed(&[Diacritics::Voiceless]),
        suprasegmentals: EMPTY_SUPRASEGEMENTALS,
    }
}
define_ipa_phoneme! {
    name: D_ALVEOLAR,
    ipa: "d",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::Plosive,
        place: Place::Coronal,
        subplace: Subplace::Alveolar,
        diacritics: Cow::Borrowed(&[Diacritics::Voiced]),
        suprasegmentals: EMPTY_SUPRASEGEMENTALS,
    }
}

// Backward Compatibility für T/D
#[deprecated(since = "0.1.0", note = "Use T_ALVEOLAR instead")]
pub use T_ALVEOLAR as T;
#[deprecated(since = "0.1.0", note = "Use D_ALVEOLAR instead")]
pub use D_ALVEOLAR as D;
define_ipa_phoneme! {
    name: T_RETROFLEX,
    ipa: "ʈ",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::Plosive,
        place: Place::Coronal,
        subplace: Subplace::Retroflex,
        diacritics: Cow::Borrowed(&[Diacritics::Voiceless]),
        suprasegmentals: EMPTY_SUPRASEGEMENTALS,
    }
}
define_ipa_phoneme! {
    name: D_RETROFLEX,
    ipa: "ɖ",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::Plosive,
        place: Place::Coronal,
        subplace: Subplace::Retroflex,
        diacritics: Cow::Borrowed(&[Diacritics::Voiced]),
        suprasegmentals: EMPTY_SUPRASEGEMENTALS,
    }
}
define_ipa_phoneme! {
    name: C_PALATAL_PLOSIVE,
    ipa: "c",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::Plosive,
        place: Place::Coronal,
        subplace: Subplace::Palatal,
        diacritics: Cow::Borrowed(&[Diacritics::Voiceless]),
        suprasegmentals: EMPTY_SUPRASEGEMENTALS,
    }
}
define_ipa_phoneme! {
    name: J_PALATAL_PLOSIVE,
    ipa: "ɟ",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::Plosive,
        place: Place::Coronal,
        subplace: Subplace::Palatal,
        diacritics: Cow::Borrowed(&[Diacritics::Voiced]),
        suprasegmentals: EMPTY_SUPRASEGEMENTALS,
    }
}

// Backward Compatibility für Palatale
#[deprecated(since = "0.1.0", note = "Use C_PALATAL_PLOSIVE instead")]
pub use C_PALATAL_PLOSIVE as C;
#[deprecated(since = "0.1.0", note = "Use J_PALATAL_PLOSIVE instead")]
pub use J_PALATAL_PLOSIVE as TURNED_F;
define_ipa_phoneme! {
    name: K_VELAR,
    ipa: "k",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::Plosive,
        place: Place::Dorsal,
        subplace: Subplace::Velar,
        diacritics: Cow::Borrowed(&[Diacritics::Voiceless]),
        suprasegmentals: EMPTY_SUPRASEGEMENTALS,
    }
}
define_ipa_phoneme! {
    name: G_VELAR,
    ipa: "ɡ",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::Plosive,
        place: Place::Dorsal,
        subplace: Subplace::Velar,
        diacritics: Cow::Borrowed(&[Diacritics::Voiced]),
        suprasegmentals: EMPTY_SUPRASEGEMENTALS,
    }
}
define_ipa_phoneme! {
    name: Q_UVULAR,
    ipa: "q",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::Plosive,
        place: Place::Dorsal,
        subplace: Subplace::Uvular,
        diacritics: Cow::Borrowed(&[Diacritics::Voiceless]),
        suprasegmentals: EMPTY_SUPRASEGEMENTALS,
    }
}
define_ipa_phoneme! {
    name: G_UVULAR,
    ipa: "ɢ",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::Plosive,
        place: Place::Dorsal,
        subplace: Subplace::Uvular,
        diacritics: Cow::Borrowed(&[Diacritics::Voiced]),
        suprasegmentals: EMPTY_SUPRASEGEMENTALS,
    }
}

// Backward Compatibility für K/G/Q
#[deprecated(since = "0.1.0", note = "Use K_VELAR instead")]
pub use K_VELAR as K;
#[deprecated(since = "0.1.0", note = "Use G_VELAR instead")]
pub use G_VELAR as G;
#[deprecated(since = "0.1.0", note = "Use Q_UVULAR instead")]
pub use Q_UVULAR as Q;
define_ipa_phoneme! {
    name: STOP_PHARYNGEAL,
    ipa: "ʡ",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::Plosive,
        place: Place::Laryngeal,
        subplace: Subplace::Pharyngeal,
        diacritics: Cow::Borrowed(&[Diacritics::Voiceless]),
        suprasegmentals: EMPTY_SUPRASEGEMENTALS,
    }
}
define_ipa_phoneme! {
    name: STOP_GLOTTAL,
    ipa: "ʔ",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::Plosive,
        place: Place::Laryngeal,
        subplace: Subplace::Glottal,
        diacritics: Cow::Borrowed(&[Diacritics::Voiceless]),
        suprasegmentals: EMPTY_SUPRASEGEMENTALS,
    }
}

// Backward Compatibility für Glottale
#[deprecated(since = "0.1.0", note = "Use STOP_PHARYNGEAL instead")]
pub use STOP_PHARYNGEAL as GLOTTAL_BAR;
#[deprecated(since = "0.1.0", note = "Use STOP_GLOTTAL instead")]
pub use STOP_GLOTTAL as GLOTTAL;

// ============================================================================
// SIBILANTE - Systematische Benennung: [S/Z]_[ARTIKULATIONSSTELLE]
// ============================================================================

define_ipa_phoneme! {
    name: S_ALVEOLAR,
    ipa: "s",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::SibilantFricative,
        place: Place::Coronal,
        subplace: Subplace::Alveolar,
        diacritics: Cow::Borrowed(&[Diacritics::Voiceless]),
        suprasegmentals: EMPTY_SUPRASEGEMENTALS,
    }
}
define_ipa_phoneme! {
    name: Z_ALVEOLAR,
    ipa: "z",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::SibilantFricative,
        place: Place::Coronal,
        subplace: Subplace::Alveolar,
        diacritics: Cow::Borrowed(&[Diacritics::Voiced]),
        suprasegmentals: EMPTY_SUPRASEGEMENTALS,
    }
}
define_ipa_phoneme! {
    name: S_POSTALVEOLAR,
    ipa: "ʃ",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::SibilantFricative,
        place: Place::Coronal,
        subplace: Subplace::Postalveolar,
        diacritics: Cow::Borrowed(&[Diacritics::Voiceless]),
        suprasegmentals: EMPTY_SUPRASEGEMENTALS,
    }
}
define_ipa_phoneme! {
    name: Z_POSTALVEOLAR,
    ipa: "ʒ",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::SibilantFricative,
        place: Place::Coronal,
        subplace: Subplace::Postalveolar,
        diacritics: Cow::Borrowed(&[Diacritics::Voiced]),
        suprasegmentals: EMPTY_SUPRASEGEMENTALS,
    }
}

// Backward Compatibility für Sibilante
#[deprecated(since = "0.1.0", note = "Use S_ALVEOLAR instead")]
pub use S_ALVEOLAR as S;
#[deprecated(since = "0.1.0", note = "Use Z_ALVEOLAR instead")]
pub use Z_ALVEOLAR as Z;
#[deprecated(since = "0.1.0", note = "Use S_POSTALVEOLAR instead")]
pub use S_POSTALVEOLAR as SH;
#[deprecated(since = "0.1.0", note = "Use Z_POSTALVEOLAR instead")]
pub use Z_POSTALVEOLAR as ZH;
define_ipa_phoneme! {
    name: S_RETROFLEX,
    ipa: "ʂ",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::SibilantFricative,
        place: Place::Coronal,
        subplace: Subplace::Retroflex,
        diacritics: Cow::Borrowed(&[Diacritics::Voiceless]),
        suprasegmentals: EMPTY_SUPRASEGEMENTALS,
    }
}
define_ipa_phoneme! {
    name: Z_RETROFLEX,
    ipa: "ʐ",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::SibilantFricative,
        place: Place::Coronal,
        subplace: Subplace::Retroflex,
        diacritics: Cow::Borrowed(&[Diacritics::Voiced]),
        suprasegmentals: EMPTY_SUPRASEGEMENTALS,
    }
}
define_ipa_phoneme! {
    name: S_PALATAL,
    ipa: "ɕ",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::SibilantFricative,
        place: Place::Coronal,
        subplace: Subplace::Palatal,
        diacritics: Cow::Borrowed(&[Diacritics::Voiceless]),
        suprasegmentals: EMPTY_SUPRASEGEMENTALS,
    }
}
define_ipa_phoneme! {
    name: Z_PALATAL,
    ipa: "ʑ",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::SibilantFricative,
        place: Place::Coronal,
        subplace: Subplace::Palatal,
        diacritics: Cow::Borrowed(&[Diacritics::Voiced]),
        suprasegmentals: EMPTY_SUPRASEGEMENTALS,
    }
}

// Non-Fircatives
define_ipa_phoneme! {
    name: PH,
    ipa: "ɸ",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::NonSibilantFricative,
        place: Place::Labial,
        subplace: Subplace::Bilabial,
        diacritics: Cow::Borrowed(&[Diacritics::Voiceless]),
        suprasegmentals: EMPTY_SUPRASEGEMENTALS,
    }
}
define_ipa_phoneme! {
    name: BH,
    ipa: "β",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::NonSibilantFricative,
        place: Place::Labial,
        subplace: Subplace::Bilabial,
        diacritics: Cow::Borrowed(&[Diacritics::Voiced]),
        suprasegmentals: EMPTY_SUPRASEGEMENTALS,
    }
}
define_ipa_phoneme! {
    name: F,
    ipa: "f",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::NonSibilantFricative,
        place: Place::Labial,
        subplace: Subplace::Labiodental,
        diacritics: Cow::Borrowed(&[Diacritics::Voiceless]),
        suprasegmentals: EMPTY_SUPRASEGEMENTALS,
    }
}
define_ipa_phoneme! {
    name: V,
    ipa: "v",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::NonSibilantFricative,
        place: Place::Labial,
        subplace: Subplace::Labiodental,
        diacritics: Cow::Borrowed(&[Diacritics::Voiced]),
        suprasegmentals: EMPTY_SUPRASEGEMENTALS,
    }
}
define_ipa_phoneme! {
    name: TH_LINGUOLABIAL,
    ipa: "θ̼",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::NonSibilantFricative,
        place: Place::Labial,
        subplace: Subplace::Linguolabial,
        diacritics: Cow::Borrowed(&[Diacritics::Voiceless]),
        suprasegmentals: EMPTY_SUPRASEGEMENTALS,
    }
}
define_ipa_phoneme! {
    name: DH_LINGUOLABIAL,
    ipa: "ð̼",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::NonSibilantFricative,
        place: Place::Labial,
        subplace: Subplace::Linguolabial,
        diacritics: Cow::Borrowed(&[Diacritics::Voiced]),
        suprasegmentals: EMPTY_SUPRASEGEMENTALS,
    }
}
define_ipa_phoneme! {
    name: TH,
    ipa: "θ",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::NonSibilantFricative,
        place: Place::Coronal,
        subplace: Subplace::Dental,
        diacritics: Cow::Borrowed(&[Diacritics::Voiceless]),
        suprasegmentals: EMPTY_SUPRASEGEMENTALS,
    }
}
define_ipa_phoneme! {
    name: DH,
    ipa: "ð",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::NonSibilantFricative,
        place: Place::Coronal,
        subplace: Subplace::Dental,
        diacritics: Cow::Borrowed(&[Diacritics::Voiced]),
        suprasegmentals: EMPTY_SUPRASEGEMENTALS,
    }
}
define_ipa_phoneme! {
    name: TH_ALVEOLAR,
    ipa: "θ̠",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::NonSibilantFricative,
        place: Place::Coronal,
        subplace: Subplace::Alveolar,
        diacritics: Cow::Borrowed(&[Diacritics::Voiceless]),
        suprasegmentals: EMPTY_SUPRASEGEMENTALS,
    }
}
define_ipa_phoneme! {
    name: DH_ALVEOLAR,
    ipa: "ð̠",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::NonSibilantFricative,
        place: Place::Coronal,
        subplace: Subplace::Alveolar,
        diacritics: Cow::Borrowed(&[Diacritics::Voiced]),
        suprasegmentals: EMPTY_SUPRASEGEMENTALS,
    }
}
define_ipa_phoneme! {
    name: TURNED_R_VOIVELESS_RETRACTED_RAISED,
    ipa: "ɹ̠̊˔",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::NonSibilantFricative,
        place: Place::Coronal,
        subplace: Subplace::Alveolar,
        diacritics: Cow::Borrowed(&[Diacritics::Voiceless, Diacritics::Retracted, Diacritics::Raised]),
        suprasegmentals: EMPTY_SUPRASEGEMENTALS,
    }
}
define_ipa_phoneme! {
    name: TURNED_R_RETRACTED_RAISED,
    ipa: "ɹ̠˔",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::NonSibilantFricative,
        place: Place::Coronal,
        subplace: Subplace::Alveolar,
        diacritics: Cow::Borrowed(&[Diacritics::Voiced, Diacritics::Retracted, Diacritics::Raised]),
        suprasegmentals: EMPTY_SUPRASEGEMENTALS,
    }
}
define_ipa_phoneme! {
    name:TURNED_R_VOIVELESS_RETROFLEX_RAISED,
    ipa: "ɻ̊˔",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::NonSibilantFricative,
        place: Place::Coronal,
        subplace: Subplace::Retroflex,
        diacritics: Cow::Borrowed(&[Diacritics::Voiceless, Diacritics::Raised]),
        suprasegmentals: EMPTY_SUPRASEGEMENTALS,
    }
}
define_ipa_phoneme! {
    name: TURNED_R_RETROFLEX_RAISED,
    ipa: "ɻ˔",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::NonSibilantFricative,
        place: Place::Coronal,
        subplace: Subplace::Retroflex,
        diacritics: Cow::Borrowed(&[Diacritics::Voiced, Diacritics::Raised]),
        suprasegmentals: EMPTY_SUPRASEGEMENTALS,
    }
}
define_ipa_phoneme! {
    name: F_PALATAL,
    ipa: "ç",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::NonSibilantFricative,
        place: Place::Coronal,
        subplace: Subplace::Palatal,
        diacritics: Cow::Borrowed(&[Diacritics::Voiceless]),
        suprasegmentals: EMPTY_SUPRASEGEMENTALS,
    }
}
define_ipa_phoneme! {
    name: V_PALATAL,
    ipa: "ʝ",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::NonSibilantFricative,
        place: Place::Coronal,
        subplace: Subplace::Palatal,
        diacritics: Cow::Borrowed(&[Diacritics::Voiced]),
        suprasegmentals: EMPTY_SUPRASEGEMENTALS,
    }
}
define_ipa_phoneme! {
    name: X,
    ipa: "x",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::NonSibilantFricative,
        place: Place::Dorsal,
        subplace: Subplace::Velar,
        diacritics: Cow::Borrowed(&[Diacritics::Voiceless]),
        suprasegmentals: EMPTY_SUPRASEGEMENTALS,
    }
}
define_ipa_phoneme! {
    name: RAMS_HORN,
    ipa: "ɣ",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::NonSibilantFricative,
        place: Place::Dorsal,
        subplace: Subplace::Velar,
        diacritics: Cow::Borrowed(&[Diacritics::Voiced]),
        suprasegmentals: EMPTY_SUPRASEGEMENTALS,
    }
}
define_ipa_phoneme! {
    name: X_UVULAR,
    ipa: "χ",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::NonSibilantFricative,
        place: Place::Dorsal,
        subplace: Subplace::Uvular,
        diacritics: Cow::Borrowed(&[Diacritics::Voiceless]),
        suprasegmentals: EMPTY_SUPRASEGEMENTALS,
    }
}
define_ipa_phoneme! {
    name: REVERSED_TRUNED_R,
    ipa: "ʁ",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::NonSibilantFricative,
        place: Place::Dorsal,
        subplace: Subplace::Uvular,
        diacritics: Cow::Borrowed(&[Diacritics::Voiced]),
        suprasegmentals: EMPTY_SUPRASEGEMENTALS,
    }
}
define_ipa_phoneme! {
    name: H_BAR,
    ipa: "ħ",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::NonSibilantFricative,
        place: Place::Laryngeal,
        subplace: Subplace::Pharyngeal,
        diacritics: Cow::Borrowed(&[Diacritics::Voiceless]),
        suprasegmentals: EMPTY_SUPRASEGEMENTALS,
    }
}
define_ipa_phoneme! {
    name: GH_PHARYNGEAL,
    ipa: "ʕ",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::NonSibilantFricative,
        place: Place::Laryngeal,
        subplace: Subplace::Pharyngeal,
        diacritics: Cow::Borrowed(&[Diacritics::Voiced]),
        suprasegmentals: EMPTY_SUPRASEGEMENTALS,
    }
}
define_ipa_phoneme! {
    name: H,
    ipa: "h",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::NonSibilantFricative,
        place: Place::Laryngeal,
        subplace: Subplace::Glottal,
        diacritics: Cow::Borrowed(&[Diacritics::Voiceless]),
        suprasegmentals: EMPTY_SUPRASEGEMENTALS,
    }
}
define_ipa_phoneme! {
    name: H_VOICED,
    ipa: "ɦ",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::NonSibilantFricative,
        place: Place::Laryngeal,
        subplace: Subplace::Glottal,
        diacritics: Cow::Borrowed(&[Diacritics::Voiced]),
        suprasegmentals: EMPTY_SUPRASEGEMENTALS,
    }
}

define_ipa_phoneme! {
    name: ROUNDED_V,
    ipa: "ʋ",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::Approximant,
        place: Place::Labial,
        subplace: Subplace::Labiodental,
        diacritics: Cow::Borrowed(&[Diacritics::Voiced]),
        suprasegmentals: EMPTY_SUPRASEGEMENTALS,
    }
}
define_ipa_phoneme! {
    name: TURNED_R,
    ipa: "ɹ",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::Approximant,
        place: Place::Coronal,
        subplace: Subplace::Alveolar,
        diacritics: Cow::Borrowed(&[Diacritics::Voiced]),
        suprasegmentals: EMPTY_SUPRASEGEMENTALS,
    }
}
define_ipa_phoneme! {
    name: TURNED_R_RETROFLEX,
    ipa: "ɻ",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::Approximant,
        place: Place::Coronal,
        subplace: Subplace::Retroflex,
        diacritics: Cow::Borrowed(&[Diacritics::Voiced]),
        suprasegmentals: EMPTY_SUPRASEGEMENTALS,
    }
}
define_ipa_phoneme! {
    name: J,
    ipa: "j",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::Approximant,
        place: Place::Coronal,
        subplace: Subplace::Palatal,
        diacritics: Cow::Borrowed(&[Diacritics::Voiced]),
        suprasegmentals: EMPTY_SUPRASEGEMENTALS,
    }
}
define_ipa_phoneme! {
    name: TURNED_M_VELAR,
    ipa: "ɰ",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::Approximant,
        place: Place::Dorsal,
        subplace: Subplace::Velar,
        diacritics: Cow::Borrowed(&[Diacritics::Voiced]),
        suprasegmentals: EMPTY_SUPRASEGEMENTALS,
    }
}
define_ipa_phoneme! {
    name: GLOTTAL_RAISED,
    ipa: "ʔ̞",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::Approximant,
        place: Place::Laryngeal,
        subplace: Subplace::Glottal,
        diacritics: Cow::Borrowed(&[Diacritics::Voiced, Diacritics::Raised]),
        suprasegmentals: EMPTY_SUPRASEGEMENTALS,
    }
}

define_ipa_phoneme! {
    name: V_RIGHT_HOOK_ADVANCED,
    ipa: "ⱱ̟",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::TapFlap,
        place: Place::Labial,
        subplace: Subplace::Bilabial,
        diacritics: Cow::Borrowed(&[Diacritics::Voiced, Diacritics::Advanced]),
        suprasegmentals: EMPTY_SUPRASEGEMENTALS,
    }
}
define_ipa_phoneme! {
    name: V_RIGHT_HOOK,
    ipa: "ⱱ",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::TapFlap,
        place: Place::Labial,
        subplace: Subplace::Bilabial,
        diacritics: Cow::Borrowed(&[Diacritics::Voiced]),
        suprasegmentals: EMPTY_SUPRASEGEMENTALS,
    }
}
define_ipa_phoneme! {
    name: REVERSED_ROUNDED_R_LINGUOLABIAL,
    ipa: "ɾ̼",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::TapFlap,
        place: Place::Labial,
        subplace: Subplace::Linguolabial,
        diacritics: Cow::Borrowed(&[Diacritics::Voiced]),
        suprasegmentals: EMPTY_SUPRASEGEMENTALS,
    }
}
define_ipa_phoneme! {
    name: REVERSED_ROUNDED_R_,
    ipa: "ɾ",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::TapFlap,
        place: Place::Coronal,
        subplace: Subplace::Alveolar,
        diacritics: Cow::Borrowed(&[Diacritics::Voiced]),
        suprasegmentals: EMPTY_SUPRASEGEMENTALS,
    }
}
define_ipa_phoneme! {
    name: REVERSED_ROUNDED_R_VOICELESS,
    ipa: "ɾ̥",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::TapFlap,
        place: Place::Coronal,
        subplace: Subplace::Alveolar,
        diacritics: Cow::Borrowed(&[Diacritics::Voiceless]),
        suprasegmentals: EMPTY_SUPRASEGEMENTALS,
    }
}

// Continue at TAP/RETROFLEX => https://en.wikipedia.org/wiki/International_Phonetic_Alphabet
