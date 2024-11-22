use crate::models;
use serde::{Deserialize, Serialize};

/// ActionGetAllOfAction : The action to be executed on recall
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActionGetAllOfAction {
    #[serde(rename = "on", skip_serializing_if = "Option::is_none")]
    pub on: Option<Box<models::On>>,
    #[serde(rename = "dimming", skip_serializing_if = "Option::is_none")]
    pub dimming: Option<Box<models::Dimming>>,
    #[serde(rename = "color", skip_serializing_if = "Option::is_none")]
    pub color: Option<Box<models::Color>>,
    #[serde(rename = "color_temperature", skip_serializing_if = "Option::is_none")]
    pub color_temperature: Option<Box<models::ColorTemperature>>,
    #[serde(rename = "gradient", skip_serializing_if = "Option::is_none")]
    pub gradient: Option<Box<models::Gradient>>,
    #[serde(rename = "effects", skip_serializing_if = "Option::is_none")]
    pub effects: Option<Box<models::ActionGetAllOfActionEffects>>,
}

impl ActionGetAllOfAction {
    /// The action to be executed on recall
    pub fn new() -> ActionGetAllOfAction {
        ActionGetAllOfAction {
            on: None,
            dimming: None,
            color: None,
            color_temperature: None,
            gradient: None,
            effects: None,
        }
    }
}
