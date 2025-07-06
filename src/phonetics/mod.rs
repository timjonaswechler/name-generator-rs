//! Phonetics module for IPA-based sound representation
//!
//! This module provides phonetic representation using IPA (International Phonetic Alphabet)
//! and phoneme-based sound processing for the LanguageProfile system.

pub mod phoneme;
pub mod ipa;

pub use phoneme::*;
pub use ipa::*;