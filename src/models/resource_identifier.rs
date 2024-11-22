use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceIdentifier {
    /// The unique id of the referenced resource
    #[serde(rename = "rid", skip_serializing_if = "Option::is_none")]
    pub rid: Option<String>,
    /// The type of the referenced resource
    #[serde(rename = "rtype", skip_serializing_if = "Option::is_none")]
    pub rtype: Option<Rtype>,
}

impl ResourceIdentifier {
    pub fn new() -> ResourceIdentifier {
        ResourceIdentifier {
            rid: None,
            rtype: None,
        }
    }
}
/// The type of the referenced resource
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Rtype {
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

impl Default for Rtype {
    fn default() -> Rtype {
        Self::Device
    }
}
