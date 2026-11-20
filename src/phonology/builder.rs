use super::{SoundCatalog, SoundId};
use crate::anatomy::production::{PossibleSoundSet, ProductionAssessment};
use crate::validation::{ValidationError, ValidationErrors};
use serde::Serialize;
use std::collections::BTreeSet;

/// A language's selection, never the anatomy's entire possible set by default.
#[derive(Debug, Clone, Serialize)]
pub struct LanguageInventory {
    sounds: BTreeSet<SoundId>,
}
impl LanguageInventory {
    pub fn new(
        ids: impl IntoIterator<Item = SoundId>,
        catalog: &SoundCatalog,
        possible: &PossibleSoundSet,
    ) -> Result<Self, ValidationErrors> {
        let mut sounds = BTreeSet::new();
        let mut errors = ValidationErrors::new();
        if !possible.matches_catalog(catalog) {
            errors.add(
                "catalog",
                ValidationError::new("possible_set_catalog_mismatch"),
            );
        }
        for id in ids {
            if !sounds.insert(id.clone()) {
                errors.add(
                    id.to_string(),
                    ValidationError::new("duplicate_inventory_sound"),
                );
            }
            if catalog.get(&id).is_none() {
                errors.add(id.to_string(), ValidationError::new("unknown_sound"));
            } else {
                match possible.assessment(&id) {
                    Some(ProductionAssessment::Possible) => {}
                    Some(ProductionAssessment::Impossible(reasons)) => errors.add(
                        id.to_string(),
                        ValidationError::new("anatomically_impossible")
                            .with_message(format!("{reasons:?}")),
                    ),
                    _ => errors.add(
                        id.to_string(),
                        ValidationError::new("unsupported_production_requirements"),
                    ),
                }
            }
        }
        if sounds.is_empty() {
            errors.add("inventory", ValidationError::new("empty_inventory"));
        }
        if errors.is_empty() {
            Ok(Self { sounds })
        } else {
            Err(errors)
        }
    }
    pub fn contains(&self, id: &SoundId) -> bool {
        self.sounds.contains(id)
    }
    pub fn iter(&self) -> impl Iterator<Item = &SoundId> {
        self.sounds.iter()
    }
    pub fn len(&self) -> usize {
        self.sounds.len()
    }
    pub fn is_empty(&self) -> bool {
        self.sounds.is_empty()
    }
}
