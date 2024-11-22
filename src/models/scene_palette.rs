use crate::models;
use serde::{Deserialize, Serialize};

/// ScenePalette : Group of colors that describe the palette of colors to be used when playing dynamics
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScenePalette {
    #[serde(rename = "color", skip_serializing_if = "Option::is_none")]
    pub color: Option<Vec<models::ColorPaletteGet>>,
    #[serde(rename = "dimming", skip_serializing_if = "Option::is_none")]
    pub dimming: Option<Vec<models::Dimming>>,
    #[serde(rename = "color_temperature", skip_serializing_if = "Option::is_none")]
    pub color_temperature: Option<Vec<models::ColorTemperaturePalettePost>>,
    #[serde(rename = "effects", skip_serializing_if = "Option::is_none")]
    pub effects: Option<Vec<models::ScenePaletteEffectsInner>>,
}

impl ScenePalette {
    /// Group of colors that describe the palette of colors to be used when playing dynamics
    pub fn new() -> ScenePalette {
        ScenePalette {
            color: None,
            dimming: None,
            color_temperature: None,
            effects: None,
        }
    }
}
