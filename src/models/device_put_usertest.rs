use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DevicePutUsertest {
    /// Activates or extends user usertest mode of device for 120 seconds. `false` deactivates usertest mode. In usertest mode, devices report changes in state faster and indicate state changes on device LED (if applicable)
    #[serde(rename = "usertest", skip_serializing_if = "Option::is_none")]
    pub usertest: Option<bool>,
}

impl DevicePutUsertest {
    pub fn new() -> DevicePutUsertest {
        DevicePutUsertest { usertest: None }
    }
}
