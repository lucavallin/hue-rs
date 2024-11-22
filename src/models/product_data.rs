use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProductData {
    /// Unique identification of device model
    #[serde(rename = "model_id", skip_serializing_if = "Option::is_none")]
    pub model_id: Option<String>,
    /// Name of device manufacturer
    #[serde(rename = "manufacturer_name", skip_serializing_if = "Option::is_none")]
    pub manufacturer_name: Option<String>,
    /// Name of the product
    #[serde(rename = "product_name", skip_serializing_if = "Option::is_none")]
    pub product_name: Option<String>,
    #[serde(rename = "product_archetype", skip_serializing_if = "Option::is_none")]
    pub product_archetype: Option<models::ProductArchetype>,
    /// This device is Hue certified
    #[serde(rename = "certified", skip_serializing_if = "Option::is_none")]
    pub certified: Option<bool>,
    /// Software version of the product
    #[serde(rename = "software_version", skip_serializing_if = "Option::is_none")]
    pub software_version: Option<String>,
    /// Hardware type; identified by Manufacturer code and ImageType
    #[serde(
        rename = "hardware_platform_type",
        skip_serializing_if = "Option::is_none"
    )]
    pub hardware_platform_type: Option<String>,
}

impl ProductData {
    pub fn new() -> ProductData {
        ProductData {
            model_id: None,
            manufacturer_name: None,
            product_name: None,
            product_archetype: None,
            certified: None,
            software_version: None,
            hardware_platform_type: None,
        }
    }
}
