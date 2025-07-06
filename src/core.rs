//! Core traits and types for the name generator system.
//!
//! This module provides the fundamental building blocks for the language profile-based name generation system.

use crate::language_profile::profile::LanguageProfile;
use crate::generators::profile_generator::LanguageProfileGenerator;
use rand::Rng;

/// Language profile category trait
///
/// This trait defines the interface for language profile-based name generation,
/// providing phonetically-grounded name generation capabilities.
pub trait LanguageProfileCategory: Default {
    /// Get the language profile for this category
    fn language_profile(&self) -> &LanguageProfile;
    
    /// Generate a name using the language profile
    fn generate_with_profile(&self, rng: &mut impl Rng) -> String {
        let profile = self.language_profile();
        let generator = LanguageProfileGenerator::new(profile);
        generator.generate(rng)
    }
}

/// Main name type that works with language profile categories
///
/// This is the core type that provides a consistent API for name generation
/// using language profiles.
///
/// # Examples
///
/// ```rust
/// use name_generator::core::Name;
/// use name_generator::categories::profile_examples::GermanLanguageProfile;
/// use rand::thread_rng;
///
/// let mut rng = thread_rng();
/// let generator = Name::<GermanLanguageProfile>::new();
/// let name = generator.generate(&mut rng);
/// ```
#[derive(Debug, Clone)]
pub struct Name<T: LanguageProfileCategory> {
    category: T,
}

impl<T: LanguageProfileCategory> Name<T> {
    /// Create a new name generator for a specific category
    pub fn new() -> Self {
        Self {
            category: T::default(),
        }
    }

    /// Create a new name generator with a specific category instance
    pub fn with_category(category: T) -> Self {
        Self { category }
    }

    /// Generate a name using this category's language profile
    pub fn generate(&self, rng: &mut impl Rng) -> String {
        self.category.generate_with_profile(rng)
    }

    /// Get the category this name generator uses
    pub fn category(&self) -> &T {
        &self.category
    }
}

impl<T: LanguageProfileCategory> Default for Name<T> {
    fn default() -> Self {
        Self::new()
    }
}
