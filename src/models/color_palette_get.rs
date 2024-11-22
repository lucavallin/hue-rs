use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ColorPaletteGet {
    #[serde(rename = "color", skip_serializing_if = "Option::is_none")]
    pub color: Option<Box<models::Color>>,
    #[serde(rename = "dimming", skip_serializing_if = "Option::is_none")]
    pub dimming: Option<Box<models::Dimming>>,
}

impl ColorPaletteGet {
    pub fn new() -> ColorPaletteGet {
        ColorPaletteGet {
            color: None,
            dimming: None,
        }
    }
}
