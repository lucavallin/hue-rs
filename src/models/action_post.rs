use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActionPost {
    #[serde(rename = "target")]
    pub target: models::ResourceIdentifier,
    #[serde(rename = "action")]
    pub action: Box<models::ActionPostAction>,
}

impl ActionPost {
    pub fn new(target: models::ResourceIdentifier, action: models::ActionPostAction) -> ActionPost {
        ActionPost {
            target,
            action: Box::new(action),
        }
    }
}
