use crate::models;
use serde::{Deserialize, Serialize};

/// LightPutTimedEffects : Basic feature containing timed effect properties.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct LightPutTimedEffects {
    #[serde(rename = "effect", skip_serializing_if = "Option::is_none")]
    pub effect: Option<models::SupportedTimedEffects>,
    /// Duration is mandatory when timed effect is set except for no_effect. Resolution decreases for a larger duration. e.g Effects with duration smaller than a minute will be rounded to a resolution of 1s, while effects with duration larger than an hour will be arounded up to a resolution of 300s. Duration has a max of 21600000 ms.
    #[serde(rename = "duration", skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
}

impl LightPutTimedEffects {
    /// Basic feature containing timed effect properties.
    pub fn new() -> LightPutTimedEffects {
        LightPutTimedEffects {
            effect: None,
            duration: None,
        }
    }
}
