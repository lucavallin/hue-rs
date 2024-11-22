use crate::models;
use serde::{Deserialize, Serialize};

/// Powerup : Feature containing properties to configure powerup behaviour of a lightsource.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Powerup {
    /// When setting the custom preset the additional properties can be set. For all other presets, no other properties can be included.
    #[serde(rename = "preset", skip_serializing_if = "Option::is_none")]
    pub preset: Option<Preset>,
    /// Indicates if the shown values have been configured in the lightsource.
    #[serde(rename = "configured", skip_serializing_if = "Option::is_none")]
    pub configured: Option<bool>,
    #[serde(rename = "on", skip_serializing_if = "Option::is_none")]
    pub on: Option<Box<models::LightGetAllOfPowerupOn>>,
    #[serde(rename = "dimming", skip_serializing_if = "Option::is_none")]
    pub dimming: Option<Box<models::PowerupDimming>>,
}

impl Powerup {
    /// Feature containing properties to configure powerup behaviour of a lightsource.
    pub fn new() -> Powerup {
        Powerup {
            preset: None,
            configured: None,
            on: None,
            dimming: None,
        }
    }
}
/// When setting the custom preset the additional properties can be set. For all other presets, no other properties can be included.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Preset {
    #[serde(rename = "safety")]
    Safety,
    #[serde(rename = "powerfail")]
    Powerfail,
    #[serde(rename = "last_on_state")]
    LastOnState,
    #[serde(rename = "custom")]
    Custom,
}

impl Default for Preset {
    fn default() -> Preset {
        Self::Safety
    }
}
