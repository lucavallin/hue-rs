use crate::models;
use serde::{Deserialize, Serialize};

/// SupportedGradientMode : Mode in which the points are currently being deployed. If not provided during PUT/POST it will be defaulted to interpolated_palette
/// Mode in which the points are currently being deployed. If not provided during PUT/POST it will be defaulted to interpolated_palette
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SupportedGradientMode {
    #[serde(rename = "interpolated_palette")]
    InterpolatedPalette,
    #[serde(rename = "interpolated_palette_mirrored")]
    InterpolatedPaletteMirrored,
    #[serde(rename = "random_pixelated")]
    RandomPixelated,
}

impl std::fmt::Display for SupportedGradientMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::InterpolatedPalette => write!(f, "interpolated_palette"),
            Self::InterpolatedPaletteMirrored => write!(f, "interpolated_palette_mirrored"),
            Self::RandomPixelated => write!(f, "random_pixelated"),
        }
    }
}

impl Default for SupportedGradientMode {
    fn default() -> SupportedGradientMode {
        Self::InterpolatedPalette
    }
}
