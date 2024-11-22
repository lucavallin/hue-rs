use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct LightGetAllOfPowerupOn {
    /// State to activate after powerup. On will use the value specified in the “on” property. When setting mode “on”, the on property must be included. Toggle will alternate between on and off on each subsequent power toggle. Previous will return to the state it was in before powering off.
    #[serde(rename = "mode", skip_serializing_if = "Option::is_none")]
    pub mode: Option<Mode>,
    #[serde(rename = "on", skip_serializing_if = "Option::is_none")]
    pub on: Option<Box<models::On>>,
}

impl LightGetAllOfPowerupOn {
    pub fn new() -> LightGetAllOfPowerupOn {
        LightGetAllOfPowerupOn {
            mode: None,
            on: None,
        }
    }
}
/// State to activate after powerup. On will use the value specified in the “on” property. When setting mode “on”, the on property must be included. Toggle will alternate between on and off on each subsequent power toggle. Previous will return to the state it was in before powering off.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Mode {
    #[serde(rename = "on")]
    On,
    #[serde(rename = "toggle")]
    Toggle,
    #[serde(rename = "previous")]
    Previous,
}

impl Default for Mode {
    fn default() -> Mode {
        Self::On
    }
}
