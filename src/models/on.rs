use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct On {
    /// On/Off state of the light on=true, off=false
    #[serde(rename = "on", skip_serializing_if = "Option::is_none")]
    pub on: Option<bool>,
}

impl On {
    pub fn new() -> On {
        On { on: None }
    }
}
