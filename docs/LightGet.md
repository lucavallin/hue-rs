# LightGet

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | Option<**String**> | Type of the supported resources | [optional]
**id** | Option<**String**> | Unique identifier representing a specific resource instance | [optional]
**id_v1** | Option<**String**> | Clip v1 resource identifier | [optional]
**owner** | Option<[**models::ResourceIdentifier**](ResourceIdentifier.md)> |  | [optional]
**metadata** | Option<[**models::LightGetAllOfMetadata**](LightGet_allOf_metadata.md)> |  | [optional]
**on** | Option<[**models::On**](On.md)> |  | [optional]
**dimming** | Option<[**models::LightGetAllOfDimming**](LightGet_allOf_dimming.md)> |  | [optional]
**color_temperature** | Option<[**models::LightGetAllOfColorTemperature**](LightGet_allOf_color_temperature.md)> |  | [optional]
**color** | Option<[**models::LightGetAllOfColor**](LightGet_allOf_color.md)> |  | [optional]
**dynamics** | Option<[**models::LightGetAllOfDynamics**](LightGet_allOf_dynamics.md)> |  | [optional]
**alert** | Option<[**serde_json::Value**](.md)> | TODO | [optional]
**signaling** | Option<[**models::LightGetAllOfSignaling**](LightGet_allOf_signaling.md)> |  | [optional]
**mode** | Option<**String**> |  | [optional]
**gradient** | Option<[**serde_json::Value**](.md)> |  | [optional]
**effects** | Option<[**models::LightGetAllOfEffects**](LightGet_allOf_effects.md)> |  | [optional]
**timed_effects** | Option<[**models::LightGetAllOfTimedEffects**](LightGet_allOf_timed_effects.md)> |  | [optional]
**powerup** | Option<[**models::LightGetAllOfPowerup**](LightGet_allOf_powerup.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


