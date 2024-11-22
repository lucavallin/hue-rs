use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PowerupDimming {
    /// Dimming will set the brightness to the specified value after power up. When setting mode “dimming”, the dimming property must be included. Previous will set brightness to the state it was in before powering off.
    #[serde(rename = "mode", skip_serializing_if = "Option::is_none")]
    pub mode: Option<Mode>,
    /// Brightness percentage. value cannot be 0, writing 0 changes it to lowest possible brightness
    #[serde(rename = "dimming", skip_serializing_if = "Option::is_none")]
    pub dimming: Option<f64>,
    #[serde(rename = "color", skip_serializing_if = "Option::is_none")]
    pub color: Option<Box<models::LightGetAllOfPowerupDimmingColor>>,
}

impl PowerupDimming {
    pub fn new() -> PowerupDimming {
        PowerupDimming {
            mode: None,
            dimming: None,
            color: None,
        }
    }
}
/// Dimming will set the brightness to the specified value after power up. When setting mode “dimming”, the dimming property must be included. Previous will set brightness to the state it was in before powering off.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Mode {
    #[serde(rename = "dimming")]
    Dimming,
    #[serde(rename = "previous")]
    Previous,
}

impl Default for Mode {
    fn default() -> Mode {
        Self::Dimming
    }
}
