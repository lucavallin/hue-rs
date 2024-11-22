use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DevicePowerGetAllOfPowerState {
    /// Status of the power source of a device, only for battery powered devices.  - `normal` – battery level is sufficient - `low` – battery level low, some features (e.g. software update) might stop working, please change battery soon - `critical` – battery level critical, device can fail any moment
    #[serde(rename = "battery_state", skip_serializing_if = "Option::is_none")]
    pub battery_state: Option<BatteryState>,
    /// The current battery state in percent, only for battery powered devices.
    #[serde(rename = "battery_level", skip_serializing_if = "Option::is_none")]
    pub battery_level: Option<i32>,
}

impl DevicePowerGetAllOfPowerState {
    pub fn new() -> DevicePowerGetAllOfPowerState {
        DevicePowerGetAllOfPowerState {
            battery_state: None,
            battery_level: None,
        }
    }
}
/// Status of the power source of a device, only for battery powered devices.  - `normal` – battery level is sufficient - `low` – battery level low, some features (e.g. software update) might stop working, please change battery soon - `critical` – battery level critical, device can fail any moment
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum BatteryState {
    #[serde(rename = "normal")]
    Normal,
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "critical")]
    Critical,
}

impl Default for BatteryState {
    fn default() -> BatteryState {
        Self::Normal
    }
}
