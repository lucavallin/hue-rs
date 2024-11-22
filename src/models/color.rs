use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Color {
    #[serde(rename = "xy", skip_serializing_if = "Option::is_none")]
    pub xy: Option<Box<models::GamutPosition>>,
}

impl Color {
    pub fn new() -> Color {
        Color { xy: None }
    }
}
