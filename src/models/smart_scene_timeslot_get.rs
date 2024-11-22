use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SmartSceneTimeslotGet {
    #[serde(rename = "start_time")]
    pub start_time: Box<models::SmartSceneTimeslotGetStartTime>,
    #[serde(rename = "target")]
    pub target: Box<models::ResourceIdentifier>,
}

impl SmartSceneTimeslotGet {
    pub fn new(
        start_time: models::SmartSceneTimeslotGetStartTime,
        target: models::ResourceIdentifier,
    ) -> SmartSceneTimeslotGet {
        SmartSceneTimeslotGet {
            start_time: Box::new(start_time),
            target: Box::new(target),
        }
    }
}
