use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceGetAllOfUsertest {
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    /// Activates or extends user usertest mode of device for 120 seconds. `false` deactivates usertest mode. In usertest mode, devices report changes in state faster and indicate state changes on device LED (if applicable)
    #[serde(rename = "usertest", skip_serializing_if = "Option::is_none")]
    pub usertest: Option<bool>,
}

impl DeviceGetAllOfUsertest {
    pub fn new() -> DeviceGetAllOfUsertest {
        DeviceGetAllOfUsertest {
            status: None,
            usertest: None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "set")]
    Set,
    #[serde(rename = "changing")]
    Changing,
}

impl Default for Status {
    fn default() -> Status {
        Self::Set
    }
}
