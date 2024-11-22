use crate::models;
use serde::{Deserialize, Serialize};

/// GamutPosition : CIE XY gamut position
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GamutPosition {
    /// X position in color gamut
    #[serde(rename = "x", skip_serializing_if = "Option::is_none")]
    pub x: Option<f64>,
    /// y position in color gamut
    #[serde(rename = "y", skip_serializing_if = "Option::is_none")]
    pub y: Option<f64>,
}

impl GamutPosition {
    /// CIE XY gamut position
    pub fn new() -> GamutPosition {
        GamutPosition { x: None, y: None }
    }
}
