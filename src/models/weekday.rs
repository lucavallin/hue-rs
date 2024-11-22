use crate::models;
use serde::{Deserialize, Serialize};

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Weekday {
    #[serde(rename = "monday")]
    Monday,
    #[serde(rename = "tuesday")]
    Tuesday,
    #[serde(rename = "wednesday")]
    Wednesday,
    #[serde(rename = "thursday")]
    Thursday,
    #[serde(rename = "friday")]
    Friday,
    #[serde(rename = "saturday")]
    Saturday,
    #[serde(rename = "sunday")]
    Sunday,
}

impl std::fmt::Display for Weekday {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Monday => write!(f, "monday"),
            Self::Tuesday => write!(f, "tuesday"),
            Self::Wednesday => write!(f, "wednesday"),
            Self::Thursday => write!(f, "thursday"),
            Self::Friday => write!(f, "friday"),
            Self::Saturday => write!(f, "saturday"),
            Self::Sunday => write!(f, "sunday"),
        }
    }
}

impl Default for Weekday {
    fn default() -> Weekday {
        Self::Monday
    }
}
