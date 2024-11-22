use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BridgeGetAllOfTimeZone {
    /// Time zone where the user's home is located (as Olson ID).
    #[serde(rename = "time_zone", skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
}

impl BridgeGetAllOfTimeZone {
    pub fn new() -> BridgeGetAllOfTimeZone {
        BridgeGetAllOfTimeZone { time_zone: None }
    }
}
