use crate::models;
use serde::{Deserialize, Serialize};

/// ActionGetAllOfActionEffects : Basic feature containing effect properties.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActionGetAllOfActionEffects {
    #[serde(rename = "effect", skip_serializing_if = "Option::is_none")]
    pub effect: Option<models::SupportedEffects>,
}

impl ActionGetAllOfActionEffects {
    /// Basic feature containing effect properties.
    pub fn new() -> ActionGetAllOfActionEffects {
        ActionGetAllOfActionEffects { effect: None }
    }
}
