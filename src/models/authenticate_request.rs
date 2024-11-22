use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthenticateRequest {
    #[serde(rename = "devicetype", skip_serializing_if = "Option::is_none")]
    pub devicetype: Option<String>,
    #[serde(rename = "generateclientkey", skip_serializing_if = "Option::is_none")]
    pub generateclientkey: Option<bool>,
}

impl AuthenticateRequest {
    pub fn new() -> AuthenticateRequest {
        AuthenticateRequest {
            devicetype: None,
            generateclientkey: None,
        }
    }
}
