use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct LightPut {
    /// Type of the supported resources (always `light` here)
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
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
    #[serde(rename = "dynamics", skip_serializing_if = "Option::is_none")]
    pub dynamics: Option<Box<models::LightDynamics>>,
    #[serde(rename = "alert", skip_serializing_if = "Option::is_none")]
    pub alert: Option<Box<models::Alert>>,
    #[serde(rename = "signaling", skip_serializing_if = "Option::is_none")]
    pub signaling: Option<Box<models::Signaling>>,
    #[serde(rename = "mode", skip_serializing_if = "Option::is_none")]
    pub mode: Option<Mode>,
    #[serde(rename = "gradient", skip_serializing_if = "Option::is_none")]
    pub gradient: Option<Box<models::Gradient>>,
    #[serde(rename = "effects", skip_serializing_if = "Option::is_none")]
    pub effects: Option<Box<models::Effects>>,
    #[serde(rename = "timed_effects", skip_serializing_if = "Option::is_none")]
    pub timed_effects: Option<Box<models::LightPutTimedEffects>>,
    #[serde(rename = "powerup", skip_serializing_if = "Option::is_none")]
    pub powerup: Option<Box<models::Powerup>>,
}

impl LightPut {
    pub fn new() -> LightPut {
        LightPut {
            r#type: None,
            on: None,
            dimming: None,
            dimming_delta: None,
            color_temperature: None,
            color_temperature_delta: None,
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
