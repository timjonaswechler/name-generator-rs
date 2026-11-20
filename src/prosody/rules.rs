use super::*;
use crate::anatomy::speaker::SpeakerAnatomy;
use crate::phonology::{LanguageInventory, SoundCatalog, SoundFeatures, Voicing};
use crate::syllables::{Presence, Syllable};
use crate::validation::{ValidationError, ValidationErrors};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DurationRules {
    pub vowels: Vec<Duration>,
    pub consonants: Vec<Duration>,
}
impl Default for DurationRules {
    fn default() -> Self {
        Self {
            vowels: vec![Duration::Normal],
            consonants: vec![Duration::Normal],
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WordEdge {
    Start,
    End,
}
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum StressRule {
    #[default]
    None,
    /// Zero-based syllable offset. Too-short words are rejected, not silently reinterpreted.
    Fixed {
        edge: WordEdge,
        offset: usize,
        allow_secondary: bool,
    },
    Lexical {
        require_primary: bool,
        allow_secondary: bool,
    },
}
impl StressRule {
    pub fn enabled(&self) -> bool {
        !matches!(self, Self::None)
    }
    pub fn allows_secondary(&self) -> bool {
        matches!(
            self,
            Self::Fixed {
                allow_secondary: true,
                ..
            } | Self::Lexical {
                allow_secondary: true,
                ..
            }
        )
    }
    pub fn primary_index(&self, syllables: usize) -> Result<Option<usize>, &'static str> {
        match *self {
            Self::Fixed { edge, offset, .. } if offset < syllables => Ok(Some(match edge {
                WordEdge::Start => offset,
                WordEdge::End => syllables - 1 - offset,
            })),
            Self::Fixed { .. } => Err("word is too short for fixed stress"),
            _ => Ok(None),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ToneRules {
    pub presence: Presence,
    pub inventory: Vec<Tone>,
}
impl Default for ToneRules {
    fn default() -> Self {
        Self {
            presence: Presence::Forbidden,
            inventory: vec![],
        }
    }
}
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProsodyRules {
    pub duration: DurationRules,
    pub stress: StressRule,
    pub tone: ToneRules,
}
fn error(errors: &mut ValidationErrors, field: &str, code: &'static str) {
    errors.add(field.to_owned(), ValidationError::new(code));
}
impl ProsodyRules {
    pub fn validate(
        &self,
        anatomy: &SpeakerAnatomy,
        catalog: &SoundCatalog,
        inventory: &LanguageInventory,
        phonotactics: &crate::syllables::Phonotactics,
    ) -> Result<(), ValidationErrors> {
        let mut errors = ValidationErrors::new();
        for (consonant, durations) in [
            (false, &self.duration.vowels),
            (true, &self.duration.consonants),
        ] {
            let used = inventory.iter().any(|id| {
                catalog
                    .get(id)
                    .is_some_and(|s| s.is_consonant() == consonant)
            });
            if used && durations.is_empty() {
                error(&mut errors, "duration", "no_permitted_duration");
            }
            for (i, duration) in durations.iter().enumerate() {
                if durations[..i].contains(duration) {
                    error(&mut errors, "duration", "duplicate_duration");
                }
                if used && !anatomy.prosody.durations.contains(duration) {
                    error(&mut errors, "duration", "duration_not_supported_by_anatomy");
                }
            }
        }
        if self.stress.enabled() && !anatomy.prosody.can_stress {
            error(&mut errors, "stress", "stress_not_supported_by_anatomy");
        }
        if self.tone.presence == Presence::Forbidden {
            if !self.tone.inventory.is_empty() {
                error(&mut errors, "tone", "forbidden_tone_has_inventory");
            }
        } else if self.tone.inventory.is_empty() {
            error(&mut errors, "tone", "empty_tone_inventory");
        }
        if self.tone.presence == Presence::Required {
            for choice in phonotactics
                .nucleus
                .choices
                .iter()
                .filter(|c| c.weight.is_enabled())
            {
                if !choice.sounds.iter().any(|id| {
                    catalog.get(id).is_some_and(|s| match s.features() {
                        SoundFeatures::Consonant(f) => f.voicing == Voicing::Voiced,
                        SoundFeatures::Vowel(f) => f.voicing == Voicing::Voiced,
                    })
                }) {
                    error(
                        &mut errors,
                        "tone",
                        "required_tone_has_unvoiced_nucleus_choice",
                    );
                }
            }
        }
        for (i, tone) in self.tone.inventory.iter().enumerate() {
            if self.tone.inventory[..i].contains(tone) {
                error(&mut errors, "tone", "duplicate_tone");
            }
            if !anatomy.larynx.voicing.voiced
                || tone
                    .contour
                    .levels()
                    .iter()
                    .any(|p| !anatomy.prosody.pitch_levels.contains(p))
                || (tone.contour.levels().len() > 1 && !anatomy.prosody.can_contour_tones)
                || (tone.register != ToneRegister::Normal && !anatomy.prosody.can_shift_register)
            {
                error(&mut errors, "tone", "tone_not_supported_by_anatomy");
            }
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    pub(crate) fn check_word(
        &self,
        syllables: &[Syllable],
        catalog: &SoundCatalog,
    ) -> Result<(), ValidationErrors> {
        let mut errors = ValidationErrors::new();
        let primary: Vec<_> = syllables
            .iter()
            .enumerate()
            .filter_map(|(i, s)| (s.prosody.stress == Stress::Primary).then_some(i))
            .collect();
        let secondary = syllables
            .iter()
            .any(|s| s.prosody.stress == Stress::Secondary);
        if primary.len() > 1 {
            error(&mut errors, "stress", "multiple_primary_stresses");
        }
        if secondary && (!self.stress.allows_secondary() || primary.is_empty()) {
            error(&mut errors, "stress", "secondary_stress_not_permitted");
        }
        match &self.stress {
            StressRule::None if !primary.is_empty() => {
                error(&mut errors, "stress", "stress_is_forbidden")
            }
            StressRule::Fixed { .. } => match self.stress.primary_index(syllables.len()) {
                Ok(Some(index)) if primary == [index] => {}
                _ => error(&mut errors, "stress", "fixed_stress_does_not_match"),
            },
            StressRule::Lexical {
                require_primary: true,
                ..
            } if primary.len() != 1 => error(&mut errors, "stress", "primary_stress_required"),
            _ => {}
        }
        for syllable in syllables {
            for segment in syllable.segments() {
                if let Some(sound) = catalog.get(&segment.sound) {
                    let allowed = if sound.is_consonant() {
                        &self.duration.consonants
                    } else {
                        &self.duration.vowels
                    };
                    if !allowed.contains(&segment.duration) {
                        error(&mut errors, "duration", "duration_not_permitted");
                    }
                } else {
                    error(&mut errors, "duration", "unknown_sound");
                }
            }
            match &syllable.prosody.tone {
                None if self.tone.presence == Presence::Required => {
                    error(&mut errors, "tone", "tone_required")
                }
                Some(tone) => {
                    if self.tone.presence == Presence::Forbidden
                        || !self.tone.inventory.contains(tone)
                    {
                        error(&mut errors, "tone", "tone_not_permitted");
                    }
                    let voiced_nucleus = syllable.nucleus.iter().any(|segment| {
                        catalog
                            .get(&segment.sound)
                            .is_some_and(|s| match s.features() {
                                SoundFeatures::Consonant(f) => f.voicing == Voicing::Voiced,
                                SoundFeatures::Vowel(f) => f.voicing == Voicing::Voiced,
                            })
                    });
                    if !voiced_nucleus {
                        error(&mut errors, "tone", "tone_requires_voiced_nucleus");
                    }
                }
                _ => {}
            }
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}
