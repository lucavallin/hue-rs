use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct LightGetAllOfColorTemperature {
    /// color temperature in mirek or null when the light color is not in the ct spectrum
    #[serde(rename = "mirek", skip_serializing_if = "Option::is_none")]
    pub mirek: Option<i32>,
    /// Indication whether the value presented in mirek is valid
    #[serde(rename = "mirek_valid", skip_serializing_if = "Option::is_none")]
    pub mirek_valid: Option<bool>,
    #[serde(rename = "mirek_schema", skip_serializing_if = "Option::is_none")]
    pub mirek_schema: Option<Box<models::LightGetAllOfColorTemperatureMirekSchema>>,
}

impl LightGetAllOfColorTemperature {
    pub fn new() -> LightGetAllOfColorTemperature {
        LightGetAllOfColorTemperature {
            mirek: None,
            mirek_valid: None,
            mirek_schema: None,
        }
    }
}
