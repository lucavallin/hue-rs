use crate::models;
use serde::{Deserialize, Serialize};

/// Signaling : Feature containing basic signaling properties.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Signaling {
    /// - `no_signal`: No signal is active. Write “no_signal” to stop active signal. - `on_off`: Toggles between max brightness and Off in fixed color. - `on_off_color`: Toggles between off and max brightness with color provided. - `alternating`: Alternates between 2 provided colors.
    #[serde(rename = "signal", skip_serializing_if = "Option::is_none")]
    pub signal: Option<Signal>,
    /// Duration has a max of 65534000 ms and a stepsize of 1 second. Values inbetween steps will be rounded. Duration is ignored for `no_signal`.
    #[serde(rename = "duration", skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
    /// List of colors to apply to the signal (not supported by all signals)
    #[serde(rename = "color", skip_serializing_if = "Option::is_none")]
    pub color: Option<Vec<models::Color>>,
}

impl Signaling {
    /// Feature containing basic signaling properties.
    pub fn new() -> Signaling {
        Signaling {
            signal: None,
            duration: None,
            color: None,
        }
    }
}
/// - `no_signal`: No signal is active. Write “no_signal” to stop active signal. - `on_off`: Toggles between max brightness and Off in fixed color. - `on_off_color`: Toggles between off and max brightness with color provided. - `alternating`: Alternates between 2 provided colors.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Signal {
    #[serde(rename = "no_signal")]
    NoSignal,
    #[serde(rename = "on_off")]
    OnOff,
    #[serde(rename = "on_off_color")]
    OnOffColor,
    #[serde(rename = "alternating")]
    Alternating,
}

impl Default for Signal {
    fn default() -> Signal {
        Self::NoSignal
    }
}
