//! A deliberately bounded articulatory model, not a physiological simulation.
use crate::phonology::{ArticulationPlace, Manner, TongueHeight, TonguePosition};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpeakerAnatomy {
    pub oral_cavity: OralCavity,
    pub lips: LipControl,
    pub tongue: TongueControl,
    pub larynx: LarynxControl,
    pub airflow: AirflowControl,
    pub prosody: crate::prosody::ProsodicCapabilities,
    pub articulation: ArticulatoryCapabilities,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OralCavity {
    pub teeth: TeethConfiguration,
    /// Shape alone does not establish whether the required contacts are usable.
    pub dental_contact: ContactCapability,
    pub labiodental_contact: ContactCapability,
    pub has_alveolar_ridge: bool,
    pub has_hard_palate: bool,
    pub has_soft_palate: bool,
    pub has_uvula: bool,
    pub has_epiglottis: bool,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContactCapability {
    Available,
    Unavailable,
    Unspecified,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArticulatoryCapabilities {
    pub trills: Vec<ArticulationPlace>,
    pub taps: Vec<ArticulationPlace>,
    pub can_coordinate_affricates: bool,
    pub can_double_articulate: bool,
    pub can_articulate_epiglottals: bool,
    pub can_fine_adjust: bool,
    pub can_rhoticize: bool,
    pub can_control_release: bool,
    pub syllabic_manners: Vec<Manner>,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LipControl {
    None,
    Rigid,
    Flexible,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TongueControl {
    pub tip: TonguePartControl,
    pub blade: TonguePartControl,
    pub body: TonguePartControl,
    pub root: TonguePartControl,
    pub can_curl_for_retroflex: bool,
    pub can_perform_lateral_release: bool,
    /// Explicit ranges avoid guessing vowel capabilities from a single agility grade.
    pub vowel_heights: Vec<TongueHeight>,
    pub vowel_positions: Vec<TonguePosition>,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TonguePartControl {
    None,
    Limited,
    Agile,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TeethConfiguration {
    None,
    Fangs,
    Flat,
    Human,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct VoicingCapabilities {
    pub voiced: bool,
    pub voiceless: bool,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LarynxControl {
    pub voicing: VoicingCapabilities,
    pub can_aspirate: bool,
    pub can_produce_breathy_voice: bool,
    pub can_produce_creaky_voice: bool,
    pub can_close_glottis: bool,
    pub can_adjust_constriction: bool,
    pub can_produce_ejectives: bool,
    pub can_produce_implosives: bool,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AirflowControl {
    pub pulmonic: PulmonicControl,
    pub velic_port: VelicPortControl,
    pub can_produce_clicks: bool,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PulmonicControl {
    None,
    Basic,
    Advanced,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VelicPortControl {
    OralOnly,
    NasalOnly,
    Controllable,
}
