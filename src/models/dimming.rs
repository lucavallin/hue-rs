use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Dimming {
    /// Brightness percentage. value cannot be 0, writing 0 changes it to lowest possible brightness
    #[serde(rename = "brightness", skip_serializing_if = "Option::is_none")]
    pub brightness: Option<f64>,
}

impl Dimming {
    pub fn new() -> Dimming {
        Dimming { brightness: None }
    }
}
