use super::*;
use crate::phonology::{LanguageInventory, SoundCatalog};
use crate::validation::{ValidationError, ValidationErrors};

fn error(errors: &mut ValidationErrors, field: &str, code: &'static str) {
    errors.add(field.to_owned(), ValidationError::new(code));
}
impl Phonotactics {
    pub fn validate(
        &self,
        catalog: &SoundCatalog,
        inventory: &LanguageInventory,
    ) -> Result<(), ValidationErrors> {
        let mut errors = ValidationErrors::new();
        if self.nucleus.presence != Presence::Required {
            error(&mut errors, "nucleus", "nucleus_must_be_required");
        }
        for (name, rules, nucleus) in [
            ("onset", &self.onset, false),
            ("nucleus", &self.nucleus, true),
            ("coda", &self.coda, false),
        ] {
            if rules.presence == Presence::Forbidden {
                if !rules.choices.is_empty() {
                    error(&mut errors, name, "forbidden_position_has_choices");
                }
            } else if !rules.choices.iter().any(|c| c.weight.is_enabled()) {
                error(&mut errors, name, "position_has_no_enabled_choices");
            }
            for choice in &rules.choices {
                if choice.sounds.is_empty() {
                    error(&mut errors, name, "empty_sound_sequence");
                }
                if nucleus
                    && !choice
                        .sounds
                        .iter()
                        .any(|id| catalog.get(id).is_some_and(|sound| sound.can_be_nucleus()))
                {
                    error(&mut errors, name, "nucleus_sequence_requires_syllabic_core");
                }
                for id in &choice.sounds {
                    if !inventory.contains(id) {
                        error(&mut errors, name, "sound_not_in_inventory");
                    }
                    match catalog.get(id) {
                        None => error(&mut errors, name, "unknown_sound"),
                        Some(sound) => {
                            let valid = if nucleus {
                                !sound.is_consonant() || sound.can_be_nucleus()
                            } else {
                                sound.can_be_margin()
                            };
                            if !valid {
                                error(&mut errors, name, "sound_not_valid_for_syllable_role");
                            }
                        }
                    }
                }
            }
        }
        if !self.patterns.iter().any(|p| p.weight.is_enabled()) {
            error(&mut errors, "patterns", "no_enabled_patterns");
        }
        for pattern in &self.patterns {
            // At least one word context must be able to realize the complete pattern.
            if ![(true, true), (true, false), (false, true), (false, false)]
                .iter()
                .any(|&(first, last)| {
                    self.has_choice(&self.onset, pattern.pattern.onset_size(), first, false)
                        && self.has_choice(
                            &self.nucleus,
                            pattern.pattern.nucleus_size(),
                            first && pattern.pattern.onset_size() == 0,
                            last && pattern.pattern.coda_size() == 0,
                        )
                        && self.has_choice(&self.coda, pattern.pattern.coda_size(), false, last)
                })
            {
                error(
                    &mut errors,
                    "patterns",
                    "pattern_has_no_realizable_word_context",
                );
            }
            for (name, count, rules) in [
                ("onset", pattern.pattern.onset_size(), &self.onset),
                ("nucleus", pattern.pattern.nucleus_size(), &self.nucleus),
                ("coda", pattern.pattern.coda_size(), &self.coda),
            ] {
                if (rules.presence == Presence::Forbidden && count != 0)
                    || (rules.presence == Presence::Required && count == 0)
                {
                    error(
                        &mut errors,
                        name,
                        "pattern_conflicts_with_position_presence",
                    );
                }
                if count > 0
                    && !rules
                        .choices
                        .iter()
                        .any(|c| c.sounds.len() == count && c.weight.is_enabled())
                {
                    error(&mut errors, name, "pattern_has_no_matching_sequence");
                }
            }
        }
        for pair in &self.forbidden_boundaries {
            if !inventory.contains(&pair.left) || !inventory.contains(&pair.right) {
                error(&mut errors, "boundaries", "boundary_sound_not_in_inventory");
            }
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    fn has_choice(&self, rules: &PositionRules, count: usize, initial: bool, final_: bool) -> bool {
        if count == 0 {
            return rules.presence != Presence::Required;
        }
        rules.presence != Presence::Forbidden
            && rules.choices.iter().any(|c| {
                c.sounds.len() == count
                    && c.weight.is_enabled()
                    && match c.position {
                        WordPosition::Any => true,
                        WordPosition::InitialOnly => initial,
                        WordPosition::FinalOnly => final_,
                    }
            })
    }

    pub(crate) fn check_word(&self, syllables: &[Syllable]) -> Result<(), ValidationErrors> {
        let mut errors = ValidationErrors::new();
        if syllables.is_empty() {
            error(&mut errors, "word", "empty_word");
        }
        for (index, syllable) in syllables.iter().enumerate() {
            if !self.patterns.iter().any(|p| {
                p.weight.is_enabled()
                    && p.pattern.matches(
                        syllable.onset.len(),
                        syllable.nucleus.len(),
                        syllable.coda.len(),
                    )
            }) {
                error(&mut errors, "syllables", "syllable_does_not_match_pattern");
            }
            let first = index == 0;
            let last = index + 1 == syllables.len();
            for (name, rules, sounds, initial, final_) in [
                ("onset", &self.onset, &syllable.onset, first, false),
                (
                    "nucleus",
                    &self.nucleus,
                    &syllable.nucleus,
                    first && syllable.onset.is_empty(),
                    last && syllable.coda.is_empty(),
                ),
                ("coda", &self.coda, &syllable.coda, false, last),
            ] {
                if sounds.is_empty() {
                    if rules.presence == Presence::Required {
                        error(&mut errors, name, "required_position_is_empty");
                    }
                    continue;
                }
                let accepted = rules.presence != Presence::Forbidden
                    && rules.choices.iter().any(|choice| {
                        choice.weight.is_enabled()
                            && choice
                                .sounds
                                .iter()
                                .eq(sounds.iter().map(|segment| &segment.sound))
                            && match choice.position {
                                WordPosition::Any => true,
                                WordPosition::InitialOnly => initial,
                                WordPosition::FinalOnly => final_,
                            }
                    });
                if !accepted {
                    error(&mut errors, name, "sequence_not_permitted_here");
                }
            }
        }
        for pair in syllables.windows(2) {
            if let (Some(left), Some(right)) =
                (pair[0].sounds().next_back(), pair[1].sounds().next())
            {
                if self
                    .forbidden_boundaries
                    .iter()
                    .any(|p| p.left == *left && p.right == *right)
                {
                    error(&mut errors, "boundaries", "forbidden_syllable_boundary");
                }
            }
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}
