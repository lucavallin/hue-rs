use crate::models;
use serde::{Deserialize, Serialize};

/// SmartSceneGetAllOfActiveTimeslot : the active time slot in execution
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SmartSceneGetAllOfActiveTimeslot {
    #[serde(rename = "timeslot_id")]
    pub timeslot_id: i32,
    #[serde(rename = "weekday")]
    pub weekday: models::Weekday,
}

impl SmartSceneGetAllOfActiveTimeslot {
    /// the active time slot in execution
    pub fn new(timeslot_id: i32, weekday: models::Weekday) -> SmartSceneGetAllOfActiveTimeslot {
        SmartSceneGetAllOfActiveTimeslot {
            timeslot_id,
            weekday,
        }
    }
}
