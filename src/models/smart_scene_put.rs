use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SmartScenePut {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Box<models::SmartSceneMetadata>>,
    /// information on what is the light state for every timeslot of the day
    #[serde(rename = "week_timeslots", skip_serializing_if = "Option::is_none")]
    pub week_timeslots: Option<Vec<models::DayTimeslotsGet>>,
    /// duration of the transition from on one timeslot's scene to the other (defaults to 60000ms)
    #[serde(
        rename = "transition_duration",
        skip_serializing_if = "Option::is_none"
    )]
    pub transition_duration: Option<i32>,
    #[serde(rename = "recall", skip_serializing_if = "Option::is_none")]
    pub recall: Option<Box<models::SmartSceneOptionalRecall>>,
}

impl SmartScenePut {
    pub fn new() -> SmartScenePut {
        SmartScenePut {
            r#type: None,
            metadata: None,
            week_timeslots: None,
            transition_duration: None,
            recall: None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "smart_scene")]
    SmartScene,
}

impl Default for Type {
    fn default() -> Type {
        Self::SmartScene
    }
}
