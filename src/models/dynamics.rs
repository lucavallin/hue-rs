use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Dynamics {
    /// Duration of a light transition or timed effects in ms.
    #[serde(rename = "duration", skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
}

impl Dynamics {
    pub fn new() -> Dynamics {
        Dynamics { duration: None }
    }
}
