use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponse {
    #[serde(rename = "errors", skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<models::Error>>,
}

impl ErrorResponse {
    pub fn new() -> ErrorResponse {
        ErrorResponse { errors: None }
    }
}
