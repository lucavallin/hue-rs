use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BridgePut {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,
}

impl BridgePut {
    pub fn new() -> BridgePut {
        BridgePut { r#type: None }
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "bridge")]
    Bridge,
}

impl Default for Type {
    fn default() -> Type {
        Self::Bridge
    }
}
