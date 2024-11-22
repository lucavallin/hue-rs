use crate::models;
use serde::{Deserialize, Serialize};

/// LightGetAllOfColorGamut : Color gamut of color bulb. Some bulbs do not properly return the Gamut information. In this case this is not present.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct LightGetAllOfColorGamut {
    #[serde(rename = "red", skip_serializing_if = "Option::is_none")]
    pub red: Option<Box<models::GamutPosition>>,
    #[serde(rename = "green", skip_serializing_if = "Option::is_none")]
    pub green: Option<Box<models::GamutPosition>>,
    #[serde(rename = "blue", skip_serializing_if = "Option::is_none")]
    pub blue: Option<Box<models::GamutPosition>>,
}

impl LightGetAllOfColorGamut {
    /// Color gamut of color bulb. Some bulbs do not properly return the Gamut information. In this case this is not present.
    pub fn new() -> LightGetAllOfColorGamut {
        LightGetAllOfColorGamut {
            red: None,
            green: None,
            blue: None,
        }
    }
}
