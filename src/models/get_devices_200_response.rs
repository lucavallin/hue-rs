use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetDevices200Response {
    #[serde(rename = "errors", skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<models::Error>>,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<models::DeviceGet>>,
}

impl GetDevices200Response {
    pub fn new() -> GetDevices200Response {
        GetDevices200Response {
            errors: None,
            data: None,
        }
    }
}
