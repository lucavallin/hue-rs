use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SmartSceneTimeslotGetStartTime {
    #[serde(rename = "kind")]
    pub kind: Kind,
    #[serde(rename = "time", skip_serializing_if = "Option::is_none")]
    pub time: Option<Box<models::SmartSceneTimeslotGetStartTimeTime>>,
}

impl SmartSceneTimeslotGetStartTime {
    pub fn new(kind: Kind) -> SmartSceneTimeslotGetStartTime {
        SmartSceneTimeslotGetStartTime { kind, time: None }
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Kind {
    #[serde(rename = "time")]
    Time,
    #[serde(rename = "sunset")]
    Sunset,
}

impl Default for Kind {
    fn default() -> Kind {
        Self::Time
    }
}
