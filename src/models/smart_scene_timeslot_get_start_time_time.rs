use crate::models;
use serde::{Deserialize, Serialize};

/// SmartSceneTimeslotGetStartTimeTime : this property is only used when property “kind” is “time”
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SmartSceneTimeslotGetStartTimeTime {
    #[serde(rename = "hour", skip_serializing_if = "Option::is_none")]
    pub hour: Option<i32>,
    #[serde(rename = "minute", skip_serializing_if = "Option::is_none")]
    pub minute: Option<i32>,
    #[serde(rename = "second", skip_serializing_if = "Option::is_none")]
    pub second: Option<i32>,
}

impl SmartSceneTimeslotGetStartTimeTime {
    /// this property is only used when property “kind” is “time”
    pub fn new() -> SmartSceneTimeslotGetStartTimeTime {
        SmartSceneTimeslotGetStartTimeTime {
            hour: None,
            minute: None,
            second: None,
        }
    }
}
