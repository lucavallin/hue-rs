use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TemperatureGetAllOfTemperatureTemperatureReport {
    /// last time the value of this property is changed.
    #[serde(rename = "changed", skip_serializing_if = "Option::is_none")]
    pub changed: Option<String>,
    /// Temperature in 1.00 degrees Celsius
    #[serde(rename = "temperature", skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
}

impl TemperatureGetAllOfTemperatureTemperatureReport {
    pub fn new() -> TemperatureGetAllOfTemperatureTemperatureReport {
        TemperatureGetAllOfTemperatureTemperatureReport {
            changed: None,
            temperature: None,
        }
    }
}
