//! Improved SpeakerAnatomy with encapsulated fields and public API

use serde::{Deserialize, Serialize};

/// Public interface for speaker anatomy - hides internal complexity
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpeakerAnatomy {
    // All fields are private - access only through methods
    oral_cavity: OralCavity,
    lips: LipControl,
    tongue: TongueControl,
    larynx: LarynxControl,
    airflow: AirflowControl,
}

impl SpeakerAnatomy {
    /// Create a new speaker anatomy (internal constructor)
    pub(crate) fn new(
        oral_cavity: OralCavity,
        lips: LipControl,
        tongue: TongueControl,
        larynx: LarynxControl,
        airflow: AirflowControl,
    ) -> Self {
        Self {
            oral_cavity,
            lips,
            tongue,
            larynx,
            airflow,
        }
    }

    /// Check if the speaker can produce a specific phoneme type
    pub fn can_produce_consonant(&self, place: Place, manner: Manner) -> bool {
        match place {
            Place::Labial => self.lips != LipControl::Immobile,
            Place::Alveolar => self.oral_cavity.has_alveolar_ridge,
            Place::Palatal => self.oral_cavity.has_hard_palate,
            Place::Velar => self.oral_cavity.has_soft_palate,
            // Add other place validations
            _ => true, // Simplified for example
        }
    }

    /// Check if speaker can produce vowels with specific characteristics
    pub fn can_produce_vowel(&self, height: VowelHeight, position: VowelPosition) -> bool {
        // Tongue control requirements for vowel production
        match (height, position) {
            (VowelHeight::Close, _) => self.tongue.can_raise_high(),
            (VowelHeight::Open, _) => self.tongue.can_lower(),
            (_, VowelPosition::Front) => self.tongue.can_move_forward(),
            (_, VowelPosition::Back) => self.tongue.can_move_back(),
            _ => true,
        }
    }

    /// Get available articulation places (public method)
    pub fn available_places(&self) -> Vec<Place> {
        let mut places = Vec::new();

        if self.lips != LipControl::Immobile {
            places.push(Place::Labial);
        }
        if self.oral_cavity.has_alveolar_ridge {
            places.push(Place::Alveolar);
        }
        if self.oral_cavity.has_hard_palate {
            places.push(Place::Palatal);
        }
        if self.oral_cavity.has_soft_palate {
            places.push(Place::Velar);
        }

        places
    }

    /// Get voicing capabilities
    pub fn voicing_capabilities(&self) -> VoicingCapabilities {
        VoicingCapabilities {
            can_voice: self.larynx.voicing != VoicingControl::None,
            can_devoice: true, // Most speakers can devoice
            can_breathy: self.larynx.voicing == VoicingControl::Full,
            can_creaky: self.larynx.voicing == VoicingControl::Full,
        }
    }

    /// Create a standard human speaker configuration
    pub fn human() -> Self {
        Self {
            oral_cavity: OralCavity::human(),
            lips: LipControl::Full,
            tongue: TongueControl::human(),
            larynx: LarynxControl::human(),
            airflow: AirflowControl::human(),
        }
    }

    // Internal access methods for crate use only
    pub(crate) fn oral_cavity(&self) -> &OralCavity {
        &self.oral_cavity
    }

    pub(crate) fn tongue(&self) -> &TongueControl {
        &self.tongue
    }
}

/// Public voicing capabilities summary
#[derive(Debug, Clone, PartialEq)]
pub struct VoicingCapabilities {
    pub can_voice: bool,
    pub can_devoice: bool,
    pub can_breathy: bool,
    pub can_creaky: bool,
}

/// Internal oral cavity structure - not directly accessible
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct OralCavity {
    teeth: TeethConfiguration,
    has_alveolar_ridge: bool,
    has_hard_palate: bool,
    has_soft_palate: bool,
    has_uvula: bool,
    has_epiglottis: bool,
}

impl OralCavity {
    fn human() -> Self {
        Self {
            teeth: TeethConfiguration::Full,
            has_alveolar_ridge: true,
            has_hard_palate: true,
            has_soft_palate: true,
            has_uvula: true,
            has_epiglottis: true,
        }
    }
}

/// Internal lip control - simplified public interface
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
enum LipControl {
    None,     // No lip control
    Basic,    // Basic lip movements
    Full,     // Full lip control including rounding
    Immobile, // Cannot move lips
}

/// Internal tongue control structure
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct TongueControl {
    tip: TonguePartControl,
    blade: TonguePartControl,
    body: TonguePartControl,
    root: TonguePartControl,
    can_curl_for_retroflex: bool,
    can_perform_lateral_release: bool,
}

impl TongueControl {
    fn human() -> Self {
        Self {
            tip: TonguePartControl::Full,
            blade: TonguePartControl::Full,
            body: TonguePartControl::Full,
            root: TonguePartControl::Basic,
            can_curl_for_retroflex: true,
            can_perform_lateral_release: true,
        }
    }

    fn can_raise_high(&self) -> bool {
        self.body == TonguePartControl::Full
    }

    fn can_lower(&self) -> bool {
        self.body != TonguePartControl::None
    }

    fn can_move_forward(&self) -> bool {
        self.body != TonguePartControl::None
    }

    fn can_move_back(&self) -> bool {
        self.body != TonguePartControl::None
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
enum TonguePartControl {
    None,  // No control
    Basic, // Limited control
    Full,  // Complete control
}

/// Internal larynx control
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct LarynxControl {
    voicing: VoicingControl,
    can_produce_ejectives: bool,
}

impl LarynxControl {
    fn human() -> Self {
        Self {
            voicing: VoicingControl::Full,
            can_produce_ejectives: true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
enum VoicingControl {
    None,  // Cannot control voicing
    Basic, // Can voice/devoice
    Full,  // Full voicing control including breathy/creaky
}

/// Internal airflow control
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct AirflowControl {
    pulmonic: PulmonicControl,
    velic_port: VelicPortControl,
    can_produce_clicks: bool,
}

impl AirflowControl {
    fn human() -> Self {
        Self {
            pulmonic: PulmonicControl::Full,
            velic_port: VelicPortControl::Full,
            can_produce_clicks: true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
enum PulmonicControl {
    None,
    Basic,
    Full,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
enum VelicPortControl {
    None,  // Cannot control nasal/oral airflow
    Basic, // Can open/close
    Full,  // Full control over nasal resonance
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TeethConfiguration {
    None,    // No teeth
    Partial, // Some teeth missing
    Full,    // Complete set of teeth
}

// Re-export only what users need for the public API
pub use crate::phonology::{Manner, Place, VowelHeight, VowelPosition};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_human_speaker_capabilities() {
        let speaker = SpeakerAnatomy::human();

        // Should be able to produce common consonants
        assert!(speaker.can_produce_consonant(Place::Labial, Manner::Plosive));
        assert!(speaker.can_produce_consonant(Place::Alveolar, Manner::Nasal));

        // Should be able to produce common vowels
        assert!(speaker.can_produce_vowel(VowelHeight::Close, VowelPosition::Front));
        assert!(speaker.can_produce_vowel(VowelHeight::Open, VowelPosition::Central));
    }

    #[test]
    fn test_available_places() {
        let speaker = SpeakerAnatomy::human();
        let places = speaker.available_places();

        assert!(places.contains(&Place::Labial));
        assert!(places.contains(&Place::Alveolar));
        assert!(places.contains(&Place::Palatal));
    }
}
