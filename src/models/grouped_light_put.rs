use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupedLightPut {
    /// Type of the supported resources (always `grouped_light` here)
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,
    #[serde(rename = "on", skip_serializing_if = "Option::is_none")]
    pub on: Option<Box<models::On>>,
    #[serde(rename = "dimming", skip_serializing_if = "Option::is_none")]
    pub dimming: Option<Box<models::Dimming>>,
    #[serde(rename = "dimming_delta", skip_serializing_if = "Option::is_none")]
    pub dimming_delta: Option<Box<models::DimmingDelta>>,
    #[serde(rename = "color_temperature", skip_serializing_if = "Option::is_none")]
    pub color_temperature: Option<Box<models::ColorTemperature>>,
    #[serde(
        rename = "color_temperature_delta",
        skip_serializing_if = "Option::is_none"
    )]
    pub color_temperature_delta: Option<Box<models::ColorTemperatureDelta>>,
    #[serde(rename = "color", skip_serializing_if = "Option::is_none")]
    pub color: Option<Box<models::Color>>,
    #[serde(rename = "alert", skip_serializing_if = "Option::is_none")]
    pub alert: Option<Box<models::Alert>>,
    #[serde(rename = "signaling", skip_serializing_if = "Option::is_none")]
    pub signaling: Option<Box<models::Signaling>>,
    #[serde(rename = "dynamics", skip_serializing_if = "Option::is_none")]
    pub dynamics: Option<Box<models::Dynamics>>,
}

impl GroupedLightPut {
    pub fn new() -> GroupedLightPut {
        GroupedLightPut {
            r#type: None,
            on: None,
            dimming: None,
            dimming_delta: None,
            color_temperature: None,
            color_temperature_delta: None,
            color: None,
            alert: None,
            signaling: None,
            dynamics: None,
        }
    }
}
/// Type of the supported resources (always `grouped_light` here)
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "grouped_light")]
    GroupedLight,
}

impl Default for Type {
    fn default() -> Type {
        Self::GroupedLight
    }
}
