use crate::models;
use serde::{Deserialize, Serialize};

/// DeviceGet : Definition of a device resource
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceGet {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,
    /// Unique identifier representing a specific resource instance
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Clip v1 resource identifier
    #[serde(rename = "id_v1", skip_serializing_if = "Option::is_none")]
    pub id_v1: Option<String>,
    #[serde(rename = "owner", skip_serializing_if = "Option::is_none")]
    pub owner: Option<Box<models::ResourceIdentifier>>,
    #[serde(rename = "product_data", skip_serializing_if = "Option::is_none")]
    pub product_data: Option<Box<models::ProductData>>,
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Box<models::DeviceGetAllOfMetadata>>,
    #[serde(rename = "usertest", skip_serializing_if = "Option::is_none")]
    pub usertest: Option<Box<models::DeviceGetAllOfUsertest>>,
    /// References all services providing control and state of the device.
    #[serde(rename = "services", skip_serializing_if = "Option::is_none")]
    pub services: Option<Vec<models::ResourceIdentifier>>,
}

impl DeviceGet {
    /// Definition of a device resource
    pub fn new() -> DeviceGet {
        DeviceGet {
            r#type: None,
            id: None,
            id_v1: None,
            owner: None,
            product_data: None,
            metadata: None,
            usertest: None,
            services: None,
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
