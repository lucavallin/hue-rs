use crate::models;
use serde::{Deserialize, Serialize};

/// LightGetAllOfEffects : Basic feature containing effect properties.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct LightGetAllOfEffects {
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<models::SupportedEffects>,
    /// Possible status values in which a light could be when playing an effect.
    #[serde(rename = "status_values", skip_serializing_if = "Option::is_none")]
    pub status_values: Option<Vec<models::SupportedEffects>>,
    #[serde(rename = "effect", skip_serializing_if = "Option::is_none")]
    pub effect: Option<models::SupportedEffects>,
    /// Possible status values in which a light could be when playing an effect.
    #[serde(rename = "effect_values", skip_serializing_if = "Option::is_none")]
    pub effect_values: Option<Vec<models::SupportedEffects>>,
}

impl LightGetAllOfEffects {
    /// Basic feature containing effect properties.
    pub fn new() -> LightGetAllOfEffects {
        LightGetAllOfEffects {
            status: None,
            status_values: None,
            effect: None,
            effect_values: None,
        }
    }
}
