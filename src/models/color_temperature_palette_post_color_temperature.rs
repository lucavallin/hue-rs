use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ColorTemperaturePalettePostColorTemperature {
    /// color temperature in mirek or null when the light color is not in the ct spectrum
    #[serde(rename = "mirek", skip_serializing_if = "Option::is_none")]
    pub mirek: Option<i32>,
}

impl ColorTemperaturePalettePostColorTemperature {
    pub fn new() -> ColorTemperaturePalettePostColorTemperature {
        ColorTemperaturePalettePostColorTemperature { mirek: None }
    }
}
