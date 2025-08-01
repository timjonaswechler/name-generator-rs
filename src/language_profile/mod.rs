//! Language Profile system for phonetically-grounded name generation
//!
//! This module provides the LanguageProfile system that enables phonetically-grounded
//! language profiles while maintaining backward compatibility with the pattern-based approach.

pub mod profile;
pub mod loader;
pub mod lua_profile;
pub mod lua_loader;

pub use profile::*;
pub use lua_profile::*;
pub use lua_loader::*;