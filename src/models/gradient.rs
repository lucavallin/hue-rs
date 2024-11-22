use crate::models;
use serde::{Deserialize, Serialize};

/// Gradient : Basic feature containing gradient properties.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Gradient {
    /// Collection of gradients points. For control of the gradient points through a PUT a minimum of 2 points need to be provided.
    #[serde(rename = "points", skip_serializing_if = "Option::is_none")]
    pub points: Option<Vec<models::Color>>,
    #[serde(rename = "mode", skip_serializing_if = "Option::is_none")]
    pub mode: Option<models::SupportedGradientMode>,
}

impl Gradient {
    /// Basic feature containing gradient properties.
    pub fn new() -> Gradient {
        Gradient {
            points: None,
            mode: None,
        }
    }
}
