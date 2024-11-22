use crate::models;
use serde::{Deserialize, Serialize};

/// RoomArchetype : Possible archetypes of a room
/// Possible archetypes of a room
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RoomArchetype {
    #[serde(rename = "living_room")]
    LivingRoom,
    #[serde(rename = "kitchen")]
    Kitchen,
    #[serde(rename = "dining")]
    Dining,
    #[serde(rename = "bedroom")]
    Bedroom,
    #[serde(rename = "kids_bedroom")]
    KidsBedroom,
    #[serde(rename = "bathroom")]
    Bathroom,
    #[serde(rename = "nursery")]
    Nursery,
    #[serde(rename = "recreation")]
    Recreation,
    #[serde(rename = "office")]
    Office,
    #[serde(rename = "gym")]
    Gym,
    #[serde(rename = "hallway")]
    Hallway,
    #[serde(rename = "toilet")]
    Toilet,
    #[serde(rename = "front_door")]
    FrontDoor,
    #[serde(rename = "garage")]
    Garage,
    #[serde(rename = "terrace")]
    Terrace,
    #[serde(rename = "garden")]
    Garden,
    #[serde(rename = "driveway")]
    Driveway,
    #[serde(rename = "carport")]
    Carport,
    #[serde(rename = "home")]
    Home,
    #[serde(rename = "downstairs")]
    Downstairs,
    #[serde(rename = "upstairs")]
    Upstairs,
    #[serde(rename = "top_floor")]
    TopFloor,
    #[serde(rename = "attic")]
    Attic,
    #[serde(rename = "guest_room")]
    GuestRoom,
    #[serde(rename = "staircase")]
    Staircase,
    #[serde(rename = "lounge")]
    Lounge,
    #[serde(rename = "man_cave")]
    ManCave,
    #[serde(rename = "computer")]
    Computer,
    #[serde(rename = "studio")]
    Studio,
    #[serde(rename = "music")]
    Music,
    #[serde(rename = "tv")]
    Tv,
    #[serde(rename = "reading")]
    Reading,
    #[serde(rename = "closet")]
    Closet,
    #[serde(rename = "storage")]
    Storage,
    #[serde(rename = "laundry_room")]
    LaundryRoom,
    #[serde(rename = "balcony")]
    Balcony,
    #[serde(rename = "porch")]
    Porch,
    #[serde(rename = "barbecue")]
    Barbecue,
    #[serde(rename = "pool")]
    Pool,
    #[serde(rename = "other")]
    Other,
}

impl std::fmt::Display for RoomArchetype {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::LivingRoom => write!(f, "living_room"),
            Self::Kitchen => write!(f, "kitchen"),
            Self::Dining => write!(f, "dining"),
            Self::Bedroom => write!(f, "bedroom"),
            Self::KidsBedroom => write!(f, "kids_bedroom"),
            Self::Bathroom => write!(f, "bathroom"),
            Self::Nursery => write!(f, "nursery"),
            Self::Recreation => write!(f, "recreation"),
            Self::Office => write!(f, "office"),
            Self::Gym => write!(f, "gym"),
            Self::Hallway => write!(f, "hallway"),
            Self::Toilet => write!(f, "toilet"),
            Self::FrontDoor => write!(f, "front_door"),
            Self::Garage => write!(f, "garage"),
            Self::Terrace => write!(f, "terrace"),
            Self::Garden => write!(f, "garden"),
            Self::Driveway => write!(f, "driveway"),
            Self::Carport => write!(f, "carport"),
            Self::Home => write!(f, "home"),
            Self::Downstairs => write!(f, "downstairs"),
            Self::Upstairs => write!(f, "upstairs"),
            Self::TopFloor => write!(f, "top_floor"),
            Self::Attic => write!(f, "attic"),
            Self::GuestRoom => write!(f, "guest_room"),
            Self::Staircase => write!(f, "staircase"),
            Self::Lounge => write!(f, "lounge"),
            Self::ManCave => write!(f, "man_cave"),
            Self::Computer => write!(f, "computer"),
            Self::Studio => write!(f, "studio"),
            Self::Music => write!(f, "music"),
            Self::Tv => write!(f, "tv"),
            Self::Reading => write!(f, "reading"),
            Self::Closet => write!(f, "closet"),
            Self::Storage => write!(f, "storage"),
            Self::LaundryRoom => write!(f, "laundry_room"),
            Self::Balcony => write!(f, "balcony"),
            Self::Porch => write!(f, "porch"),
            Self::Barbecue => write!(f, "barbecue"),
            Self::Pool => write!(f, "pool"),
            Self::Other => write!(f, "other"),
        }
    }
}

impl Default for RoomArchetype {
    fn default() -> RoomArchetype {
        Self::LivingRoom
    }
}
