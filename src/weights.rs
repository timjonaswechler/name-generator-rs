use serde::{Deserialize, Serialize};

/// Relative selection weight. Zero disables a choice; invalid values are errors.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "f64", into = "f64")]
pub struct Weight(f64);
impl Weight {
    pub const ONE: Self = Self(1.0);
    pub fn value(self) -> f64 {
        self.0
    }
    pub fn is_enabled(self) -> bool {
        self.0 > 0.0
    }
}
impl TryFrom<f64> for Weight {
    type Error = &'static str;
    fn try_from(value: f64) -> Result<Self, Self::Error> {
        if value.is_finite() && value >= 0.0 {
            Ok(Self(value))
        } else {
            Err("weights must be finite and nonnegative")
        }
    }
}
impl From<Weight> for f64 {
    fn from(value: Weight) -> Self {
        value.0
    }
}
impl Default for Weight {
    fn default() -> Self {
        Self::ONE
    }
}
