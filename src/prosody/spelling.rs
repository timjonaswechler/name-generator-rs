//! A feature may be unwritten, but only by an explicit spelling choice.
use super::*;
use crate::validation::{ValidationError, ValidationErrors};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum WrittenMark {
    Omit,
    Text(String),
}
impl WrittenMark {
    pub(crate) fn text(&self) -> Result<&str, &'static str> {
        match self {
            Self::Omit => Ok(""),
            Self::Text(text) if !text.trim().is_empty() => Ok(text),
            Self::Text(_) => Err("empty_prosody_mark_use_explicit_omit"),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DurationSpelling {
    Omit,
    Repeat,
    Affix { prefix: String, suffix: String },
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ToneSpelling {
    pub tone: Tone,
    pub mark: WrittenMark,
}
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProsodySpelling {
    pub duration: BTreeMap<Duration, DurationSpelling>,
    pub primary_stress: Option<WrittenMark>,
    pub secondary_stress: Option<WrittenMark>,
    pub tones: Vec<ToneSpelling>,
}
impl ProsodySpelling {
    pub fn validate(&self, rules: &ProsodyRules) -> Result<(), ValidationErrors> {
        let mut errors = ValidationErrors::new();
        for mark in self
            .primary_stress
            .iter()
            .chain(self.secondary_stress.iter())
            .chain(self.tones.iter().map(|s| &s.mark))
        {
            if let Err(code) = mark.text() {
                errors.add("spelling", ValidationError::new(code));
            }
        }
        for marking in self.duration.values() {
            if matches!(marking, DurationSpelling::Affix { prefix, suffix } if prefix.trim().is_empty() && suffix.trim().is_empty())
            {
                errors.add(
                    "spelling",
                    ValidationError::new("empty_duration_affix_use_explicit_omit"),
                );
            }
        }
        for duration in rules
            .duration
            .vowels
            .iter()
            .chain(&rules.duration.consonants)
        {
            if *duration != Duration::Normal && !self.duration.contains_key(duration) {
                errors.add(
                    "spelling",
                    ValidationError::new("missing_duration_spelling"),
                );
            }
        }
        if rules.stress.enabled() && self.primary_stress.is_none() {
            errors.add(
                "spelling",
                ValidationError::new("missing_primary_stress_spelling"),
            );
        }
        if rules.stress.allows_secondary() && self.secondary_stress.is_none() {
            errors.add(
                "spelling",
                ValidationError::new("missing_secondary_stress_spelling"),
            );
        }
        for tone in &rules.tone.inventory {
            if !self.tones.iter().any(|s| s.tone == *tone) {
                errors.add("spelling", ValidationError::new("missing_tone_spelling"));
            }
        }
        for (i, spelling) in self.tones.iter().enumerate() {
            if self.tones[..i].iter().any(|s| s.tone == spelling.tone) {
                errors.add("spelling", ValidationError::new("duplicate_tone_spelling"));
            }
            if !rules.tone.inventory.contains(&spelling.tone) {
                errors.add("spelling", ValidationError::new("unknown_spelling_tone"));
            }
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
    pub(crate) fn segment(&self, base: &str, duration: Duration) -> Result<String, &'static str> {
        match self.duration.get(&duration) {
            None if duration == Duration::Normal => Ok(base.to_owned()),
            Some(DurationSpelling::Omit) => Ok(base.to_owned()),
            Some(DurationSpelling::Repeat) => Ok(base.repeat(2)),
            Some(DurationSpelling::Affix { prefix, suffix }) => {
                if prefix.trim().is_empty() && suffix.trim().is_empty() {
                    Err("empty_duration_affix_use_explicit_omit")
                } else {
                    Ok(format!("{prefix}{base}{suffix}"))
                }
            }
            None => Err("missing_duration_spelling"),
        }
    }
    pub(crate) fn stress(&self, stress: Stress) -> Result<&str, &'static str> {
        match stress {
            Stress::Unstressed => Ok(""),
            Stress::Primary => self
                .primary_stress
                .as_ref()
                .ok_or("missing_primary_stress_spelling")
                .and_then(WrittenMark::text),
            Stress::Secondary => self
                .secondary_stress
                .as_ref()
                .ok_or("missing_secondary_stress_spelling")
                .and_then(WrittenMark::text),
        }
    }
    pub(crate) fn tone(&self, tone: &Tone) -> Result<&str, &'static str> {
        self.tones
            .iter()
            .find(|s| s.tone == *tone)
            .ok_or("missing_tone_spelling")
            .and_then(|s| s.mark.text())
    }
}
