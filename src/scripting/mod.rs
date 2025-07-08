//! Scripting module for dynamic rule execution
//!
//! This module provides Lua scripting capabilities for harmony rules,
//! allowing users to define custom linguistic rules without modifying Rust code.

pub mod lua_engine;

pub use lua_engine::{LuaRuleEngine, RuleContext};