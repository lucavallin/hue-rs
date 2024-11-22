use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ColorTemperaturePalettePost {
    #[serde(rename = "color_temperature", skip_serializing_if = "Option::is_none")]
    pub color_temperature: Option<Box<models::ColorTemperaturePalettePostColorTemperature>>,
    #[serde(rename = "dimming", skip_serializing_if = "Option::is_none")]
    pub dimming: Option<Box<models::Dimming>>,
}

impl ColorTemperaturePalettePost {
    pub fn new() -> ColorTemperaturePalettePost {
        ColorTemperaturePalettePost {
            color_temperature: None,
            dimming: None,
        }
    }
}
