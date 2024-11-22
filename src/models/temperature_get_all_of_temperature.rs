use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TemperatureGetAllOfTemperature {
    /// Deprecated. Moved to Temperature_report/temperature
    #[serde(rename = "temperature", skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    /// Deprecated. Indication whether the value presented in temperature is valid
    #[serde(rename = "temperature_valid", skip_serializing_if = "Option::is_none")]
    pub temperature_valid: Option<bool>,
    #[serde(rename = "temperature_report", skip_serializing_if = "Option::is_none")]
    pub temperature_report: Option<Box<models::TemperatureGetAllOfTemperatureTemperatureReport>>,
}

impl TemperatureGetAllOfTemperature {
    pub fn new() -> TemperatureGetAllOfTemperature {
        TemperatureGetAllOfTemperature {
            temperature: None,
            temperature_valid: None,
            temperature_report: None,
        }
    }
}
