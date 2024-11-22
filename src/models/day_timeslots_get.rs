use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DayTimeslotsGet {
    #[serde(rename = "timeslots")]
    pub timeslots: Vec<models::SmartSceneTimeslotGet>,
    #[serde(rename = "recurrence")]
    pub recurrence: Vec<models::Weekday>,
}

impl DayTimeslotsGet {
    pub fn new(
        timeslots: Vec<models::SmartSceneTimeslotGet>,
        recurrence: Vec<models::Weekday>,
    ) -> DayTimeslotsGet {
        DayTimeslotsGet {
            timeslots,
            recurrence,
        }
    }
}
