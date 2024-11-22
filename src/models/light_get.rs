use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct LightGet {
    /// Type of the supported resources
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// Unique identifier representing a specific resource instance
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Clip v1 resource identifier
    #[serde(rename = "id_v1", skip_serializing_if = "Option::is_none")]
    pub id_v1: Option<String>,
    #[serde(rename = "owner", skip_serializing_if = "Option::is_none")]
    pub owner: Option<Box<models::ResourceIdentifier>>,
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Box<models::LightGetAllOfMetadata>>,
    #[serde(rename = "on", skip_serializing_if = "Option::is_none")]
    pub on: Option<Box<models::On>>,
    #[serde(rename = "dimming", skip_serializing_if = "Option::is_none")]
    pub dimming: Option<Box<models::LightGetAllOfDimming>>,
    #[serde(rename = "color_temperature", skip_serializing_if = "Option::is_none")]
    pub color_temperature: Option<Box<models::LightGetAllOfColorTemperature>>,
    #[serde(rename = "color", skip_serializing_if = "Option::is_none")]
    pub color: Option<Box<models::LightGetAllOfColor>>,
    #[serde(rename = "dynamics", skip_serializing_if = "Option::is_none")]
    pub dynamics: Option<Box<models::LightGetAllOfDynamics>>,
    /// TODO
    #[serde(rename = "alert", skip_serializing_if = "Option::is_none")]
    pub alert: Option<serde_json::Value>,
    #[serde(rename = "signaling", skip_serializing_if = "Option::is_none")]
    pub signaling: Option<Box<models::LightGetAllOfSignaling>>,
    #[serde(rename = "mode", skip_serializing_if = "Option::is_none")]
    pub mode: Option<Mode>,
    #[serde(rename = "gradient", skip_serializing_if = "Option::is_none")]
    pub gradient: Option<serde_json::Value>,
    #[serde(rename = "effects", skip_serializing_if = "Option::is_none")]
    pub effects: Option<Box<models::LightGetAllOfEffects>>,
    #[serde(rename = "timed_effects", skip_serializing_if = "Option::is_none")]
    pub timed_effects: Option<Box<models::LightGetAllOfTimedEffects>>,
    #[serde(rename = "powerup", skip_serializing_if = "Option::is_none")]
    pub powerup: Option<Box<models::LightGetAllOfPowerup>>,
}

impl LightGet {
    pub fn new() -> LightGet {
        LightGet {
            r#type: None,
            id: None,
            id_v1: None,
            owner: None,
            metadata: None,
            on: None,
            dimming: None,
            color_temperature: None,
            color: None,
            dynamics: None,
            alert: None,
            signaling: None,
            mode: None,
            gradient: None,
            effects: None,
            timed_effects: None,
            powerup: None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Mode {
    #[serde(rename = "normal")]
    Normal,
    #[serde(rename = "streaming")]
    Streaming,
}

impl Default for Mode {
    fn default() -> Mode {
        Self::Normal
    }
}
