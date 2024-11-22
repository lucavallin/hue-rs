use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScenePaletteEffectsInner {
    #[serde(rename = "effect", skip_serializing_if = "Option::is_none")]
    pub effect: Option<models::SupportedEffects>,
}

impl ScenePaletteEffectsInner {
    pub fn new() -> ScenePaletteEffectsInner {
        ScenePaletteEffectsInner { effect: None }
    }
}
