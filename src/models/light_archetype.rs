use crate::models;
use serde::{Deserialize, Serialize};

/// LightArchetype : Light archetype
/// Light archetype
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum LightArchetype {
    #[serde(rename = "unknown_archetype")]
    UnknownArchetype,
    #[serde(rename = "classic_bulb")]
    ClassicBulb,
    #[serde(rename = "sultan_bulb")]
    SultanBulb,
    #[serde(rename = "flood_bulb")]
    FloodBulb,
    #[serde(rename = "spot_bulb")]
    SpotBulb,
    #[serde(rename = "candle_bulb")]
    CandleBulb,
    #[serde(rename = "luster_bulb")]
    LusterBulb,
    #[serde(rename = "pendant_round")]
    PendantRound,
    #[serde(rename = "pendant_long")]
    PendantLong,
    #[serde(rename = "ceiling_round")]
    CeilingRound,
    #[serde(rename = "ceiling_square")]
    CeilingSquare,
    #[serde(rename = "floor_shade")]
    FloorShade,
    #[serde(rename = "floor_lantern")]
    FloorLantern,
    #[serde(rename = "table_shade")]
    TableShade,
    #[serde(rename = "recessed_ceiling")]
    RecessedCeiling,
    #[serde(rename = "recessed_floor")]
    RecessedFloor,
    #[serde(rename = "single_spot")]
    SingleSpot,
    #[serde(rename = "double_spot")]
    DoubleSpot,
    #[serde(rename = "table_wash")]
    TableWash,
    #[serde(rename = "wall_lantern")]
    WallLantern,
    #[serde(rename = "wall_shade")]
    WallShade,
    #[serde(rename = "flexible_lamp")]
    FlexibleLamp,
    #[serde(rename = "ground_spot")]
    GroundSpot,
    #[serde(rename = "wall_spot")]
    WallSpot,
    #[serde(rename = "plug")]
    Plug,
    #[serde(rename = "hue_go")]
    HueGo,
    #[serde(rename = "hue_lightstrip")]
    HueLightstrip,
    #[serde(rename = "hue_iris")]
    HueIris,
    #[serde(rename = "hue_bloom")]
    HueBloom,
    #[serde(rename = "bollard")]
    Bollard,
    #[serde(rename = "wall_washer")]
    WallWasher,
    #[serde(rename = "hue_play")]
    HuePlay,
    #[serde(rename = "vintage_bulb")]
    VintageBulb,
    #[serde(rename = "vintage_candle_bulb")]
    VintageCandleBulb,
    #[serde(rename = "ellipse_bulb")]
    EllipseBulb,
    #[serde(rename = "triangle_bulb")]
    TriangleBulb,
    #[serde(rename = "small_globe_bulb")]
    SmallGlobeBulb,
    #[serde(rename = "large_globe_bulb")]
    LargeGlobeBulb,
    #[serde(rename = "edison_bulb")]
    EdisonBulb,
    #[serde(rename = "christmas_tree")]
    ChristmasTree,
    #[serde(rename = "string_light")]
    StringLight,
    #[serde(rename = "hue_centris")]
    HueCentris,
    #[serde(rename = "hue_lightstrip_tv")]
    HueLightstripTv,
    #[serde(rename = "hue_lightstrip_pc")]
    HueLightstripPc,
    #[serde(rename = "hue_tube")]
    HueTube,
    #[serde(rename = "hue_signe")]
    HueSigne,
    #[serde(rename = "pendant_spot")]
    PendantSpot,
    #[serde(rename = "ceiling_horizontal")]
    CeilingHorizontal,
    #[serde(rename = "ceiling_tube")]
    CeilingTube,
}

impl std::fmt::Display for LightArchetype {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::UnknownArchetype => write!(f, "unknown_archetype"),
            Self::ClassicBulb => write!(f, "classic_bulb"),
            Self::SultanBulb => write!(f, "sultan_bulb"),
            Self::FloodBulb => write!(f, "flood_bulb"),
            Self::SpotBulb => write!(f, "spot_bulb"),
            Self::CandleBulb => write!(f, "candle_bulb"),
            Self::LusterBulb => write!(f, "luster_bulb"),
            Self::PendantRound => write!(f, "pendant_round"),
            Self::PendantLong => write!(f, "pendant_long"),
            Self::CeilingRound => write!(f, "ceiling_round"),
            Self::CeilingSquare => write!(f, "ceiling_square"),
            Self::FloorShade => write!(f, "floor_shade"),
            Self::FloorLantern => write!(f, "floor_lantern"),
            Self::TableShade => write!(f, "table_shade"),
            Self::RecessedCeiling => write!(f, "recessed_ceiling"),
            Self::RecessedFloor => write!(f, "recessed_floor"),
            Self::SingleSpot => write!(f, "single_spot"),
            Self::DoubleSpot => write!(f, "double_spot"),
            Self::TableWash => write!(f, "table_wash"),
            Self::WallLantern => write!(f, "wall_lantern"),
            Self::WallShade => write!(f, "wall_shade"),
            Self::FlexibleLamp => write!(f, "flexible_lamp"),
            Self::GroundSpot => write!(f, "ground_spot"),
            Self::WallSpot => write!(f, "wall_spot"),
            Self::Plug => write!(f, "plug"),
            Self::HueGo => write!(f, "hue_go"),
            Self::HueLightstrip => write!(f, "hue_lightstrip"),
            Self::HueIris => write!(f, "hue_iris"),
            Self::HueBloom => write!(f, "hue_bloom"),
            Self::Bollard => write!(f, "bollard"),
            Self::WallWasher => write!(f, "wall_washer"),
            Self::HuePlay => write!(f, "hue_play"),
            Self::VintageBulb => write!(f, "vintage_bulb"),
            Self::VintageCandleBulb => write!(f, "vintage_candle_bulb"),
            Self::EllipseBulb => write!(f, "ellipse_bulb"),
            Self::TriangleBulb => write!(f, "triangle_bulb"),
            Self::SmallGlobeBulb => write!(f, "small_globe_bulb"),
            Self::LargeGlobeBulb => write!(f, "large_globe_bulb"),
            Self::EdisonBulb => write!(f, "edison_bulb"),
            Self::ChristmasTree => write!(f, "christmas_tree"),
            Self::StringLight => write!(f, "string_light"),
            Self::HueCentris => write!(f, "hue_centris"),
            Self::HueLightstripTv => write!(f, "hue_lightstrip_tv"),
            Self::HueLightstripPc => write!(f, "hue_lightstrip_pc"),
            Self::HueTube => write!(f, "hue_tube"),
            Self::HueSigne => write!(f, "hue_signe"),
            Self::PendantSpot => write!(f, "pendant_spot"),
            Self::CeilingHorizontal => write!(f, "ceiling_horizontal"),
            Self::CeilingTube => write!(f, "ceiling_tube"),
        }
    }
}

impl Default for LightArchetype {
    fn default() -> LightArchetype {
        Self::UnknownArchetype
    }
}
