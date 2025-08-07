pub mod clicks;
pub mod ejectives;
pub mod implosives;
pub mod pulmonics;
// pub mod co_articulated;
//
use super::consonants::{
    clicks::ALL_CLICKS, ejectives::ALL_EJECTIVES, implosives::ALL_IMPLOSIVES,
    pulmonics::ALL_PULMONICS,
};
pub fn all_consonants() -> Vec<&'static str> {
    [ALL_PULMONICS, ALL_CLICKS, ALL_IMPLOSIVES, ALL_EJECTIVES].concat()
}
