use super::speaker::*;
use crate::phonology::{ArticulationPlace, Manner, TongueHeight, TonguePosition};

impl SpeakerAnatomy {
    pub fn new() -> Self {
        Self::human()
    }
    pub fn human() -> Self {
        Self {
            oral_cavity: OralCavity {
                teeth: TeethConfiguration::Human,
                dental_contact: ContactCapability::Available,
                labiodental_contact: ContactCapability::Available,
                has_alveolar_ridge: true,
                has_hard_palate: true,
                has_soft_palate: true,
                has_uvula: true,
                has_epiglottis: true,
            },
            lips: LipControl::Flexible,
            tongue: TongueControl {
                tip: TonguePartControl::Agile,
                blade: TonguePartControl::Agile,
                body: TonguePartControl::Agile,
                root: TonguePartControl::Agile,
                can_curl_for_retroflex: true,
                can_perform_lateral_release: true,
                vowel_heights: vec![
                    TongueHeight::Close,
                    TongueHeight::NearClose,
                    TongueHeight::CloseMid,
                    TongueHeight::Mid,
                    TongueHeight::OpenMid,
                    TongueHeight::NearOpen,
                    TongueHeight::Open,
                ],
                vowel_positions: vec![
                    TonguePosition::Front,
                    TonguePosition::NearFront,
                    TonguePosition::Central,
                    TonguePosition::NearBack,
                    TonguePosition::Back,
                ],
            },
            larynx: LarynxControl {
                voicing: VoicingCapabilities {
                    voiced: true,
                    voiceless: true,
                },
                can_aspirate: true,
                can_produce_breathy_voice: true,
                can_produce_creaky_voice: true,
                can_close_glottis: true,
                can_adjust_constriction: true,
                can_produce_ejectives: true,
                can_produce_implosives: true,
            },
            airflow: AirflowControl {
                pulmonic: PulmonicControl::Advanced,
                velic_port: VelicPortControl::Controllable,
                can_produce_clicks: true,
            },
            prosody: crate::prosody::ProsodicCapabilities::human(),
            articulation: ArticulatoryCapabilities {
                trills: vec![
                    ArticulationPlace::Bilabial,
                    ArticulationPlace::Alveolar,
                    ArticulationPlace::Uvular,
                ],
                taps: vec![
                    ArticulationPlace::Bilabial,
                    ArticulationPlace::Labiodental,
                    ArticulationPlace::Linguolabial,
                    ArticulationPlace::Dental,
                    ArticulationPlace::Alveolar,
                    ArticulationPlace::Postalveolar,
                    ArticulationPlace::Retroflex,
                ],
                can_coordinate_affricates: true,
                can_double_articulate: true,
                can_articulate_epiglottals: true,
                can_fine_adjust: true,
                can_rhoticize: true,
                can_control_release: true,
                syllabic_manners: vec![
                    Manner::Nasal,
                    Manner::Trill,
                    Manner::TapFlap,
                    Manner::Approximant,
                    Manner::SibilantFricative,
                    Manner::NonSibilantFricative,
                    Manner::Fricative,
                    Manner::LateralApproximant,
                    Manner::LateralFricative,
                ],
            },
        }
    }
    pub fn teeth(mut self, teeth: TeethConfiguration) -> Self {
        self.oral_cavity.teeth = teeth;
        let contact = match teeth {
            TeethConfiguration::None => ContactCapability::Unavailable,
            TeethConfiguration::Fangs => ContactCapability::Unspecified,
            TeethConfiguration::Flat | TeethConfiguration::Human => ContactCapability::Available,
        };
        self.oral_cavity.dental_contact = contact;
        self.oral_cavity.labiodental_contact = contact;
        self
    }
    pub fn dental_contact(mut self, contact: ContactCapability) -> Self {
        self.oral_cavity.dental_contact = contact;
        self
    }
    pub fn labiodental_contact(mut self, contact: ContactCapability) -> Self {
        self.oral_cavity.labiodental_contact = contact;
        self
    }
    pub fn lips(mut self, lips: LipControl) -> Self {
        self.lips = lips;
        self
    }
    pub fn tongue(mut self, tongue: TongueControl) -> Self {
        self.tongue = tongue;
        self
    }
    pub fn larynx(mut self, larynx: LarynxControl) -> Self {
        self.larynx = larynx;
        self
    }
    pub fn with_airflow(mut self, airflow: AirflowControl) -> Self {
        self.airflow = airflow;
        self
    }
}
impl Default for SpeakerAnatomy {
    fn default() -> Self {
        Self::human()
    }
}
