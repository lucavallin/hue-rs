# MotionGet

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | Option<**String**> | Type of the supported resources | [optional]
**id** | Option<**String**> | Unique identifier representing a specific resource instance | [optional]
**id_v1** | Option<**String**> | Clip v1 resource identifier | [optional]
**owner** | Option<[**models::ResourceIdentifier**](ResourceIdentifier.md)> |  | [optional]
**enabled** | Option<**bool**> | ture when the sensor is activated, false when deactivated | [optional]
**motion** | Option<[**models::MotionGetAllOfMotion**](MotionGet_allOf_motion.md)> |  | [optional]
**sensitivity** | Option<[**models::MotionGetAllOfSensitivity**](MotionGet_allOf_sensitivity.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

