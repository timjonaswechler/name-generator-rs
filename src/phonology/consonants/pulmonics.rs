use crate::phonology::macros::define_ipa_phoneme;
use crate::phonology::{AirflowMechanism, ArticulationPlace, Manner, Modifier, Voicing};
use std::borrow::Cow;

// ============================================================================
// NASALE - Systematische Benennung: [BASIS]_[ARTIKULATIONSSTELLE]_[STIMMHAFTIGKEIT]
// ============================================================================

define_ipa_phoneme! {
    name: M_BILABIAL,
    id: "m_bilabial",
    ipa: "m",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::Nasal,
        place: ArticulationPlace::Bilabial,
        voicing: Voicing::Voiced,
        modifiers: Cow::Borrowed(&[]),
    }
}
define_ipa_phoneme! {
    name: M_BILABIAL_VOICELESS,
    id: "m_bilabial_voiceless",
    ipa: "m̥",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::Nasal,
        place: ArticulationPlace::Bilabial,
        voicing: Voicing::Voiceless,
        modifiers: Cow::Borrowed(&[]),
    }
}
define_ipa_phoneme! {
    name: M_LABIODENTAL,
    id: "m_labiodental",
    ipa: "ɱ",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::Nasal,
        place: ArticulationPlace::Labiodental,
        voicing: Voicing::Voiced,
        modifiers: Cow::Borrowed(&[]),
    }
}
define_ipa_phoneme! {
    name: M_LABIODENTAL_VOICELESS,
    id: "m_labiodental_voiceless",
    ipa: "ɱ̥",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::Nasal,
        place: ArticulationPlace::Labiodental,
        voicing: Voicing::Voiceless,
        modifiers: Cow::Borrowed(&[]),
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
    id: "n_linguolabial",
    ipa: "n̼",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::Nasal,
        place: ArticulationPlace::Linguolabial,
        voicing: Voicing::Voiced,
        modifiers: Cow::Borrowed(&[]),
    }
}

define_ipa_phoneme! {
    name: N_ALVEOLAR,
    id: "n_alveolar",
    ipa: "n",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::Nasal,
        place: ArticulationPlace::Alveolar,
        voicing: Voicing::Voiced,
        modifiers: Cow::Borrowed(&[]),
    }
}
define_ipa_phoneme! {
    name: N_ALVEOLAR_VOICELESS,
    id: "n_alveolar_voiceless",
    ipa: "n̥",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::Nasal,
        place: ArticulationPlace::Alveolar,
        voicing: Voicing::Voiceless,
        modifiers: Cow::Borrowed(&[]),
    }
}

// Backward Compatibility für N-Laute
#[deprecated(since = "0.1.0", note = "Use N_ALVEOLAR instead")]
pub use N_ALVEOLAR as N_VOICED;
#[deprecated(since = "0.1.0", note = "Use N_ALVEOLAR_VOICELESS instead")]
pub use N_ALVEOLAR_VOICELESS as N_VOICELESS;
#[deprecated(since = "0.1.0", note = "Use N_LINGUOLABIAL instead (typo fixed)")]
pub use N_LINGUOLABIAL as N_LINGOULABIAL;
define_ipa_phoneme! {
    name: N_RETROFLEX,
    id: "n_retroflex",
    ipa: "ɳ",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::Nasal,
        place: ArticulationPlace::Retroflex,
        voicing: Voicing::Voiced,
        modifiers: Cow::Borrowed(&[]),
    }
}
define_ipa_phoneme! {
    name: N_RETROFLEX_VOICELESS,
    id: "n_retroflex_voiceless",
    ipa: "ɳ̊",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::Nasal,
        place: ArticulationPlace::Retroflex,
        voicing: Voicing::Voiceless,
        modifiers: Cow::Borrowed(&[]),
    }
}
define_ipa_phoneme! {
    name: N_PALATAL,
    id: "n_palatal",
    ipa: "ɲ",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::Nasal,
        place: ArticulationPlace::Palatal,
        voicing: Voicing::Voiced,
        modifiers: Cow::Borrowed(&[]),
    }
}
define_ipa_phoneme! {
    name: N_PALATAL_VOICELESS,
    id: "n_palatal_voiceless",
    ipa: "ɲ̊",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::Nasal,
        place: ArticulationPlace::Palatal,
        voicing: Voicing::Voiceless,
        modifiers: Cow::Borrowed(&[]),
    }
}
define_ipa_phoneme! {
    name: N_VELAR,
    id: "n_velar",
    ipa: "ŋ",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::Nasal,
        place: ArticulationPlace::Velar,
        voicing: Voicing::Voiced,
        modifiers: Cow::Borrowed(&[]),
    }
}
define_ipa_phoneme! {
    name: N_VELAR_VOICELESS,
    id: "n_velar_voiceless",
    ipa: "ŋ̊",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::Nasal,
        place: ArticulationPlace::Velar,
        voicing: Voicing::Voiceless,
        modifiers: Cow::Borrowed(&[]),
    }
}
define_ipa_phoneme! {
    name: N_UVULAR,
    id: "n_uvular",
    ipa: "ɴ",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::Nasal,
        place: ArticulationPlace::Uvular,
        voicing: Voicing::Voiced,
        modifiers: Cow::Borrowed(&[]),
    }
}
define_ipa_phoneme! {
    name: N_UVULAR_VOICELESS,
    id: "n_uvular_voiceless",
    ipa: "ɴ̊",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::Nasal,
        place: ArticulationPlace::Uvular,
        voicing: Voicing::Voiceless,
        modifiers: Cow::Borrowed(&[]),
    }
}

// ============================================================================
// PLOSIVE - Systematische Benennung: [BASIS]_[ARTIKULATIONSSTELLE]
// ============================================================================

define_ipa_phoneme! {
    name: P_BILABIAL,
    id: "p_bilabial",
    ipa: "p",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::Plosive,
        place: ArticulationPlace::Bilabial,
        voicing: Voicing::Voiceless,
        modifiers: Cow::Borrowed(&[]),
    }
}
define_ipa_phoneme! {
    name: B_BILABIAL,
    id: "b_bilabial",
    ipa: "b",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::Plosive,
        place: ArticulationPlace::Bilabial,
        voicing: Voicing::Voiced,
        modifiers: Cow::Borrowed(&[]),
    }
}

// Backward Compatibility für einfache Plosive
#[deprecated(since = "0.1.0", note = "Use B_BILABIAL instead")]
pub use B_BILABIAL as B;
#[deprecated(since = "0.1.0", note = "Use P_BILABIAL instead")]
pub use P_BILABIAL as P;
define_ipa_phoneme! {
    name: P_LABIODENTAL,
    id: "p_labiodental",
    ipa: "p̪",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::Plosive,
        place: ArticulationPlace::Labiodental,
        voicing: Voicing::Voiceless,
        modifiers: Cow::Borrowed(&[]),
    }
}
define_ipa_phoneme! {
    name: B_LABIODENTAL,
    id: "b_labiodental",
    ipa: "b̪",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::Plosive,
        place: ArticulationPlace::Labiodental,
        voicing: Voicing::Voiced,
        modifiers: Cow::Borrowed(&[]),
    }
}
define_ipa_phoneme! {
    name: T_LINGUOLABIAL,
    id: "t_linguolabial",
    ipa: "t̼",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::Plosive,
        place: ArticulationPlace::Linguolabial,
        voicing: Voicing::Voiceless,
        modifiers: Cow::Borrowed(&[]),
    }
}
define_ipa_phoneme! {
    name: D_LINGUOLABIAL,
    id: "d_linguolabial",
    ipa: "d̼",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::Plosive,
        place: ArticulationPlace::Linguolabial,
        voicing: Voicing::Voiced,
        modifiers: Cow::Borrowed(&[]),
    }
}
define_ipa_phoneme! {
    name: T_ALVEOLAR,
    id: "t_alveolar",
    ipa: "t",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::Plosive,
        place: ArticulationPlace::Alveolar,
        voicing: Voicing::Voiceless,
        modifiers: Cow::Borrowed(&[]),
    }
}
define_ipa_phoneme! {
    name: D_ALVEOLAR,
    id: "d_alveolar",
    ipa: "d",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::Plosive,
        place: ArticulationPlace::Alveolar,
        voicing: Voicing::Voiced,
        modifiers: Cow::Borrowed(&[]),
    }
}

// Backward Compatibility für T/D
#[deprecated(since = "0.1.0", note = "Use D_ALVEOLAR instead")]
pub use D_ALVEOLAR as D;
#[deprecated(since = "0.1.0", note = "Use T_ALVEOLAR instead")]
pub use T_ALVEOLAR as T;
define_ipa_phoneme! {
    name: T_RETROFLEX,
    id: "t_retroflex",
    ipa: "ʈ",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::Plosive,
        place: ArticulationPlace::Retroflex,
        voicing: Voicing::Voiceless,
        modifiers: Cow::Borrowed(&[]),
    }
}
define_ipa_phoneme! {
    name: D_RETROFLEX,
    id: "d_retroflex",
    ipa: "ɖ",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::Plosive,
        place: ArticulationPlace::Retroflex,
        voicing: Voicing::Voiced,
        modifiers: Cow::Borrowed(&[]),
    }
}
define_ipa_phoneme! {
    name: C_PALATAL_PLOSIVE,
    id: "c_palatal_plosive",
    ipa: "c",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::Plosive,
        place: ArticulationPlace::Palatal,
        voicing: Voicing::Voiceless,
        modifiers: Cow::Borrowed(&[]),
    }
}
define_ipa_phoneme! {
    name: J_PALATAL_PLOSIVE,
    id: "j_palatal_plosive",
    ipa: "ɟ",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::Plosive,
        place: ArticulationPlace::Palatal,
        voicing: Voicing::Voiced,
        modifiers: Cow::Borrowed(&[]),
    }
}

// Backward Compatibility für Palatale
#[deprecated(since = "0.1.0", note = "Use C_PALATAL_PLOSIVE instead")]
pub use C_PALATAL_PLOSIVE as C;
#[deprecated(since = "0.1.0", note = "Use J_PALATAL_PLOSIVE instead")]
pub use J_PALATAL_PLOSIVE as TURNED_F;
define_ipa_phoneme! {
    name: K_VELAR,
    id: "k_velar",
    ipa: "k",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::Plosive,
        place: ArticulationPlace::Velar,
        voicing: Voicing::Voiceless,
        modifiers: Cow::Borrowed(&[]),
    }
}
define_ipa_phoneme! {
    name: G_VELAR,
    id: "g_velar",
    ipa: "ɡ",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::Plosive,
        place: ArticulationPlace::Velar,
        voicing: Voicing::Voiced,
        modifiers: Cow::Borrowed(&[]),
    }
}
define_ipa_phoneme! {
    name: Q_UVULAR,
    id: "q_uvular",
    ipa: "q",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::Plosive,
        place: ArticulationPlace::Uvular,
        voicing: Voicing::Voiceless,
        modifiers: Cow::Borrowed(&[]),
    }
}
define_ipa_phoneme! {
    name: G_UVULAR,
    id: "g_uvular",
    ipa: "ɢ",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::Plosive,
        place: ArticulationPlace::Uvular,
        voicing: Voicing::Voiced,
        modifiers: Cow::Borrowed(&[]),
    }
}

// Backward Compatibility für K/G/Q
#[deprecated(since = "0.1.0", note = "Use G_VELAR instead")]
pub use G_VELAR as G;
#[deprecated(since = "0.1.0", note = "Use K_VELAR instead")]
pub use K_VELAR as K;
#[deprecated(since = "0.1.0", note = "Use Q_UVULAR instead")]
pub use Q_UVULAR as Q;
define_ipa_phoneme! {
    name: STOP_EPIGLOTTAL,
    id: "stop_epiglottal",
    ipa: "ʡ",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::Plosive,
        place: ArticulationPlace::Epiglottal,
        voicing: Voicing::Voiceless,
        modifiers: Cow::Borrowed(&[]),
    }
}
define_ipa_phoneme! {
    name: STOP_GLOTTAL,
    id: "stop_glottal",
    ipa: "ʔ",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::Plosive,
        place: ArticulationPlace::Glottal,
        voicing: Voicing::Voiceless,
        modifiers: Cow::Borrowed(&[]),
    }
}

// Backward Compatibility für Glottale
#[deprecated(since = "0.1.0", note = "Use STOP_EPIGLOTTAL instead")]
pub use STOP_EPIGLOTTAL as GLOTTAL_BAR;
#[deprecated(since = "0.1.0", note = "Use STOP_GLOTTAL instead")]
pub use STOP_GLOTTAL as GLOTTAL;

// ============================================================================
// SIBILANTE - Systematische Benennung: [S/Z]_[ARTIKULATIONSSTELLE]
// ============================================================================

define_ipa_phoneme! {
    name: S_ALVEOLAR,
    id: "s_alveolar",
    ipa: "s",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::SibilantFricative,
        place: ArticulationPlace::Alveolar,
        voicing: Voicing::Voiceless,
        modifiers: Cow::Borrowed(&[]),
    }
}
define_ipa_phoneme! {
    name: Z_ALVEOLAR,
    id: "z_alveolar",
    ipa: "z",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::SibilantFricative,
        place: ArticulationPlace::Alveolar,
        voicing: Voicing::Voiced,
        modifiers: Cow::Borrowed(&[]),
    }
}
define_ipa_phoneme! {
    name: S_POSTALVEOLAR,
    id: "s_postalveolar",
    ipa: "ʃ",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::SibilantFricative,
        place: ArticulationPlace::Postalveolar,
        voicing: Voicing::Voiceless,
        modifiers: Cow::Borrowed(&[]),
    }
}
define_ipa_phoneme! {
    name: Z_POSTALVEOLAR,
    id: "z_postalveolar",
    ipa: "ʒ",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::SibilantFricative,
        place: ArticulationPlace::Postalveolar,
        voicing: Voicing::Voiced,
        modifiers: Cow::Borrowed(&[]),
    }
}

// Backward Compatibility für Sibilante
#[deprecated(since = "0.1.0", note = "Use S_ALVEOLAR instead")]
pub use S_ALVEOLAR as S;
#[deprecated(since = "0.1.0", note = "Use S_POSTALVEOLAR instead")]
pub use S_POSTALVEOLAR as SH;
#[deprecated(since = "0.1.0", note = "Use Z_ALVEOLAR instead")]
pub use Z_ALVEOLAR as Z;
#[deprecated(since = "0.1.0", note = "Use Z_POSTALVEOLAR instead")]
pub use Z_POSTALVEOLAR as ZH;
define_ipa_phoneme! {
    name: S_RETROFLEX,
    id: "s_retroflex",
    ipa: "ʂ",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::SibilantFricative,
        place: ArticulationPlace::Retroflex,
        voicing: Voicing::Voiceless,
        modifiers: Cow::Borrowed(&[]),
    }
}
define_ipa_phoneme! {
    name: Z_RETROFLEX,
    id: "z_retroflex",
    ipa: "ʐ",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::SibilantFricative,
        place: ArticulationPlace::Retroflex,
        voicing: Voicing::Voiced,
        modifiers: Cow::Borrowed(&[]),
    }
}
define_ipa_phoneme! {
    name: S_PALATAL,
    id: "s_palatal",
    ipa: "ɕ",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::SibilantFricative,
        place: ArticulationPlace::AlveoloPalatal,
        voicing: Voicing::Voiceless,
        modifiers: Cow::Borrowed(&[]),
    }
}
define_ipa_phoneme! {
    name: Z_PALATAL,
    id: "z_palatal",
    ipa: "ʑ",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::SibilantFricative,
        place: ArticulationPlace::AlveoloPalatal,
        voicing: Voicing::Voiced,
        modifiers: Cow::Borrowed(&[]),
    }
}

// Non-Fircatives
define_ipa_phoneme! {
    name: PH,
    id: "ph",
    ipa: "ɸ",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::NonSibilantFricative,
        place: ArticulationPlace::Bilabial,
        voicing: Voicing::Voiceless,
        modifiers: Cow::Borrowed(&[]),
    }
}
define_ipa_phoneme! {
    name: BH,
    id: "bh",
    ipa: "β",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::NonSibilantFricative,
        place: ArticulationPlace::Bilabial,
        voicing: Voicing::Voiced,
        modifiers: Cow::Borrowed(&[]),
    }
}
define_ipa_phoneme! {
    name: F,
    id: "f",
    ipa: "f",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::NonSibilantFricative,
        place: ArticulationPlace::Labiodental,
        voicing: Voicing::Voiceless,
        modifiers: Cow::Borrowed(&[]),
    }
}
define_ipa_phoneme! {
    name: V,
    id: "v",
    ipa: "v",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::NonSibilantFricative,
        place: ArticulationPlace::Labiodental,
        voicing: Voicing::Voiced,
        modifiers: Cow::Borrowed(&[]),
    }
}
define_ipa_phoneme! {
    name: TH_LINGUOLABIAL,
    id: "th_linguolabial",
    ipa: "θ̼",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::NonSibilantFricative,
        place: ArticulationPlace::Linguolabial,
        voicing: Voicing::Voiceless,
        modifiers: Cow::Borrowed(&[]),
    }
}
define_ipa_phoneme! {
    name: DH_LINGUOLABIAL,
    id: "dh_linguolabial",
    ipa: "ð̼",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::NonSibilantFricative,
        place: ArticulationPlace::Linguolabial,
        voicing: Voicing::Voiced,
        modifiers: Cow::Borrowed(&[]),
    }
}
define_ipa_phoneme! {
    name: TH,
    id: "th",
    ipa: "θ",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::NonSibilantFricative,
        place: ArticulationPlace::Dental,
        voicing: Voicing::Voiceless,
        modifiers: Cow::Borrowed(&[]),
    }
}
define_ipa_phoneme! {
    name: DH,
    id: "dh",
    ipa: "ð",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::NonSibilantFricative,
        place: ArticulationPlace::Dental,
        voicing: Voicing::Voiced,
        modifiers: Cow::Borrowed(&[]),
    }
}
define_ipa_phoneme! {
    name: TH_ALVEOLAR,
    id: "th_alveolar",
    ipa: "θ̠",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::NonSibilantFricative,
        place: ArticulationPlace::Alveolar,
        voicing: Voicing::Voiceless,
        modifiers: Cow::Borrowed(&[Modifier::Retracted]),
    }
}
define_ipa_phoneme! {
    name: DH_ALVEOLAR,
    id: "dh_alveolar",
    ipa: "ð̠",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::NonSibilantFricative,
        place: ArticulationPlace::Alveolar,
        voicing: Voicing::Voiced,
        modifiers: Cow::Borrowed(&[Modifier::Retracted]),
    }
}
define_ipa_phoneme! {
    name: TURNED_R_VOIVELESS_RETRACTED_RAISED,
    id: "turned_r_voiveless_retracted_raised",
    ipa: "ɹ̠̊˔",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::NonSibilantFricative,
        place: ArticulationPlace::Alveolar,
        voicing: Voicing::Voiceless,
        modifiers: Cow::Borrowed(&[Modifier::Retracted, Modifier::Raised]),
    }
}
define_ipa_phoneme! {
    name: TURNED_R_RETRACTED_RAISED,
    id: "turned_r_retracted_raised",
    ipa: "ɹ̠˔",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::NonSibilantFricative,
        place: ArticulationPlace::Alveolar,
        voicing: Voicing::Voiced,
        modifiers: Cow::Borrowed(&[Modifier::Retracted, Modifier::Raised]),
    }
}
define_ipa_phoneme! {
    name: TURNED_R_VOIVELESS_RETROFLEX_RAISED,
    id: "turned_r_voiveless_retroflex_raised",
    ipa: "ɻ̊˔",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::NonSibilantFricative,
        place: ArticulationPlace::Retroflex,
        voicing: Voicing::Voiceless,
        modifiers: Cow::Borrowed(&[Modifier::Raised]),
    }
}
define_ipa_phoneme! {
    name: TURNED_R_RETROFLEX_RAISED,
    id: "turned_r_retroflex_raised",
    ipa: "ɻ˔",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::NonSibilantFricative,
        place: ArticulationPlace::Retroflex,
        voicing: Voicing::Voiced,
        modifiers: Cow::Borrowed(&[Modifier::Raised]),
    }
}
define_ipa_phoneme! {
    name: F_PALATAL,
    id: "f_palatal",
    ipa: "ç",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::NonSibilantFricative,
        place: ArticulationPlace::Palatal,
        voicing: Voicing::Voiceless,
        modifiers: Cow::Borrowed(&[]),
    }
}
define_ipa_phoneme! {
    name: V_PALATAL,
    id: "v_palatal",
    ipa: "ʝ",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::NonSibilantFricative,
        place: ArticulationPlace::Palatal,
        voicing: Voicing::Voiced,
        modifiers: Cow::Borrowed(&[]),
    }
}
define_ipa_phoneme! {
    name: X,
    id: "x",
    ipa: "x",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::NonSibilantFricative,
        place: ArticulationPlace::Velar,
        voicing: Voicing::Voiceless,
        modifiers: Cow::Borrowed(&[]),
    }
}
define_ipa_phoneme! {
    name: RAMS_HORN,
    id: "gh_velar",
    ipa: "ɣ",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::NonSibilantFricative,
        place: ArticulationPlace::Velar,
        voicing: Voicing::Voiced,
        modifiers: Cow::Borrowed(&[]),
    }
}
define_ipa_phoneme! {
    name: X_UVULAR,
    id: "x_uvular",
    ipa: "χ",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::NonSibilantFricative,
        place: ArticulationPlace::Uvular,
        voicing: Voicing::Voiceless,
        modifiers: Cow::Borrowed(&[]),
    }
}
define_ipa_phoneme! {
    name: REVERSED_TRUNED_R,
    id: "reversed_truned_r",
    ipa: "ʁ",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::NonSibilantFricative,
        place: ArticulationPlace::Uvular,
        voicing: Voicing::Voiced,
        modifiers: Cow::Borrowed(&[]),
    }
}
define_ipa_phoneme! {
    name: H_BAR,
    id: "h_bar",
    ipa: "ħ",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::NonSibilantFricative,
        place: ArticulationPlace::Pharyngeal,
        voicing: Voicing::Voiceless,
        modifiers: Cow::Borrowed(&[]),
    }
}
define_ipa_phoneme! {
    name: GH_PHARYNGEAL,
    id: "gh_pharyngeal",
    ipa: "ʕ",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::NonSibilantFricative,
        place: ArticulationPlace::Pharyngeal,
        voicing: Voicing::Voiced,
        modifiers: Cow::Borrowed(&[]),
    }
}
define_ipa_phoneme! {
    name: H,
    id: "h",
    ipa: "h",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::NonSibilantFricative,
        place: ArticulationPlace::Glottal,
        voicing: Voicing::Voiceless,
        modifiers: Cow::Borrowed(&[]),
    }
}
define_ipa_phoneme! {
    name: H_VOICED,
    id: "h_voiced",
    ipa: "ɦ",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::NonSibilantFricative,
        place: ArticulationPlace::Glottal,
        voicing: Voicing::Voiced,
        modifiers: Cow::Borrowed(&[]),
    }
}

define_ipa_phoneme! {
    name: ROUNDED_V,
    id: "rounded_v",
    ipa: "ʋ",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::Approximant,
        place: ArticulationPlace::Labiodental,
        voicing: Voicing::Voiced,
        modifiers: Cow::Borrowed(&[]),
    }
}
define_ipa_phoneme! {
    name: TURNED_R,
    id: "turned_r",
    ipa: "ɹ",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::Approximant,
        place: ArticulationPlace::Alveolar,
        voicing: Voicing::Voiced,
        modifiers: Cow::Borrowed(&[]),
    }
}
define_ipa_phoneme! {
    name: TURNED_R_RETROFLEX,
    id: "turned_r_retroflex",
    ipa: "ɻ",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::Approximant,
        place: ArticulationPlace::Retroflex,
        voicing: Voicing::Voiced,
        modifiers: Cow::Borrowed(&[]),
    }
}
define_ipa_phoneme! {
    name: J,
    id: "j",
    ipa: "j",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::Approximant,
        place: ArticulationPlace::Palatal,
        voicing: Voicing::Voiced,
        modifiers: Cow::Borrowed(&[]),
    }
}
define_ipa_phoneme! {
    name: TURNED_M_VELAR,
    id: "turned_m_velar",
    ipa: "ɰ",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::Approximant,
        place: ArticulationPlace::Velar,
        voicing: Voicing::Voiced,
        modifiers: Cow::Borrowed(&[]),
    }
}
define_ipa_phoneme! {
    name: GLOTTAL_RAISED,
    id: "glottal_raised",
    ipa: "ʔ̞",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::Approximant,
        place: ArticulationPlace::Glottal,
        voicing: Voicing::Voiced,
        modifiers: Cow::Borrowed(&[Modifier::Lowered]),
    }
}

define_ipa_phoneme! {
    name: V_RIGHT_HOOK_ADVANCED,
    id: "v_right_hook_advanced",
    ipa: "ⱱ̟",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::TapFlap,
        place: ArticulationPlace::Bilabial,
        voicing: Voicing::Voiced,
        modifiers: Cow::Borrowed(&[Modifier::Advanced]),
    }
}
define_ipa_phoneme! {
    name: V_RIGHT_HOOK,
    id: "v_right_hook",
    ipa: "ⱱ",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::TapFlap,
        place: ArticulationPlace::Labiodental,
        voicing: Voicing::Voiced,
        modifiers: Cow::Borrowed(&[]),
    }
}
define_ipa_phoneme! {
    name: REVERSED_ROUNDED_R_LINGUOLABIAL,
    id: "reversed_rounded_r_linguolabial",
    ipa: "ɾ̼",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::TapFlap,
        place: ArticulationPlace::Linguolabial,
        voicing: Voicing::Voiced,
        modifiers: Cow::Borrowed(&[]),
    }
}
define_ipa_phoneme! {
    name: REVERSED_ROUNDED_R_,
    id: "reversed_rounded_r_",
    ipa: "ɾ",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::TapFlap,
        place: ArticulationPlace::Alveolar,
        voicing: Voicing::Voiced,
        modifiers: Cow::Borrowed(&[]),
    }
}
define_ipa_phoneme! {
    name: REVERSED_ROUNDED_R_VOICELESS,
    id: "reversed_rounded_r_voiceless",
    ipa: "ɾ̥",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::TapFlap,
        place: ArticulationPlace::Alveolar,
        voicing: Voicing::Voiceless,
        modifiers: Cow::Borrowed(&[]),
    }
}

// Continue at TAP/RETROFLEX => https://en.wikipedia.org/wiki/International_Phonetic_Alphabet
// Remaining base symbols and the chart's tie-bar examples.
define_ipa_phoneme! {
    name: R_BILABIAL_TRILL,
    id: "r_bilabial_trill",
    ipa: "ʙ",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::Trill,
        place: ArticulationPlace::Bilabial,
        voicing: Voicing::Voiced,
        modifiers: Cow::Borrowed(&[]),
    }
}

define_ipa_phoneme! {
    name: R_ALVEOLAR_TRILL,
    id: "r_alveolar_trill",
    ipa: "r",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::Trill,
        place: ArticulationPlace::Alveolar,
        voicing: Voicing::Voiced,
        modifiers: Cow::Borrowed(&[]),
    }
}

define_ipa_phoneme! {
    name: R_UVULAR_TRILL,
    id: "r_uvular_trill",
    ipa: "ʀ",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::Trill,
        place: ArticulationPlace::Uvular,
        voicing: Voicing::Voiced,
        modifiers: Cow::Borrowed(&[]),
    }
}

define_ipa_phoneme! {
    name: R_RETROFLEX_TAP,
    id: "r_retroflex_tap",
    ipa: "ɽ",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::TapFlap,
        place: ArticulationPlace::Retroflex,
        voicing: Voicing::Voiced,
        modifiers: Cow::Borrowed(&[]),
    }
}

define_ipa_phoneme! {
    name: L_ALVEOLAR_FRICATIVE_VOICELESS,
    id: "l_alveolar_fricative_voiceless",
    ipa: "ɬ",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::LateralFricative,
        place: ArticulationPlace::Alveolar,
        voicing: Voicing::Voiceless,
        modifiers: Cow::Borrowed(&[]),
    }
}

define_ipa_phoneme! {
    name: L_ALVEOLAR_FRICATIVE_VOICED,
    id: "l_alveolar_fricative_voiced",
    ipa: "ɮ",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::LateralFricative,
        place: ArticulationPlace::Alveolar,
        voicing: Voicing::Voiced,
        modifiers: Cow::Borrowed(&[]),
    }
}

define_ipa_phoneme! {
    name: L_ALVEOLAR,
    id: "l_alveolar",
    ipa: "l",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::LateralApproximant,
        place: ArticulationPlace::Alveolar,
        voicing: Voicing::Voiced,
        modifiers: Cow::Borrowed(&[]),
    }
}

define_ipa_phoneme! {
    name: L_RETROFLEX,
    id: "l_retroflex",
    ipa: "ɭ",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::LateralApproximant,
        place: ArticulationPlace::Retroflex,
        voicing: Voicing::Voiced,
        modifiers: Cow::Borrowed(&[]),
    }
}

define_ipa_phoneme! {
    name: L_PALATAL,
    id: "l_palatal",
    ipa: "ʎ",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::LateralApproximant,
        place: ArticulationPlace::Palatal,
        voicing: Voicing::Voiced,
        modifiers: Cow::Borrowed(&[]),
    }
}

define_ipa_phoneme! {
    name: L_VELAR,
    id: "l_velar",
    ipa: "ʟ",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::LateralApproximant,
        place: ArticulationPlace::Velar,
        voicing: Voicing::Voiced,
        modifiers: Cow::Borrowed(&[]),
    }
}

define_ipa_phoneme! {
    name: W_LABIAL_VELAR_VOICELESS,
    id: "w_labial_velar_voiceless",
    ipa: "ʍ",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::NonSibilantFricative,
        place: ArticulationPlace::LabialVelar,
        voicing: Voicing::Voiceless,
        modifiers: Cow::Borrowed(&[]),
    }
}

define_ipa_phoneme! {
    name: W_LABIAL_VELAR,
    id: "w_labial_velar",
    ipa: "w",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::Approximant,
        place: ArticulationPlace::LabialVelar,
        voicing: Voicing::Voiced,
        modifiers: Cow::Borrowed(&[]),
    }
}

define_ipa_phoneme! {
    name: H_LABIAL_PALATAL,
    id: "h_labial_palatal",
    ipa: "ɥ",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::Approximant,
        place: ArticulationPlace::LabialPalatal,
        voicing: Voicing::Voiced,
        modifiers: Cow::Borrowed(&[]),
    }
}

define_ipa_phoneme! {
    name: H_EPIGLOTTAL_VOICELESS,
    id: "h_epiglottal_voiceless",
    ipa: "ʜ",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::NonSibilantFricative,
        place: ArticulationPlace::Epiglottal,
        voicing: Voicing::Voiceless,
        modifiers: Cow::Borrowed(&[]),
    }
}

define_ipa_phoneme! {
    name: H_EPIGLOTTAL_VOICED,
    id: "h_epiglottal_voiced",
    ipa: "ʢ",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::NonSibilantFricative,
        place: ArticulationPlace::Epiglottal,
        voicing: Voicing::Voiced,
        modifiers: Cow::Borrowed(&[]),
    }
}

define_ipa_phoneme! {
    name: L_ALVEOLAR_FLAP,
    id: "l_alveolar_flap",
    ipa: "ɺ",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::LateralTapFlap,
        place: ArticulationPlace::Alveolar,
        voicing: Voicing::Voiced,
        modifiers: Cow::Borrowed(&[]),
    }
}

define_ipa_phoneme! {
    name: SH_X_SIMULTANEOUS,
    id: "sh_x_simultaneous",
    ipa: "ɧ",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::Fricative,
        place: ArticulationPlace::PostalveolarVelar,
        voicing: Voicing::Voiceless,
        modifiers: Cow::Borrowed(&[]),
    }
}

define_ipa_phoneme! {
    name: T_S_AFFRICATE,
    id: "t_s_affricate",
    ipa: "t͡s",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::Affricate,
        release: ArticulationPlace::Alveolar,
        place: ArticulationPlace::Alveolar,
        voicing: Voicing::Voiceless,
        modifiers: Cow::Borrowed(&[]),
    }
}

define_ipa_phoneme! {
    name: D_ZH_AFFRICATE,
    id: "d_zh_affricate",
    ipa: "d͡ʒ",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::Affricate,
        release: ArticulationPlace::Postalveolar,
        place: ArticulationPlace::Alveolar,
        voicing: Voicing::Voiced,
        modifiers: Cow::Borrowed(&[]),
    }
}

define_ipa_phoneme! {
    name: KP_DOUBLE_PLOSIVE,
    id: "kp_double_plosive",
    ipa: "k͡p",
    consonantal: {
        airflow: AirflowMechanism::Pulmonic,
        manner: Manner::Plosive,
        place: ArticulationPlace::LabialVelar,
        voicing: Voicing::Voiceless,
        modifiers: Cow::Borrowed(&[]),
    }
}

pub const ALL_PULMONICS: &[&crate::phonology::SoundDefinition] = &[
    &M_BILABIAL,
    &M_BILABIAL_VOICELESS,
    &M_LABIODENTAL,
    &M_LABIODENTAL_VOICELESS,
    &N_LINGUOLABIAL,
    &N_ALVEOLAR,
    &N_ALVEOLAR_VOICELESS,
    &N_RETROFLEX,
    &N_RETROFLEX_VOICELESS,
    &N_PALATAL,
    &N_PALATAL_VOICELESS,
    &N_VELAR,
    &N_VELAR_VOICELESS,
    &N_UVULAR,
    &N_UVULAR_VOICELESS,
    &P_BILABIAL,
    &B_BILABIAL,
    &P_LABIODENTAL,
    &B_LABIODENTAL,
    &T_LINGUOLABIAL,
    &D_LINGUOLABIAL,
    &T_ALVEOLAR,
    &D_ALVEOLAR,
    &T_RETROFLEX,
    &D_RETROFLEX,
    &C_PALATAL_PLOSIVE,
    &J_PALATAL_PLOSIVE,
    &K_VELAR,
    &G_VELAR,
    &Q_UVULAR,
    &G_UVULAR,
    &STOP_EPIGLOTTAL,
    &STOP_GLOTTAL,
    &S_ALVEOLAR,
    &Z_ALVEOLAR,
    &S_POSTALVEOLAR,
    &Z_POSTALVEOLAR,
    &S_RETROFLEX,
    &Z_RETROFLEX,
    &S_PALATAL,
    &Z_PALATAL,
    &PH,
    &BH,
    &F,
    &V,
    &TH_LINGUOLABIAL,
    &DH_LINGUOLABIAL,
    &TH,
    &DH,
    &TH_ALVEOLAR,
    &DH_ALVEOLAR,
    &TURNED_R_VOIVELESS_RETRACTED_RAISED,
    &TURNED_R_RETRACTED_RAISED,
    &TURNED_R_VOIVELESS_RETROFLEX_RAISED,
    &TURNED_R_RETROFLEX_RAISED,
    &F_PALATAL,
    &V_PALATAL,
    &X,
    &RAMS_HORN,
    &X_UVULAR,
    &REVERSED_TRUNED_R,
    &H_BAR,
    &GH_PHARYNGEAL,
    &H,
    &H_VOICED,
    &ROUNDED_V,
    &TURNED_R,
    &TURNED_R_RETROFLEX,
    &J,
    &TURNED_M_VELAR,
    &GLOTTAL_RAISED,
    &V_RIGHT_HOOK_ADVANCED,
    &V_RIGHT_HOOK,
    &REVERSED_ROUNDED_R_LINGUOLABIAL,
    &REVERSED_ROUNDED_R_,
    &REVERSED_ROUNDED_R_VOICELESS,
    &R_BILABIAL_TRILL,
    &R_ALVEOLAR_TRILL,
    &R_UVULAR_TRILL,
    &R_RETROFLEX_TAP,
    &L_ALVEOLAR_FRICATIVE_VOICELESS,
    &L_ALVEOLAR_FRICATIVE_VOICED,
    &L_ALVEOLAR,
    &L_RETROFLEX,
    &L_PALATAL,
    &L_VELAR,
    &W_LABIAL_VELAR_VOICELESS,
    &W_LABIAL_VELAR,
    &H_LABIAL_PALATAL,
    &H_EPIGLOTTAL_VOICELESS,
    &H_EPIGLOTTAL_VOICED,
    &L_ALVEOLAR_FLAP,
    &SH_X_SIMULTANEOUS,
    &T_S_AFFRICATE,
    &D_ZH_AFFRICATE,
    &KP_DOUBLE_PLOSIVE,
];
