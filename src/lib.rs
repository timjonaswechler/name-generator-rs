//! Name generation system for language profiles.
//!
//! This module provides a language profile-based name generation system that creates
//! phonetically accurate names using sophisticated linguistic rules.
//!
//! # Examples
//!
//! ```rust
//! use name_generator::core::Name;
//! use name_generator::categories::profile_examples::GermanLanguageProfile;
//! use rand::thread_rng;
//!
//! let mut rng = thread_rng();
//! let generator = Name::<GermanLanguageProfile>::new();
//! let name = generator.generate(&mut rng);
//! ```

// Public modules
pub mod categories;
pub mod core;
pub mod phonetics;
pub mod language_profile;
pub mod generators;
pub mod scripting;
