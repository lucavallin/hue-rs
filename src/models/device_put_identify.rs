use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DevicePutIdentify {
    /// Triggers a visual identification sequence, current implemented as (which can change in the future): Bridge performs Zigbee LED identification cycles for 5 seconds Lights perform one breathe cycle Sensors perform LED identification cycles for 15 seconds
    #[serde(rename = "action", skip_serializing_if = "Option::is_none")]
    pub action: Option<Action>,
}

impl DevicePutIdentify {
    pub fn new() -> DevicePutIdentify {
        DevicePutIdentify { action: None }
    }
}
/// Triggers a visual identification sequence, current implemented as (which can change in the future): Bridge performs Zigbee LED identification cycles for 5 seconds Lights perform one breathe cycle Sensors perform LED identification cycles for 15 seconds
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Action {
    #[serde(rename = "identify")]
    Identify,
}

impl Default for Action {
    fn default() -> Action {
        Self::Identify
    }
}
