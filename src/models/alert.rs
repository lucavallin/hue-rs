use crate::models;
use serde::{Deserialize, Serialize};

/// Alert : Joined alert control
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Alert {
    #[serde(rename = "action", skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
}

impl Alert {
    /// Joined alert control
    pub fn new() -> Alert {
        Alert { action: None }
    }
}
