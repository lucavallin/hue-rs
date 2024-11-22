use crate::models;
use serde::{Deserialize, Serialize};

/// ResourceGet : The API is actually returning the full resource description depending on the type, not just the fields that are documented below.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceGet {
    /// Type of the supported resources
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,
    /// Unique identifier representing a specific resource instance
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Clip v1 resource identifier
    #[serde(rename = "id_v1", skip_serializing_if = "Option::is_none")]
    pub id_v1: Option<String>,
    #[serde(rename = "owner", skip_serializing_if = "Option::is_none")]
    pub owner: Option<Box<models::ResourceIdentifier>>,
}

impl ResourceGet {
    /// The API is actually returning the full resource description depending on the type, not just the fields that are documented below.
    pub fn new() -> ResourceGet {
        ResourceGet {
            r#type: None,
            id: None,
            id_v1: None,
            owner: None,
        }
    }
}
/// Type of the supported resources
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "device")]
    Device,
    #[serde(rename = "bridge_home")]
    BridgeHome,
    #[serde(rename = "room")]
    Room,
    #[serde(rename = "zone")]
    Zone,
    #[serde(rename = "light")]
    Light,
    #[serde(rename = "button")]
    Button,
    #[serde(rename = "relative_rotary")]
    RelativeRotary,
    #[serde(rename = "temperature")]
    Temperature,
    #[serde(rename = "light_level")]
    LightLevel,
    #[serde(rename = "motion")]
    Motion,
    #[serde(rename = "camera_motion")]
    CameraMotion,
    #[serde(rename = "entertainment")]
    Entertainment,
    #[serde(rename = "contact")]
    Contact,
    #[serde(rename = "tamper")]
    Tamper,
    #[serde(rename = "grouped_light")]
    GroupedLight,
    #[serde(rename = "device_power")]
    DevicePower,
    #[serde(rename = "zigbee_bridge_connectivity")]
    ZigbeeBridgeConnectivity,
    #[serde(rename = "zigbee_connectivity")]
    ZigbeeConnectivity,
    #[serde(rename = "zgp_connectivity")]
    ZgpConnectivity,
    #[serde(rename = "bridge")]
    Bridge,
    #[serde(rename = "zigbee_device_discovery")]
    ZigbeeDeviceDiscovery,
    #[serde(rename = "homekit")]
    Homekit,
    #[serde(rename = "matter")]
    Matter,
    #[serde(rename = "matter_fabric")]
    MatterFabric,
    #[serde(rename = "scene")]
    Scene,
    #[serde(rename = "entertainment_configuration")]
    EntertainmentConfiguration,
    #[serde(rename = "public_image")]
    PublicImage,
    #[serde(rename = "auth_v1")]
    AuthV1,
    #[serde(rename = "behavior_script")]
    BehaviorScript,
    #[serde(rename = "behavior_instance")]
    BehaviorInstance,
    #[serde(rename = "geofence")]
    Geofence,
    #[serde(rename = "geofence_client")]
    GeofenceClient,
    #[serde(rename = "geolocation")]
    Geolocation,
    #[serde(rename = "smart_scene")]
    SmartScene,
}

impl Default for Type {
    fn default() -> Type {
        Self::Device
    }
}
