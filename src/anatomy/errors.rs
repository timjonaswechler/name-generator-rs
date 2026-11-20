use super::speaker::*;
use crate::validation::{ValidationError, ValidationErrors};

#[derive(Debug, Clone, Default)]
pub struct AnatomyReport {
    pub warnings: Vec<ValidationError>,
}

impl SpeakerAnatomy {
    /// Inconsistent capabilities are errors; reduced or unusual anatomy is not.
    pub fn validate_anatomical_consistency(&self) -> Result<AnatomyReport, ValidationErrors> {
        let mut errors = ValidationErrors::new();
        let mut report = AnatomyReport::default();
        if self.airflow.velic_port == VelicPortControl::Controllable
            && !self.oral_cavity.has_soft_palate
        {
            errors.add(
                "airflow",
                ValidationError::new("velic_control_requires_soft_palate"),
            );
        }
        if self.tongue.can_curl_for_retroflex && self.tongue.tip != TonguePartControl::Agile {
            errors.add(
                "tongue",
                ValidationError::new("retroflex_requires_agile_tip"),
            );
        }
        if self.tongue.can_perform_lateral_release
            && [self.tongue.tip, self.tongue.blade, self.tongue.body]
                .iter()
                .all(|p| *p == TonguePartControl::None)
        {
            errors.add(
                "tongue",
                ValidationError::new("lateral_control_requires_tongue"),
            );
        }
        if (self.larynx.can_produce_ejectives || self.larynx.can_produce_implosives)
            && !self.larynx.can_close_glottis
        {
            errors.add(
                "larynx",
                ValidationError::new("glottalic_airflow_requires_glottal_closure"),
            );
        }
        if self.oral_cavity.teeth == TeethConfiguration::None
            && (self.oral_cavity.dental_contact == ContactCapability::Available
                || self.oral_cavity.labiodental_contact == ContactCapability::Available)
        {
            errors.add(
                "teeth",
                ValidationError::new("dental_contact_requires_teeth"),
            );
        }
        if self.lips == LipControl::Flexible && self.oral_cavity.teeth == TeethConfiguration::None {
            report
                .warnings
                .push(ValidationError::new("toothless_limits_dental_contact"));
        }
        if self.lips == LipControl::None && self.oral_cavity.teeth == TeethConfiguration::Human {
            report
                .warnings
                .push(ValidationError::new("teeth_without_lips_unusual"));
        }
        if self.oral_cavity.has_alveolar_ridge
            && self.tongue.tip == TonguePartControl::None
            && self.tongue.blade == TonguePartControl::None
        {
            report
                .warnings
                .push(ValidationError::new("alveolar_ridge_currently_unusable"));
        }
        if errors.is_empty() {
            Ok(report)
        } else {
            Err(errors)
        }
    }
}
