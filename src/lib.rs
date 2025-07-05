//! Name generation system for celestial objects.
//!
//! This module provides a macro-based name generation system that follows the same
//! design patterns as the physics units system. It supports type-safe name generation
//! for different categories of celestial objects.
//!
//! # Examples
//!
//! ```rust
//! use star_sim::utilities::name_generator::*;
//! use rand::thread_rng;
//!
//! let mut rng = thread_rng();
//!
//! // Generate names for different celestial object types
//! let star_name = Name::<Star>::new().generate(&mut rng);
//! let planet_name = Name::<RockyBody>::new().generate(&mut rng);
//! let gas_giant = Name::<GaseousBody>::new().generate(&mut rng);
//! let ice_body = Name::<IcyBody>::new().generate(&mut rng);
//! ```

// Public modules
pub mod categories;
pub mod phonetic_rules;
pub mod symbol_types;
pub mod core;
pub mod macros;
pub mod pattern;
pub mod symbols;
pub mod phonetics;
pub mod language_profile;
