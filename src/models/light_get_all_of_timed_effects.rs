use crate::models;
use serde::{Deserialize, Serialize};

/// LightGetAllOfTimedEffects : Basic feature containing timed effect properties.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct LightGetAllOfTimedEffects {
    #[serde(rename = "effect", skip_serializing_if = "Option::is_none")]
    pub effect: Option<models::SupportedTimedEffects>,
    /// Possible timed effect values you can set in a light
    #[serde(rename = "effect_values", skip_serializing_if = "Option::is_none")]
    pub effect_values: Option<Vec<models::SupportedTimedEffects>>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<models::SupportedTimedEffects>,
    /// Possible status values in which a light could be when playing a timed effect.
    #[serde(rename = "status_values", skip_serializing_if = "Option::is_none")]
    pub status_values: Option<Vec<models::SupportedTimedEffects>>,
    /// Duration is mandatory when timed effect is set except for no_effect. Resolution decreases for a larger duration. e.g Effects with duration smaller than a minute will be rounded to a resolution of 1s, while effects with duration larger than an hour will be arounded up to a resolution of 300s. Duration has a max of 21600000 ms.
    #[serde(rename = "duration", skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
}

impl LightGetAllOfTimedEffects {
    /// Basic feature containing timed effect properties.
    pub fn new() -> LightGetAllOfTimedEffects {
        LightGetAllOfTimedEffects {
            effect: None,
            effect_values: None,
            status: None,
            status_values: None,
            duration: None,
        }
    }
}
