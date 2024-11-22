use crate::models;
use serde::{Deserialize, Serialize};

/// Effects : Basic feature containing effect properties.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Effects {
    #[serde(rename = "effect", skip_serializing_if = "Option::is_none")]
    pub effect: Option<models::SupportedEffects>,
}

impl Effects {
    /// Basic feature containing effect properties.
    pub fn new() -> Effects {
        Effects { effect: None }
    }
}
