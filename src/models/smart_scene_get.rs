use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SmartSceneGet {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,
    /// Unique identifier representing a specific resource instance
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Clip v1 resource identifier
    #[serde(rename = "id_v1", skip_serializing_if = "Option::is_none")]
    pub id_v1: Option<String>,
    #[serde(rename = "metadata")]
    pub metadata: Box<models::SmartSceneMetadataWithImage>,
    #[serde(rename = "group")]
    pub group: Box<models::ResourceIdentifier>,
    /// information on what is the light state for every timeslot of the day
    #[serde(rename = "week_timeslots")]
    pub week_timeslots: Vec<models::DayTimeslotsGet>,
    /// duration of the transition from on one timeslot's scene to the other (defaults to 60000ms)
    #[serde(rename = "transition_duration")]
    pub transition_duration: i32,
    #[serde(rename = "active_timeslot", skip_serializing_if = "Option::is_none")]
    pub active_timeslot: Option<Box<models::SmartSceneGetAllOfActiveTimeslot>>,
    /// the current state of the smart scene. The default state is inactive if no recall is provided
    #[serde(rename = "state")]
    pub state: State,
}

impl SmartSceneGet {
    pub fn new(
        metadata: models::SmartSceneMetadataWithImage,
        group: models::ResourceIdentifier,
        week_timeslots: Vec<models::DayTimeslotsGet>,
        transition_duration: i32,
        state: State,
    ) -> SmartSceneGet {
        SmartSceneGet {
            r#type: None,
            id: None,
            id_v1: None,
            metadata: Box::new(metadata),
            group: Box::new(group),
            week_timeslots,
            transition_duration,
            active_timeslot: None,
            state,
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
/// the current state of the smart scene. The default state is inactive if no recall is provided
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum State {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "inactive")]
    Inactive,
}

impl Default for State {
    fn default() -> State {
        Self::Active
    }
}
