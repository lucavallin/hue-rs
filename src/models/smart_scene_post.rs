use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SmartScenePost {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,
    #[serde(rename = "metadata")]
    pub metadata: Box<models::SmartSceneMetadataWithImage>,
    #[serde(rename = "group")]
    pub group: Box<models::ResourceIdentifier>,
    /// information on what is the light state for every timeslot of the day
    #[serde(rename = "week_timeslots")]
    pub week_timeslots: Vec<models::DayTimeslotsGet>,
    /// duration of the transition from on one timeslot's scene to the other (defaults to 60000ms)
    #[serde(
        rename = "transition_duration",
        skip_serializing_if = "Option::is_none"
    )]
    pub transition_duration: Option<i32>,
    #[serde(rename = "recall", skip_serializing_if = "Option::is_none")]
    pub recall: Option<Box<models::SmartSceneRecall>>,
}

impl SmartScenePost {
    pub fn new(
        metadata: models::SmartSceneMetadataWithImage,
        group: models::ResourceIdentifier,
        week_timeslots: Vec<models::DayTimeslotsGet>,
    ) -> SmartScenePost {
        SmartScenePost {
            r#type: None,
            metadata: Box::new(metadata),
            group: Box::new(group),
            week_timeslots,
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
