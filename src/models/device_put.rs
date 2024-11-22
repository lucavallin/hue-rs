use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DevicePut {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Box<models::DeviceGetAllOfMetadata>>,
    #[serde(rename = "identify", skip_serializing_if = "Option::is_none")]
    pub identify: Option<Box<models::DevicePutIdentify>>,
    #[serde(rename = "usertest", skip_serializing_if = "Option::is_none")]
    pub usertest: Option<Box<models::DevicePutUsertest>>,
}

impl DevicePut {
    pub fn new() -> DevicePut {
        DevicePut {
            r#type: None,
            metadata: None,
            identify: None,
            usertest: None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "device")]
    Device,
}

impl Default for Type {
    fn default() -> Type {
        Self::Device
    }
}
