use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetDevicePowers200Response {
    #[serde(rename = "errors", skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<models::Error>>,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<models::DevicePowerGet>>,
}

impl GetDevicePowers200Response {
    pub fn new() -> GetDevicePowers200Response {
        GetDevicePowers200Response {
            errors: None,
            data: None,
        }
    }
}
