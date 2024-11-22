use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Error {
    /// a human-readable explanation specific to this occurrence of the problem.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl Error {
    pub fn new() -> Error {
        Error { description: None }
    }
}
