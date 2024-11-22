# A Rust Client for the Philips Hue API

This API client (for version v2) was originaly generated from [openhue.io](https://www.openhue.io) with the [OpenAPI Generator](https://openapi-generator.tech) project.

## Documentation for API Endpoints

All URIs are relative to *<https://192.168.1.0>*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*AuthApi* | [**authenticate**](docs/AuthApi.md#authenticate) | **POST** /api | Authenticate
*BridgeApi* | [**get_bridge**](docs/BridgeApi.md#get_bridge) | **GET** /clip/v2/resource/bridge/{bridgeId} | Get bridge
*BridgeApi* | [**get_bridges**](docs/BridgeApi.md#get_bridges) | **GET** /clip/v2/resource/bridge | List bridges
*BridgeApi* | [**update_bridge**](docs/BridgeApi.md#update_bridge) | **PUT** /clip/v2/resource/bridge/{bridgeId} | Update bridge
*BridgeHomeApi* | [**get_bridge_home**](docs/BridgeHomeApi.md#get_bridge_home) | **GET** /clip/v2/resource/bridge_home/{bridgeHomeId} | Get bridge home.
*BridgeHomeApi* | [**get_bridge_homes**](docs/BridgeHomeApi.md#get_bridge_homes) | **GET** /clip/v2/resource/bridge_home | List bridge homes.
*DeviceApi* | [**delete_device**](docs/DeviceApi.md#delete_device) | **DELETE** /clip/v2/resource/device/{deviceId} | Delete Device
*DeviceApi* | [**get_device**](docs/DeviceApi.md#get_device) | **GET** /clip/v2/resource/device/{deviceId} | Get device
*DeviceApi* | [**get_devices**](docs/DeviceApi.md#get_devices) | **GET** /clip/v2/resource/device | List devices
*DeviceApi* | [**update_device**](docs/DeviceApi.md#update_device) | **PUT** /clip/v2/resource/device/{deviceId} | Update device
*DevicePowerApi* | [**get_device_power**](docs/DevicePowerApi.md#get_device_power) | **GET** /clip/v2/resource/device_power/{deviceId} | Get device power
*DevicePowerApi* | [**get_device_powers**](docs/DevicePowerApi.md#get_device_powers) | **GET** /clip/v2/resource/device_power | List device powers
*GroupedLightApi* | [**get_grouped_light**](docs/GroupedLightApi.md#get_grouped_light) | **GET** /clip/v2/resource/grouped_light/{groupedLightId} | Get grouped light
*GroupedLightApi* | [**get_grouped_lights**](docs/GroupedLightApi.md#get_grouped_lights) | **GET** /clip/v2/resource/grouped_light | List grouped lights
*GroupedLightApi* | [**update_grouped_light**](docs/GroupedLightApi.md#update_grouped_light) | **PUT** /clip/v2/resource/grouped_light/{groupedLightId} | Update grouped light
*LightApi* | [**get_light**](docs/LightApi.md#get_light) | **GET** /clip/v2/resource/light/{lightId} | Get light
*LightApi* | [**get_lights**](docs/LightApi.md#get_lights) | **GET** /clip/v2/resource/light | List lights.
*LightApi* | [**update_light**](docs/LightApi.md#update_light) | **PUT** /clip/v2/resource/light/{lightId} | Update light
*LightLevelApi* | [**get_light_level**](docs/LightLevelApi.md#get_light_level) | **GET** /clip/v2/resource/light_level/{lightId} | Get light
*LightLevelApi* | [**get_light_levels**](docs/LightLevelApi.md#get_light_levels) | **GET** /clip/v2/resource/light_level | List light levels.
*LightLevelApi* | [**update_light_level**](docs/LightLevelApi.md#update_light_level) | **PUT** /clip/v2/resource/light_level/{lightId} | Update light
*MotionApi* | [**get_motion_sensor**](docs/MotionApi.md#get_motion_sensor) | **GET** /clip/v2/resource/motion/{motionId} | Get motion sensor.
*MotionApi* | [**get_motion_sensors**](docs/MotionApi.md#get_motion_sensors) | **GET** /clip/v2/resource/motion | List motion sensors.
*MotionApi* | [**update_motion_sensor**](docs/MotionApi.md#update_motion_sensor) | **PUT** /clip/v2/resource/motion/{motionId} | Update Motion Sensor
*ResourceApi* | [**get_resources**](docs/ResourceApi.md#get_resources) | **GET** /clip/v2/resource | List resources
*RoomApi* | [**create_room**](docs/RoomApi.md#create_room) | **POST** /clip/v2/resource/room | Create room
*RoomApi* | [**delete_room**](docs/RoomApi.md#delete_room) | **DELETE** /clip/v2/resource/room/{roomId} | Delete room
*RoomApi* | [**get_room**](docs/RoomApi.md#get_room) | **GET** /clip/v2/resource/room/{roomId} | Get room.
*RoomApi* | [**get_rooms**](docs/RoomApi.md#get_rooms) | **GET** /clip/v2/resource/room | List rooms
*RoomApi* | [**update_room**](docs/RoomApi.md#update_room) | **PUT** /clip/v2/resource/room/{roomId} | Update room
*SceneApi* | [**create_scene**](docs/SceneApi.md#create_scene) | **POST** /clip/v2/resource/scene | Create a new scene
*SceneApi* | [**delete_scene**](docs/SceneApi.md#delete_scene) | **DELETE** /clip/v2/resource/scene/{sceneId} | Delete a scene
*SceneApi* | [**get_scene**](docs/SceneApi.md#get_scene) | **GET** /clip/v2/resource/scene/{sceneId} | Get a scene
*SceneApi* | [**get_scenes**](docs/SceneApi.md#get_scenes) | **GET** /clip/v2/resource/scene | List scenes
*SceneApi* | [**update_scene**](docs/SceneApi.md#update_scene) | **PUT** /clip/v2/resource/scene/{sceneId} | Update a scene
*SmartSceneApi* | [**create_smart_scene**](docs/SmartSceneApi.md#create_smart_scene) | **POST** /clip/v2/resource/smart_scene | Create a new smart scene
*SmartSceneApi* | [**delete_smart_scene**](docs/SmartSceneApi.md#delete_smart_scene) | **DELETE** /clip/v2/resource/smart_scene/{sceneId} | Delete a smart scene
*SmartSceneApi* | [**get_smart_scene**](docs/SmartSceneApi.md#get_smart_scene) | **GET** /clip/v2/resource/smart_scene/{sceneId} | Get a smart scene
*SmartSceneApi* | [**get_smart_scenes**](docs/SmartSceneApi.md#get_smart_scenes) | **GET** /clip/v2/resource/smart_scene | List smart scenes
*SmartSceneApi* | [**update_smart_scene**](docs/SmartSceneApi.md#update_smart_scene) | **PUT** /clip/v2/resource/smart_scene/{sceneId} | Update a smart scene
*TemperatureApi* | [**get_temperature**](docs/TemperatureApi.md#get_temperature) | **GET** /clip/v2/resource/temperature/{temperatureId} | Get temperature sensor information
*TemperatureApi* | [**get_temperatures**](docs/TemperatureApi.md#get_temperatures) | **GET** /clip/v2/resource/temperature | List temperatures
*TemperatureApi* | [**update_temperature**](docs/TemperatureApi.md#update_temperature) | **PUT** /clip/v2/resource/temperature/{temperatureId} | Update temperature sensor
*ZoneApi* | [**create_zone**](docs/ZoneApi.md#create_zone) | **POST** /clip/v2/resource/zone | Create zone
*ZoneApi* | [**delete_zone**](docs/ZoneApi.md#delete_zone) | **DELETE** /clip/v2/resource/zone/{zoneId} | Delete Zone
*ZoneApi* | [**get_zone**](docs/ZoneApi.md#get_zone) | **GET** /clip/v2/resource/zone/{zoneId} | Get Zone.
*ZoneApi* | [**get_zones**](docs/ZoneApi.md#get_zones) | **GET** /clip/v2/resource/zone | List zones
*ZoneApi* | [**update_zone**](docs/ZoneApi.md#update_zone) | **PUT** /clip/v2/resource/zone/{zoneId} | Update Zone

## Documentation For Models

- [ActionGet](docs/ActionGet.md)
- [ActionGetAllOfAction](docs/ActionGetAllOfAction.md)
- [ActionGetAllOfActionEffects](docs/ActionGetAllOfActionEffects.md)
- [ActionPost](docs/ActionPost.md)
- [ActionPostAction](docs/ActionPostAction.md)
- [Alert](docs/Alert.md)
- [ApiResponse](docs/ApiResponse.md)
- [AuthenticateRequest](docs/AuthenticateRequest.md)
- [BridgeGet](docs/BridgeGet.md)
- [BridgeGetAllOfTimeZone](docs/BridgeGetAllOfTimeZone.md)
- [BridgeHomeGet](docs/BridgeHomeGet.md)
- [BridgePut](docs/BridgePut.md)
- [Color](docs/Color.md)
- [ColorPaletteGet](docs/ColorPaletteGet.md)
- [ColorTemperature](docs/ColorTemperature.md)
- [ColorTemperatureDelta](docs/ColorTemperatureDelta.md)
- [ColorTemperaturePalettePost](docs/ColorTemperaturePalettePost.md)
- [ColorTemperaturePalettePostColorTemperature](docs/ColorTemperaturePalettePostColorTemperature.md)
- [DayTimeslotsGet](docs/DayTimeslotsGet.md)
- [DeviceGet](docs/DeviceGet.md)
- [DeviceGetAllOfMetadata](docs/DeviceGetAllOfMetadata.md)
- [DeviceGetAllOfUsertest](docs/DeviceGetAllOfUsertest.md)
- [DevicePowerGet](docs/DevicePowerGet.md)
- [DevicePowerGetAllOfPowerState](docs/DevicePowerGetAllOfPowerState.md)
- [DevicePut](docs/DevicePut.md)
- [DevicePutIdentify](docs/DevicePutIdentify.md)
- [DevicePutUsertest](docs/DevicePutUsertest.md)
- [Dimming](docs/Dimming.md)
- [DimmingDelta](docs/DimmingDelta.md)
- [Dynamics](docs/Dynamics.md)
- [Effects](docs/Effects.md)
- [Error](docs/Error.md)
- [ErrorResponse](docs/ErrorResponse.md)
- [GamutPosition](docs/GamutPosition.md)
- [GetBridgeHomes200Response](docs/GetBridgeHomes200Response.md)
- [GetBridges200Response](docs/GetBridges200Response.md)
- [GetDevicePowers200Response](docs/GetDevicePowers200Response.md)
- [GetDevices200Response](docs/GetDevices200Response.md)
- [GetGroupedLights200Response](docs/GetGroupedLights200Response.md)
- [GetLightLevels200Response](docs/GetLightLevels200Response.md)
- [GetLights200Response](docs/GetLights200Response.md)
- [GetMotionSensors200Response](docs/GetMotionSensors200Response.md)
- [GetResources200Response](docs/GetResources200Response.md)
- [GetRooms200Response](docs/GetRooms200Response.md)
- [GetScenes200Response](docs/GetScenes200Response.md)
- [GetSmartScenes200Response](docs/GetSmartScenes200Response.md)
- [GetTemperatures200Response](docs/GetTemperatures200Response.md)
- [Gradient](docs/Gradient.md)
- [GroupedLightGet](docs/GroupedLightGet.md)
- [GroupedLightGetAllOfAlert](docs/GroupedLightGetAllOfAlert.md)
- [GroupedLightGetAllOfSignaling](docs/GroupedLightGetAllOfSignaling.md)
- [GroupedLightPut](docs/GroupedLightPut.md)
- [LightArchetype](docs/LightArchetype.md)
- [LightDynamics](docs/LightDynamics.md)
- [LightGet](docs/LightGet.md)
- [LightGetAllOfColor](docs/LightGetAllOfColor.md)
- [LightGetAllOfColorGamut](docs/LightGetAllOfColorGamut.md)
- [LightGetAllOfColorTemperature](docs/LightGetAllOfColorTemperature.md)
- [LightGetAllOfColorTemperatureMirekSchema](docs/LightGetAllOfColorTemperatureMirekSchema.md)
- [LightGetAllOfDimming](docs/LightGetAllOfDimming.md)
- [LightGetAllOfDynamics](docs/LightGetAllOfDynamics.md)
- [LightGetAllOfEffects](docs/LightGetAllOfEffects.md)
- [LightGetAllOfMetadata](docs/LightGetAllOfMetadata.md)
- [LightGetAllOfPowerup](docs/LightGetAllOfPowerup.md)
- [LightGetAllOfPowerupDimming](docs/LightGetAllOfPowerupDimming.md)
- [LightGetAllOfPowerupDimmingColor](docs/LightGetAllOfPowerupDimmingColor.md)
- [LightGetAllOfPowerupDimmingColorColorTemperature](docs/LightGetAllOfPowerupDimmingColorColorTemperature.md)
- [LightGetAllOfPowerupOn](docs/LightGetAllOfPowerupOn.md)
- [LightGetAllOfSignaling](docs/LightGetAllOfSignaling.md)
- [LightGetAllOfTimedEffects](docs/LightGetAllOfTimedEffects.md)
- [LightLevelGet](docs/LightLevelGet.md)
- [LightLevelGetAllOfLight](docs/LightLevelGetAllOfLight.md)
- [LightLevelGetAllOfLightLightLevelReport](docs/LightLevelGetAllOfLightLightLevelReport.md)
- [LightLevelPut](docs/LightLevelPut.md)
- [LightPut](docs/LightPut.md)
- [LightPutTimedEffects](docs/LightPutTimedEffects.md)
- [MotionGet](docs/MotionGet.md)
- [MotionGetAllOfMotion](docs/MotionGetAllOfMotion.md)
- [MotionGetAllOfMotionMotionReport](docs/MotionGetAllOfMotionMotionReport.md)
- [MotionGetAllOfSensitivity](docs/MotionGetAllOfSensitivity.md)
- [MotionPut](docs/MotionPut.md)
- [MotionPutSensitivity](docs/MotionPutSensitivity.md)
- [On](docs/On.md)
- [Powerup](docs/Powerup.md)
- [PowerupDimming](docs/PowerupDimming.md)
- [ProductArchetype](docs/ProductArchetype.md)
- [ProductData](docs/ProductData.md)
- [Resource](docs/Resource.md)
- [ResourceGet](docs/ResourceGet.md)
- [ResourceIdentifier](docs/ResourceIdentifier.md)
- [ResourceOwned](docs/ResourceOwned.md)
- [ResponseInner](docs/ResponseInner.md)
- [ResponseInnerError](docs/ResponseInnerError.md)
- [ResponseInnerSuccess](docs/ResponseInnerSuccess.md)
- [RoomArchetype](docs/RoomArchetype.md)
- [RoomGet](docs/RoomGet.md)
- [RoomGetAllOfMetadata](docs/RoomGetAllOfMetadata.md)
- [RoomPut](docs/RoomPut.md)
- [SceneGet](docs/SceneGet.md)
- [SceneGetAllOfStatus](docs/SceneGetAllOfStatus.md)
- [SceneMetadata](docs/SceneMetadata.md)
- [ScenePalette](docs/ScenePalette.md)
- [ScenePaletteEffectsInner](docs/ScenePaletteEffectsInner.md)
- [ScenePost](docs/ScenePost.md)
- [ScenePut](docs/ScenePut.md)
- [SceneRecall](docs/SceneRecall.md)
- [Signaling](docs/Signaling.md)
- [SmartSceneGet](docs/SmartSceneGet.md)
- [SmartSceneGetAllOfActiveTimeslot](docs/SmartSceneGetAllOfActiveTimeslot.md)
- [SmartSceneMetadata](docs/SmartSceneMetadata.md)
- [SmartSceneMetadataWithImage](docs/SmartSceneMetadataWithImage.md)
- [SmartSceneOptionalRecall](docs/SmartSceneOptionalRecall.md)
- [SmartScenePost](docs/SmartScenePost.md)
- [SmartScenePut](docs/SmartScenePut.md)
- [SmartSceneRecall](docs/SmartSceneRecall.md)
- [SmartSceneTimeslotGet](docs/SmartSceneTimeslotGet.md)
- [SmartSceneTimeslotGetStartTime](docs/SmartSceneTimeslotGetStartTime.md)
- [SmartSceneTimeslotGetStartTimeTime](docs/SmartSceneTimeslotGetStartTimeTime.md)
- [SupportedDynamicStatus](docs/SupportedDynamicStatus.md)
- [SupportedEffects](docs/SupportedEffects.md)
- [SupportedGradientMode](docs/SupportedGradientMode.md)
- [SupportedSignals](docs/SupportedSignals.md)
- [SupportedTimedEffects](docs/SupportedTimedEffects.md)
- [TemperatureGet](docs/TemperatureGet.md)
- [TemperatureGetAllOfTemperature](docs/TemperatureGetAllOfTemperature.md)
- [TemperatureGetAllOfTemperatureTemperatureReport](docs/TemperatureGetAllOfTemperatureTemperatureReport.md)
- [TemperaturePut](docs/TemperaturePut.md)
- [UpdateDevice200Response](docs/UpdateDevice200Response.md)
- [Weekday](docs/Weekday.md)

To get access to the crate's generated documentation, use:

```bash
cargo doc --open
```
