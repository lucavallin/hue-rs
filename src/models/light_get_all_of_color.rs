use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct LightGetAllOfColor {
    #[serde(rename = "xy", skip_serializing_if = "Option::is_none")]
    pub xy: Option<Box<models::GamutPosition>>,
    #[serde(rename = "gamut", skip_serializing_if = "Option::is_none")]
    pub gamut: Option<Box<models::LightGetAllOfColorGamut>>,
    /// The gammut types supported by hue – A Gamut of early Philips color-only products – B Limited gamut of first Hue color products – C Richer color gamut of Hue white and color ambiance products – other Color gamut of non-hue products with non-hue gamuts resp w/o gamut
    #[serde(rename = "gamut_type", skip_serializing_if = "Option::is_none")]
    pub gamut_type: Option<GamutType>,
}

impl LightGetAllOfColor {
    pub fn new() -> LightGetAllOfColor {
        LightGetAllOfColor {
            xy: None,
            gamut: None,
            gamut_type: None,
        }
    }
}
/// The gammut types supported by hue – A Gamut of early Philips color-only products – B Limited gamut of first Hue color products – C Richer color gamut of Hue white and color ambiance products – other Color gamut of non-hue products with non-hue gamuts resp w/o gamut
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum GamutType {
    #[serde(rename = "A")]
    A,
    #[serde(rename = "B")]
    B,
    #[serde(rename = "C")]
    C,
    #[serde(rename = "other")]
    Other,
}

impl Default for GamutType {
    fn default() -> GamutType {
        Self::A
    }
}
