use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TemperaturePut {
    /// Type of the supported resources (always `temperature` here)
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,
    /// true when sensor is activated, false when deactivated
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

impl TemperaturePut {
    pub fn new() -> TemperaturePut {
        TemperaturePut {
            r#type: None,
            enabled: None,
        }
    }
}
/// Type of the supported resources (always `temperature` here)
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "temperature")]
    Temperature,
}

impl Default for Type {
    fn default() -> Type {
        Self::Temperature
    }
}
