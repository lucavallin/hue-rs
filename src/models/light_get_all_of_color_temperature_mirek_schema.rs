use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct LightGetAllOfColorTemperatureMirekSchema {
    /// minimum color temperature this light supports
    #[serde(rename = "mirek_minimum", skip_serializing_if = "Option::is_none")]
    pub mirek_minimum: Option<i32>,
    /// maximum color temperature this light supports
    #[serde(rename = "mirek_maximum", skip_serializing_if = "Option::is_none")]
    pub mirek_maximum: Option<i32>,
}

impl LightGetAllOfColorTemperatureMirekSchema {
    pub fn new() -> LightGetAllOfColorTemperatureMirekSchema {
        LightGetAllOfColorTemperatureMirekSchema {
            mirek_minimum: None,
            mirek_maximum: None,
        }
    }
}
