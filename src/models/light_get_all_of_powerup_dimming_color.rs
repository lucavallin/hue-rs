use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct LightGetAllOfPowerupDimmingColor {
    /// State to activate after powerup. Availability of “color_temperature” and “color” modes depend on the capabilities of the lamp. Colortemperature will set the colortemperature to the specified value after power up. When setting color_temperature, the color_temperature property must be included Color will set the color tot he specified value after power up. When setting color mode, the color property must be included Previous will set color to the state it was in before powering off.
    #[serde(rename = "mode", skip_serializing_if = "Option::is_none")]
    pub mode: Option<Mode>,
    #[serde(rename = "color_temperature", skip_serializing_if = "Option::is_none")]
    pub color_temperature: Option<Box<models::LightGetAllOfPowerupDimmingColorColorTemperature>>,
}

impl LightGetAllOfPowerupDimmingColor {
    pub fn new() -> LightGetAllOfPowerupDimmingColor {
        LightGetAllOfPowerupDimmingColor {
            mode: None,
            color_temperature: None,
        }
    }
}
/// State to activate after powerup. Availability of “color_temperature” and “color” modes depend on the capabilities of the lamp. Colortemperature will set the colortemperature to the specified value after power up. When setting color_temperature, the color_temperature property must be included Color will set the color tot he specified value after power up. When setting color mode, the color property must be included Previous will set color to the state it was in before powering off.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Mode {
    #[serde(rename = "color_temperature")]
    ColorTemperature,
    #[serde(rename = "color")]
    Color,
    #[serde(rename = "previous")]
    Previous,
}

impl Default for Mode {
    fn default() -> Mode {
        Self::ColorTemperature
    }
}
