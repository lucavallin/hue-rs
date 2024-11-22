use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SceneGetAllOfStatus {
    #[serde(rename = "active", skip_serializing_if = "Option::is_none")]
    pub active: Option<Active>,
}

impl SceneGetAllOfStatus {
    pub fn new() -> SceneGetAllOfStatus {
        SceneGetAllOfStatus { active: None }
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Active {
    #[serde(rename = "inactive")]
    Inactive,
    #[serde(rename = "static")]
    Static,
    #[serde(rename = "dynamic_palette")]
    DynamicPalette,
}

impl Default for Active {
    fn default() -> Active {
        Self::Inactive
    }
}
