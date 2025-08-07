use crate::anatomy::speaker::SpeakerAnatomy;
use crate::phonology::{
    AirflowMechanism, Consonant, Diacritics, Manner, PhonologyConfiguration, Place, Roundness,
    Subplace, Vowel,
};
use crate::validation::{ValidationError, ValidationErrors};
use std::borrow::Cow;

impl Default for PhonologyConfiguration {
    fn default() -> Self {
        PhonologyConfiguration {
            consonants: Vec::new(),
            vowels: Vec::new(),
            symmetrics_score: 0.0, // Default Wert für symmetrics_score
        }
    }
}

impl PhonologyConfiguration {
    pub fn new() -> PhonologyConfiguration {
        PhonologyConfiguration {
            consonants: Vec::new(),
            vowels: Vec::new(),
            symmetrics_score: 0.0, // Initialisiere mit 0.0
        }
    }

    pub fn consonants(&self) -> &[Cow<'static, Consonant>] {
        &self.consonants
    }

    pub fn vowels(&self) -> &[Cow<'static, Vowel>] {
        &self.vowels
    }

    pub fn add_vowels(
        mut self,
        vowels: Vec<&'static Vowel>,
    ) -> Result<PhonologyConfiguration, ValidationErrors> {
        let mut errors = ValidationErrors::new();

        for vowel in vowels {
            // 1. Duplicate-Check
            if self.vowels.iter().any(|v| v.ipa == vowel.ipa) {
                errors.add("duplicate_vowel", self.duplicate_vowel(&vowel.ipa));
            }

            self.vowels.push(Cow::Borrowed(vowel));
        }

        if errors.is_empty() {
            Ok(self)
        } else {
            Err(errors)
        }
    }

    pub fn add_consonants(
        mut self,
        consonants: Vec<&'static Consonant>,
    ) -> Result<PhonologyConfiguration, ValidationErrors> {
        let mut errors = ValidationErrors::new();

        for consonant in consonants {
            // 1. Duplicate-Check
            if self.consonants.iter().any(|c| c.ipa == consonant.ipa) {
                errors.add(
                    "duplicate_consonant",
                    self.duplicate_consonant(&consonant.ipa),
                );
            }

            self.consonants.push(Cow::Borrowed(consonant));
        }

        if errors.is_empty() {
            Ok(self)
        } else {
            Err(errors)
        }
    }

    pub fn validate_against_anatomy(
        &self,
        anatomy: &SpeakerAnatomy,
    ) -> Result<(), ValidationErrors> {
        let mut errors = ValidationErrors::new();

        // Validiere alle Konsonanten
        for consonant_cow in &self.consonants {
            let consonant = consonant_cow.as_ref();
            if let Err(validation_error) =
                self.validate_consonant_against_anatomy(consonant, anatomy)
            {
                errors.add(format!("consonant_{}", consonant.ipa), validation_error);
            }
        }

        // Validiere alle Vokale
        for vowel_cow in &self.vowels {
            let vowel = vowel_cow.as_ref();
            if let Err(validation_error) = self.validate_vowel_against_anatomy(vowel, anatomy) {
                errors.add(format!("vowel_{}", vowel.ipa), validation_error);
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
    // Hauptvalidierungsfunktion für einen Konsonanten
    fn validate_consonant_against_anatomy(
        &self,
        consonant: &Consonant,
        anatomy: &SpeakerAnatomy,
    ) -> Result<(), ValidationError> {
        // Hier nutzt du die Funktionen aus errors.rs:

        // Je nach consonant.place und consonant.manner die richtige Validierung aufrufen
        match consonant.phoneme.place {
            Place::Labial => {
                let subplace = self.determine_labial_subplace(consonant);
                anatomy.validate_labial_articulation(&consonant.ipa, &subplace)?;
            }
            Place::Coronal => {
                let subplace = self.determine_coronal_subplace(consonant);
                anatomy.validate_coronal_articulation(&consonant.ipa, &subplace)?;
            }
            Place::Dorsal => {
                let subplace = self.determine_dorsal_subplace(consonant);
                anatomy.validate_dorsal_articulation(&consonant.ipa, &subplace)?;
            }
            Place::Laryngeal => {
                // Laryngeal validation
            }
        }

        // Airflow-Validierung
        let airflow_type = self.determine_airflow_type(consonant);
        anatomy.validate_airflow_mechanism(&consonant.ipa, &airflow_type)?;

        // Voicing-Validierung
        let voicing_type = self.determine_voicing_type(consonant);
        anatomy.validate_voicing_requirements(&consonant.ipa, &voicing_type)?;

        // Nasalität-Validierung
        let is_nasal = matches!(consonant.phoneme.manner, Manner::Nasal);
        anatomy.validate_nasality_requirements(&consonant.ipa, is_nasal)?;

        Ok(())
    }

    fn validate_vowel_against_anatomy(
        &self,
        vowel: &Vowel,
        anatomy: &SpeakerAnatomy,
    ) -> Result<(), ValidationError> {
        // Vokale haben weniger anatomische Constraints als Konsonanten
        // Aber du könntest hier z.B. prüfen:
        // - Extreme Vokal-Positionen erfordern bestimmte Zungen-Agilität
        // - Gerundete Vokale erfordern Lippen-Kontrolle

        match vowel.phoneme.roundness {
            Roundness::Rounded => {
                // Gerundete Vokale [u, o, ɔ] erfordern Lippen-Kontrolle
                if matches!(anatomy.lips, crate::anatomy::speaker::LipControl::None) {
                    return Err(anatomy.phoneme_anatomically_impossible(
                        &vowel.ipa,
                        "Gerundete Vokale erfordern Lippen-Kontrolle",
                    ));
                }
            }
            Roundness::Unrounded => {
                // Ungerundete Vokale haben keine speziellen Anforderungen
            }
        }

        Ok(())
    }
    // DETERMINE-FUNKTIONEN (basierend auf ConsonantConfiguration):

    fn determine_labial_subplace(&self, consonant: &Consonant) -> String {
        match consonant.phoneme.subplace {
            Subplace::Bilabial => "bilabial".to_string(),
            Subplace::Labiodental => "labiodental".to_string(),
            Subplace::Linguolabial => "linguolabial".to_string(),
            _ => "bilabial".to_string(), // Default fallback
        }
    }

    fn determine_coronal_subplace(&self, consonant: &Consonant) -> String {
        match consonant.phoneme.subplace {
            Subplace::Dental => "dental".to_string(),
            Subplace::Alveolar => "alveolar".to_string(),
            Subplace::Postalveolar => "postalveolar".to_string(),
            Subplace::Retroflex => "retroflex".to_string(),
            Subplace::Palatal => "palatal".to_string(),
            _ => "alveolar".to_string(), // Default fallback
        }
    }

    fn determine_dorsal_subplace(&self, consonant: &Consonant) -> String {
        match consonant.phoneme.subplace {
            Subplace::Palatal => "palatal".to_string(),
            Subplace::Velar => "velar".to_string(),
            Subplace::Uvular => "uvular".to_string(),
            _ => "velar".to_string(), // Default fallback
        }
    }

    fn determine_airflow_type(&self, consonant: &Consonant) -> String {
        match consonant.phoneme.airflow {
            AirflowMechanism::Pulmonic => "pulmonic".to_string(),
            AirflowMechanism::Click => "click".to_string(),
            AirflowMechanism::Ejective => "ejective".to_string(),
            AirflowMechanism::Implosive => "implosive".to_string(),
        }
    }

    fn determine_voicing_type(&self, consonant: &Consonant) -> String {
        // Prüfe Diacritics für Voicing-Information
        for diacritic in consonant.phoneme.diacritics.iter() {
            match diacritic {
                Diacritics::Voiced => return "voiced".to_string(),
                Diacritics::Voiceless => return "voiceless".to_string(),
                Diacritics::Aspirated => return "aspirated".to_string(),
                Diacritics::Ejective => return "ejective".to_string(),
                _ => continue,
            }
        }

        // Fallback: Basierend auf typischer Voicing von Manner
        match consonant.phoneme.manner {
            Manner::Nasal => "voiced".to_string(),
            Manner::Plosive => "voiceless".to_string(), // Default, kann überschrieben werden
            Manner::SibilantFricative => "voiceless".to_string(),
            Manner::NonSibilantFricative => "voiceless".to_string(),
            Manner::Approximant => "voiced".to_string(),
            Manner::TapFlap => "voiced".to_string(),
            Manner::Trill => "voiced".to_string(),
            Manner::LateralApproximant => "voiced".to_string(),
            Manner::Affricate => "voiceless".to_string(),
        }
    }
}
