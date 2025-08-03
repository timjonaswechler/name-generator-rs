use crate::anatomy::speaker::{
    AirflowControl, LarynxControl, LipControl, OralCavity, PulmonicControl, SpeakerAnatomy,
    TeethConfiguration, TongueControl, TonguePartControl, VelicPortControl, VoicingControl,
};
use crate::validation::ValidationErrors;

impl SpeakerAnatomy {
    /// Erstellt einen neuen Builder mit Standardwerten für einen menschlichen Sprecher.
    pub fn new() -> Self {
        Self {
            oral_cavity: OralCavity {
                teeth: TeethConfiguration::Human,
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
            },
            larynx: LarynxControl {
                voicing: VoicingControl::Advanced,
                can_produce_ejectives: true,
            },
            airflow: AirflowControl {
                pulmonic: PulmonicControl::Advanced,
                velic_port: VelicPortControl::Controllable,
                can_produce_clicks: true,
            },
        }
    }
    /// Fügt die Konfiguration der Mundhöhle hinzu.
    pub fn oral_cavity(mut self, oral_cavity: OralCavity) -> Self {
        self.oral_cavity = oral_cavity;
        self
    }
    /// Fügt die Konfiguration der Zähne hinzu.
    pub fn teeth(mut self, teeth: TeethConfiguration) -> Self {
        self.oral_cavity.teeth = teeth;
        self
    }
    pub fn oral_cavity_has_alveolar_ridge(mut self, has_alveolar_ridge: bool) -> Self {
        self.oral_cavity.has_alveolar_ridge = has_alveolar_ridge;
        self
    }
    pub fn oral_cavity_has_hard_palate(mut self, has_hard_palate: bool) -> Self {
        self.oral_cavity.has_hard_palate = has_hard_palate;
        self
    }
    pub fn oral_cavity_has_soft_palate(mut self, has_soft_palate: bool) -> Self {
        self.oral_cavity.has_soft_palate = has_soft_palate;
        self
    }
    pub fn oral_cavity_has_uvula(mut self, has_uvula: bool) -> Self {
        self.oral_cavity.has_uvula = has_uvula;
        self
    }
    pub fn oral_cavity_has_epiglottis(mut self, has_epiglottis: bool) -> Self {
        self.oral_cavity.has_epiglottis = has_epiglottis;
        self
    }

    /// Fügt die Lippenkontrolle hinzu.
    pub fn lips(mut self, lips: LipControl) -> Self {
        self.lips = lips;
        self
    }

    /// Fügt die Zungenkontrolle hinzu.
    pub fn tongue(mut self, tongue: TongueControl) -> Self {
        self.tongue = tongue;
        self
    }
    pub fn tongue_tip(mut self, tip: TonguePartControl) -> Self {
        self.tongue.tip = tip;
        self
    }
    pub fn tongue_blade(mut self, blade: TonguePartControl) -> Self {
        self.tongue.blade = blade;
        self
    }
    pub fn tongue_body(mut self, body: TonguePartControl) -> Self {
        self.tongue.body = body;
        self
    }
    pub fn tongue_root(mut self, root: TonguePartControl) -> Self {
        self.tongue.root = root;
        self
    }
    pub fn tongue_can_curl_for_retroflex(mut self, can_curl: bool) -> Self {
        self.tongue.can_curl_for_retroflex = can_curl;
        self
    }
    pub fn tongue_can_perform_lateral_release(mut self, can_release: bool) -> Self {
        self.tongue.can_perform_lateral_release = can_release;
        self
    }

    /// Fügt die Kehlkopfsteuerung hinzu.
    pub fn larynx(mut self, larynx: LarynxControl) -> Self {
        self.larynx = larynx;
        self
    }
    pub fn larynx_voicing(mut self, voicing: VoicingControl) -> Self {
        self.larynx.voicing = voicing;
        self
    }
    pub fn larynx_can_produce_ejectives(mut self, can_produce: bool) -> Self {
        self.larynx.can_produce_ejectives = can_produce;
        self
    }

    /// Fügt die Luftstromkontrolle hinzu.
    pub fn with_airflow(mut self, airflow: AirflowControl) -> Self {
        self.airflow = airflow;
        self
    }
    pub fn airflow_pulmonic(mut self, pulmonic: PulmonicControl) -> Self {
        self.airflow.pulmonic = pulmonic;
        self
    }
    pub fn airflow_velic_port(mut self, velic_port: VelicPortControl) -> Self {
        self.airflow.velic_port = velic_port;
        self
    }
    pub fn airflow_can_produce_clicks(mut self, can_produce: bool) -> Self {
        self.airflow.can_produce_clicks = can_produce;
        self
    }

    /// Baut das vollständige Anatomie-Modell des Sprechers.
    /// Führt anatomische Validierung durch und gibt einen Fehler zurück,
    /// wenn die Konfiguration anatomisch unmöglich ist.
    pub fn build(self) -> Result<SpeakerAnatomy, ValidationErrors> {
        let mut errors = ValidationErrors::new();

        if let Err(e) = self.validate_anatomical_consistency() {
            errors.merge(e);
        }

        if errors.is_empty() {
            Ok(SpeakerAnatomy {
                oral_cavity: self.oral_cavity,
                lips: self.lips,
                tongue: self.tongue,
                larynx: self.larynx,
                airflow: self.airflow,
            })
        } else {
            Err(errors)
        }
    }
    pub fn human() -> Self {
        Self {
            oral_cavity: OralCavity {
                teeth: TeethConfiguration::Human,
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
            },
            larynx: LarynxControl {
                voicing: VoicingControl::Advanced,
                can_produce_ejectives: true,
            },
            airflow: AirflowControl {
                pulmonic: PulmonicControl::Advanced,
                velic_port: VelicPortControl::Controllable,
                can_produce_clicks: true,
            },
        }
    }
}

impl Default for SpeakerAnatomy {
    fn default() -> Self {
        Self::human()
    }
}
