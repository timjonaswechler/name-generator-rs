//! Anatomical validation errors
//!
//! This module defines errors that occur when anatomical configurations
//! are inconsistent or impossible.
//!
//! Following the validation_architecture.md approach: domain-specific validation
//! functions that return ValidationError directly.

use crate::anatomy::speaker::{
    LipControl, PulmonicControl, SpeakerAnatomy, TeethConfiguration, TonguePartControl,
    VelicPortControl, VoicingControl,
};
use crate::validation::errors::{ValidationError, ValidationErrors};
impl SpeakerAnatomy {
    /// Validates the anatomical consistency of a SpeakerAnatomy configuration
    ///
    /// This function checks all inter-dependencies between anatomical features
    /// to ensure the configuration is internally consistent and realistic.
    pub fn validate_anatomical_consistency(&self) -> Result<(), ValidationErrors> {
        let mut errors = ValidationErrors::new();

        // 1. Mundhöhlen-Abhängigkeiten (OralCavity)

        // 1.1 Velic Port Control Abhängigkeiten
        // VelicPortControl::Controllable erfordert has_soft_palate = true
        if matches!(self.airflow.velic_port, VelicPortControl::Controllable)
            && !self.oral_cavity.has_soft_palate
        {
            errors.add(
                "velic_port_requires_soft_palate",
                ValidationError::new("velic_port_requires_soft_palate")
                    .add_param("current_velic_port", "Controllable")
                    .add_param("current_soft_palate", "false")
                    .add_param("required_soft_palate", "true")
                    .with_message("Kontrollierbare Nasal-/Oralluft-Umschaltung erfordert einen weichen Gaumen (velum)"),
            );
        }

        // 1.2 Uvulare Artikulation Abhängigkeiten
        // has_uvula = true erfordert has_soft_palate = true
        if self.oral_cavity.has_uvula && !self.oral_cavity.has_soft_palate {
            errors.add(
                "uvula_requires_soft_palate",
                ValidationError::new("uvula_requires_soft_palate")
                    .add_param("current_uvula", "true")
                    .add_param("current_soft_palate", "false")
                    .add_param("required_soft_palate", "true")
                    .with_message("Ein Zäpfchen (uvula) kann anatomisch nur existieren wenn ein weicher Gaumen vorhanden ist"),
            );
        }

        // 1.3 Hierarchische Gaumen-Struktur
        // has_soft_palate = true sollte has_hard_palate = true implizieren
        if self.oral_cavity.has_soft_palate && !self.oral_cavity.has_hard_palate {
            errors.add(
                "soft_palate_requires_hard_palate",
                ValidationError::new("soft_palate_requires_hard_palate")
                    .add_param("current_soft_palate", "true")
                    .add_param("current_hard_palate", "false")
                    .add_param("required_hard_palate", "true")
                    .with_message("Ein weicher Gaumen erfordert normalerweise auch einen harten Gaumen als anatomische Grundlage"),
            );
        }

        // 2. Zungen-Kontrolle Abhängigkeiten (TongueControl)

        // 2.1 Retroflex Fähigkeiten
        // can_curl_for_retroflex = true erfordert tip = Agile
        if self.tongue.can_curl_for_retroflex
            && !matches!(self.tongue.tip, TonguePartControl::Agile)
        {
            errors.add(
                "retroflex_requires_agile_tongue_tip",
                ValidationError::new("anatomical_configuration_error")
                    .add_param("reason", "retroflex_requires_agile_tongue_tip")
                    .add_param("required_configuration", "tongue.tip = Agile")
                    .add_param("current_configuration", "tongue.tip != Agile")
                    .with_message("Retroflex capability requires agile tongue tip control"),
            );
        }

        // 2.2 Laterale Freisetzung
        // can_perform_lateral_release = true erfordert mindestens tip oder blade != None
        if self.tongue.can_perform_lateral_release
            && matches!(self.tongue.tip, TonguePartControl::None)
            && matches!(self.tongue.blade, TonguePartControl::None)
        {
            errors.add(
                "lateral_release_requires_tongue_control",
                ValidationError::new("anatomical_configuration_error")
                    .add_param("reason", "lateral_release_requires_tongue_control")
                    .add_param(
                        "required_configuration",
                        "tongue.blade != None OR tongue.tip != None",
                    )
                    .with_message(
                        "Lateral release requires functional tongue blade or tip control",
                    ),
            );
        }

        // 2.3 Koronale Artikulation Grundlagen
        // Für coronale Laute (dental, alveolar, postalveolar) wird mindestens tip oder blade benötigt
        if matches!(self.tongue.tip, TonguePartControl::None)
            && matches!(self.tongue.blade, TonguePartControl::None)
            && self.oral_cavity.has_alveolar_ridge
        {
            errors.add(
                "alveolar_ridge_unusable_without_tongue_front",
                ValidationError::new("alveolar_ridge_unusable_without_tongue_front")
                    .add_param("current_tongue_tip", "None")
                    .add_param("current_tongue_blade", "None")
                    .add_param("current_alveolar_ridge", "true")
                    .add_param("required_tongue_control", "tip != None OR blade != None")
                    .with_message("Ein Alveolarfortsatz ist ohne Zungenspitzen- oder Zungenblattkontrolle nicht nutzbar"),
            );
        }

        // 3. Zahn-Konfiguration Abhängigkeiten (TeethConfiguration)

        // 3.1 Labio-dentale Artikulation
        // LipControl::Flexible + TeethConfiguration::None = Problem für labiodentale Laute
        if matches!(self.lips, LipControl::Flexible)
            && matches!(self.oral_cavity.teeth, TeethConfiguration::None)
        {
            // Warning, nicht Error - könnte trotzdem funktionieren
            errors.add(
                "flexible_lips_without_teeth_limits_sounds",
                ValidationError::new("flexible_lips_without_teeth_limits_sounds")
                    .add_param("current_lips", "Flexible")
                    .add_param("current_teeth", "None")
                    .add_param("affected_sounds", "[f], [v]")
                    .with_message("Flexible Lippen ohne Zähne schränken labiodentale Laute wie [f] und [v] ein"),
            );
        }

        // 3.2 Interdentale Artikulation
        // Fangs könnten interdentale Laute stören
        if matches!(self.oral_cavity.teeth, TeethConfiguration::Fangs)
            && matches!(self.tongue.tip, TonguePartControl::Agile)
        {
            errors.add(
                "fangs_may_interfere_with_interdentals",
                ValidationError::new("fangs_may_interfere_with_interdentals")
                    .add_param("current_teeth", "Fangs")
                    .add_param("current_tongue_tip", "Agile")
                    .add_param("affected_sounds", "[θ], [ð]")
                    .with_message("Reißzähne können die Produktion interdentaler Laute wie [θ] und [ð] beeinträchtigen"),
            );
        }

        // 4. Kehlkopf-Kontrolle Abhängigkeiten (LarynxControl)

        // 4.1 Ejektive Produktion
        // can_produce_ejectives = true mit VoicingControl::None ist widersprüchlich
        if self.larynx.can_produce_ejectives && matches!(self.larynx.voicing, VoicingControl::None)
        {
            errors.add(
                "ejectives_require_larynx_control",
                ValidationError::new("ejectives_require_larynx_control")
                    .add_param("current_ejectives", "true")
                    .add_param("current_voicing", "None")
                    .add_param("required_voicing", "Basic OR Advanced")
                    .with_message(
                        "Ejektive Laute erfordern aktive Kehlkopfkontrolle für Druckaufbau",
                    ),
            );
        }

        // 5. Luftstrom-Kontrolle Abhängigkeiten (AirflowControl)

        // 5.1 Klick-Laute Komplexität
        // can_produce_clicks = true erfordert sowohl vordere als auch hintere Zungenkontrolle
        if self.airflow.can_produce_clicks
            && (matches!(self.tongue.tip, TonguePartControl::None)
                || matches!(self.tongue.blade, TonguePartControl::None)
                || matches!(self.tongue.body, TonguePartControl::None))
        {
            errors.add(
                "clicks_require_complex_tongue_control",
                ValidationError::new("anatomical_configuration_error")
                    .add_param("reason", "clicks_require_agile_tongue_control")
                    .add_param(
                        "required_configuration",
                        "tongue.tip = Agile AND tongue.body = Agile",
                    )
                    .with_message(
                        "Click sound production requires agile tongue tip and body control",
                    ),
            );
        }

        // 5.2 Pulmonische vs. Alternative Luftströme
        // can_produce_clicks = true mit PulmonicControl::None könnte problematisch sein
        if self.airflow.can_produce_clicks && matches!(self.airflow.pulmonic, PulmonicControl::None)
        {
            errors.add(
                "clicks_without_pulmonic_limits_combinations",
                ValidationError::new("clicks_without_pulmonic_limits_combinations")
                    .add_param("current_clicks", "true")
                    .add_param("current_pulmonic", "None")
                    .add_param("limitation", "combined_articulations")
                    .with_message("Klicklaute ohne pulmonischen Luftstrom begrenzen kombinierte Artikulationen"),
            );
        }

        // 6. Übergreifende Anatomische Konsistenz

        // 6.1 Lippen-Zähne Koordination
        // LipControl::None aber TeethConfiguration::Human ist ungewöhnlich
        if matches!(self.lips, LipControl::None)
            && matches!(self.oral_cavity.teeth, TeethConfiguration::Human)
        {
            errors.add(
                "human_teeth_without_lips_unusual",
                ValidationError::new("human_teeth_without_lips_unusual")
                    .add_param("current_lips", "None")
                    .add_param("current_teeth", "Human")
                    .add_param("anatomical_consistency", "unusual")
                    .with_message(
                        "Menschliche Zahnkonfiguration ohne Lippen ist anatomisch ungewöhnlich",
                    ),
            );
        }

        // 6.2 Vollständiger Artikulator-Verlust
        // Keine Lippen UND keine Zungenkontrolle = sehr begrenzte Artikulation
        if matches!(self.lips, LipControl::None)
            && matches!(self.tongue.tip, TonguePartControl::None)
            && matches!(self.tongue.blade, TonguePartControl::None)
            && matches!(self.tongue.body, TonguePartControl::None)
        {
            errors.add(
                "insufficient_articulators",
                ValidationError::new("insufficient_articulators")
                    .add_param("current_lips", "None")
                    .add_param("current_tongue_tip", "None")
                    .add_param("current_tongue_blade", "None")
                    .add_param("current_tongue_body", "None")
                    .add_param("consequence", "no_consonant_production")
                    .with_message(
                        "Ohne Lippen oder Zungenkontrolle sind praktisch keine Konsonanten möglich",
                    ),
            );
        }

        // 6.3 Strukturelle aber nicht funktionale Anatomie
        // has_alveolar_ridge = true aber keine Zungenkontrolle
        if self.oral_cavity.has_alveolar_ridge
            && matches!(self.tongue.tip, TonguePartControl::None)
            && matches!(self.tongue.blade, TonguePartControl::None)
        {
            errors.add(
                "alveolar_ridge_functionally_unused",
                ValidationError::new("alveolar_ridge_functionally_unused")
                    .add_param("current_alveolar_ridge", "true")
                    .add_param("current_tongue_tip", "None")
                    .add_param("current_tongue_blade", "None")
                    .add_param("functional_status", "unused")
                    .with_message("Alveolarfortsatz vorhanden aber ohne Zungenkontrolle funktional nicht nutzbar"),
            );
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    // ===== ANATOMICAL PHONEME VALIDATION (based on validation.txt Decision Tree) =====
    /// Creates error for anatomically impossible phonemes
    pub fn phoneme_anatomically_impossible(&self, phoneme: &str, reason: &str) -> ValidationError {
        ValidationError::new("phoneme_anatomically_impossible")
            .add_param("phoneme", phoneme)
            .add_param("anatomical_reason", reason)
            .add_param("speaker_anatomy", "incompatible")
            .with_message(format!(
                "Phonem '{}' ist anatomisch unmöglich für diesen Sprecher: {}",
                phoneme, reason
            ))
    }

    /// Validates airflow mechanism requirements (validation.txt lines 2-14)
    pub fn validate_airflow_mechanism(
        &self,
        phoneme_ipa: &str,
        airflow_type: &str,
    ) -> Result<(), ValidationError> {
        use crate::anatomy::speaker::{PulmonicControl, TonguePartControl};
        match airflow_type {
            "pulmonic" => {
                if matches!(self.airflow.pulmonic, PulmonicControl::None) {
                    return Err(self.phoneme_anatomically_impossible(
                        phoneme_ipa,
                        "Pulmonische Laute erfordern airflow.pulmonic ≠ None",
                    ));
                }
            }
            "click" => {
                if !self.airflow.can_produce_clicks {
                    return Err(self.phoneme_anatomically_impossible(
                        phoneme_ipa,
                        "Klicklaute erfordern airflow.can_produce_clicks = true",
                    ));
                }
                // Additional check from validation.txt line 9: tongue.tip + tongue.body = Agile
                if !matches!(self.tongue.tip, TonguePartControl::Agile)
                    || !matches!(self.tongue.body, TonguePartControl::Agile)
                {
                    return Err(self.phoneme_anatomically_impossible(
                        phoneme_ipa,
                        "Klicklaute erfordern agile Zungenspitze UND Zungenkörper",
                    ));
                }
            }
            "ejective" => {
                if !self.larynx.can_produce_ejectives {
                    return Err(self.phoneme_anatomically_impossible(
                        phoneme_ipa,
                        "Ejektive erfordern larynx.can_produce_ejectives = true",
                    ));
                }
            }
            _ => {} // Unknown airflow types pass through
        }
        Ok(())
    }

    /// Validates labial articulation (validation.txt lines 17-31)
    pub fn validate_labial_articulation(
        &self,
        phoneme_ipa: &str,
        labial_type: &str,
    ) -> Result<(), ValidationError> {
        use crate::anatomy::speaker::{LipControl, TeethConfiguration, TonguePartControl};

        match labial_type {
            "bilabial" => {
                // Lines 18-21: [p, b, m, w] require lips ≠ None
                if matches!(self.lips, LipControl::None) {
                    return Err(self.phoneme_anatomically_impossible(
                        phoneme_ipa,
                        "Bilabiale Laute [p, b, m, w] erfordern lips ≠ None",
                    ));
                }
            }
            "labiodental" => {
                // Lines 23-26: [f, v] require lips = Flexible AND teeth ∈ {Human, Flat}
                if !matches!(self.lips, LipControl::Flexible) {
                    return Err(self.phoneme_anatomically_impossible(
                        phoneme_ipa,
                        "Labiodentale Laute [f, v] erfordern lips = Flexible",
                    ));
                }
                if matches!(
                    self.oral_cavity.teeth,
                    TeethConfiguration::None | TeethConfiguration::Fangs
                ) {
                    return Err(self.phoneme_anatomically_impossible(
                        phoneme_ipa,
                        "Labiodentale Laute [f, v] erfordern teeth ∈ {Human, Flat}. Fangs/None → UNMÖGLICH"
                    ));
                }
            }
            "linguolabial" => {
                // Lines 28-31: [t̼, d̼] (RARE) require lips = Flexible AND tongue.tip = Agile
                if !matches!(self.lips, LipControl::Flexible) {
                    return Err(self.phoneme_anatomically_impossible(
                        phoneme_ipa,
                        "Linguolabiale Laute [t̼, d̼] erfordern lips = Flexible",
                    ));
                }
                if !matches!(self.tongue.tip, TonguePartControl::Agile) {
                    return Err(self.phoneme_anatomically_impossible(
                        phoneme_ipa,
                        "Linguolabiale Laute [t̼, d̼] erfordern tongue.tip = Agile",
                    ));
                }
            }
            _ => {}
        }
        Ok(())
    }

    /// Validates coronal articulation (validation.txt lines 33-58)
    pub fn validate_coronal_articulation(
        &self,
        phoneme_ipa: &str,
        coronal_type: &str,
    ) -> Result<(), ValidationError> {
        use crate::anatomy::speaker::{TeethConfiguration, TonguePartControl};

        match coronal_type {
            "dental" => {
                // Lines 34-37: [θ, ð] require teeth ≠ None AND tongue.tip/blade ≠ None
                if matches!(self.oral_cavity.teeth, TeethConfiguration::None) {
                    return Err(self.phoneme_anatomically_impossible(
                        phoneme_ipa,
                        "Dentale Laute [θ, ð] erfordern teeth ≠ None",
                    ));
                }
                if matches!(self.tongue.tip, TonguePartControl::None)
                    && matches!(self.tongue.blade, TonguePartControl::None)
                {
                    return Err(self.phoneme_anatomically_impossible(
                        phoneme_ipa,
                        "Dentale Laute [θ, ð] erfordern tongue.tip/blade ≠ None",
                    ));
                }
            }
            "alveolar" => {
                // Lines 39-43: [t, d, n, s, z, l, r] require alveolar_ridge = true AND tongue.tip/blade ≠ None
                if !self.oral_cavity.has_alveolar_ridge {
                    return Err(self.phoneme_anatomically_impossible(
                        phoneme_ipa,
                        "Alveolare Laute [t, d, n, s, z, l, r] erfordern has_alveolar_ridge = true",
                    ));
                }
                if matches!(self.tongue.tip, TonguePartControl::None)
                    && matches!(self.tongue.blade, TonguePartControl::None)
                {
                    return Err(self.phoneme_anatomically_impossible(
                        phoneme_ipa,
                        "Alveolare Laute erfordern tongue.tip/blade ≠ None",
                    ));
                }
                // Line 42: Laterals [l] require can_perform_lateral_release = true
                if phoneme_ipa.contains('l') && !self.tongue.can_perform_lateral_release {
                    return Err(self.phoneme_anatomically_impossible(
                        phoneme_ipa,
                        "Laterale [l] erfordern tongue.can_perform_lateral_release = true",
                    ));
                }
            }
            "postalveolar" => {
                // Lines 46-49: [ʃ, ʒ, tʃ, dʒ] require alveolar_ridge + hard_palate + tongue.blade = Agile
                if !self.oral_cavity.has_alveolar_ridge || !self.oral_cavity.has_hard_palate {
                    return Err(self.phoneme_anatomically_impossible(
                        phoneme_ipa,
                        "Postalveolare [ʃ, ʒ, tʃ, dʒ] erfordern has_alveolar_ridge = true UND has_hard_palate = true"
                    ));
                }
                if !matches!(self.tongue.blade, TonguePartControl::Agile) {
                    return Err(self.phoneme_anatomically_impossible(
                        phoneme_ipa,
                        "Postalveolare erfordern tongue.blade = Agile",
                    ));
                }
            }
            "retroflex" => {
                // Lines 55-58: [ʈ, ɖ, ɳ, ʂ, ʐ] require can_curl_for_retroflex = true AND tongue.tip = Agile
                if !self.tongue.can_curl_for_retroflex {
                    return Err(self.phoneme_anatomically_impossible(
                        phoneme_ipa,
                        "Retroflexe [ʈ, ɖ, ɳ, ʂ, ʐ] erfordern tongue.can_curl_for_retroflex = true",
                    ));
                }
                if !matches!(self.tongue.tip, TonguePartControl::Agile) {
                    return Err(self.phoneme_anatomically_impossible(
                        phoneme_ipa,
                        "Retroflexe erfordern tongue.tip = Agile",
                    ));
                }
            }
            "palatal" => {
                // Lines 60-63: [c, ɟ, ɲ, j] require hard_palate = true AND tongue.body = Agile
                if !self.oral_cavity.has_hard_palate {
                    return Err(self.phoneme_anatomically_impossible(
                        phoneme_ipa,
                        "Palatale [c, ɟ, ɲ, j] erfordern has_hard_palate = true",
                    ));
                }
                if !matches!(self.tongue.body, TonguePartControl::Agile) {
                    return Err(self.phoneme_anatomically_impossible(
                        phoneme_ipa,
                        "Palatale erfordern tongue.body = Agile",
                    ));
                }
            }
            _ => {}
        }
        Ok(())
    }

    /// Validates dorsal articulation (validation.txt lines 65-74)
    pub fn validate_dorsal_articulation(
        &self,
        phoneme_ipa: &str,
        dorsal_type: &str,
    ) -> Result<(), ValidationError> {
        use crate::anatomy::speaker::TonguePartControl;

        match dorsal_type {
            "velar" => {
                // Lines 66-69: [k, g, ŋ, x, ɣ] require soft_palate = true AND tongue.body = Agile
                if !self.oral_cavity.has_soft_palate {
                    return Err(self.phoneme_anatomically_impossible(
                        phoneme_ipa,
                        "Velare [k, g, ŋ, x, ɣ] erfordern has_soft_palate = true",
                    ));
                }
                if !matches!(self.tongue.body, TonguePartControl::Agile) {
                    return Err(self.phoneme_anatomically_impossible(
                        phoneme_ipa,
                        "Velare erfordern tongue.body = Agile",
                    ));
                }
            }
            "uvular" => {
                // Lines 71-74: [q, ɢ, ɴ, χ, ʁ] require uvula = true AND tongue.body = Agile
                if !self.oral_cavity.has_uvula {
                    return Err(self.phoneme_anatomically_impossible(
                        phoneme_ipa,
                        "Uvulare [q, ɢ, ɴ, χ, ʁ] erfordern has_uvula = true",
                    ));
                }
                if !matches!(self.tongue.body, TonguePartControl::Agile) {
                    return Err(self.phoneme_anatomically_impossible(
                        phoneme_ipa,
                        "Uvulare erfordern tongue.body = Agile",
                    ));
                }
            }
            _ => {}
        }
        Ok(())
    }

    /// Validates voicing requirements (validation.txt lines 89-104)
    pub fn validate_voicing_requirements(
        &self,
        phoneme_ipa: &str,
        voicing_type: &str,
    ) -> Result<(), ValidationError> {
        use crate::anatomy::speaker::{PulmonicControl, VoicingControl};

        match voicing_type {
            "voiceless" => {
                // Line 91: Basic requirement larynx.voicing ≠ None
                if matches!(self.larynx.voicing, VoicingControl::None) {
                    return Err(self.phoneme_anatomically_impossible(
                        phoneme_ipa,
                        "Stimmlose Laute erfordern larynx.voicing ≠ None",
                    ));
                }
            }
            "voiced" => {
                // Lines 94-96: larynx.voicing ∈ {Basic, Advanced}
                if matches!(self.larynx.voicing, VoicingControl::None) {
                    return Err(self.phoneme_anatomically_impossible(
                        phoneme_ipa,
                        "Stimmhafte Laute erfordern larynx.voicing ∈ {Basic, Advanced}",
                    ));
                }
            }
            "aspirated" => {
                // Lines 98-100: [pʰ, tʰ, kʰ] require Advanced voicing AND pulmonic
                if !matches!(self.larynx.voicing, VoicingControl::Advanced) {
                    return Err(self.phoneme_anatomically_impossible(
                        phoneme_ipa,
                        "Aspirierte Laute [pʰ, tʰ, kʰ] erfordern larynx.voicing = Advanced",
                    ));
                }
                if !matches!(self.airflow.pulmonic, PulmonicControl::Advanced) {
                    return Err(self.phoneme_anatomically_impossible(
                        phoneme_ipa,
                        "Aspirierte Laute erfordern airflow.pulmonic = Advanced",
                    ));
                }
            }
            "ejective" => {
                // Lines 102-104: Already handled in validate_airflow_mechanism
                if !self.larynx.can_produce_ejectives {
                    return Err(self.phoneme_anatomically_impossible(
                        phoneme_ipa,
                        "Ejektive [p', t', k'] erfordern larynx.can_produce_ejectives = true",
                    ));
                }
            }
            _ => {}
        }
        Ok(())
    }

    /// Validates nasality requirements (validation.txt lines 106-114)
    pub fn validate_nasality_requirements(
        &self,
        phoneme_ipa: &str,
        is_nasal: bool,
    ) -> Result<(), ValidationError> {
        use crate::anatomy::speaker::VelicPortControl;

        if is_nasal {
            // Lines 111-114: Nasal [m, n, ŋ, ɲ] require velic_port = Controllable AND soft_palate = true
            if !matches!(self.airflow.velic_port, VelicPortControl::Controllable) {
                return Err(self.phoneme_anatomically_impossible(
                    phoneme_ipa,
                    "Nasale [m, n, ŋ, ɲ] erfordern airflow.velic_port = Controllable",
                ));
            }
            if !self.oral_cavity.has_soft_palate {
                return Err(self.phoneme_anatomically_impossible(
                    phoneme_ipa,
                    "Nasale erfordern has_soft_palate = true",
                ));
            }
        }
        Ok(())
    }
}
