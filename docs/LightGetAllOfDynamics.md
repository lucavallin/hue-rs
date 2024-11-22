# LightGetAllOfDynamics

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**status** | Option<[**models::SupportedDynamicStatus**](SupportedDynamicStatus.md)> |  | [optional]
**status_values** | Option<[**Vec<models::SupportedDynamicStatus>**](SupportedDynamicStatus.md)> | Statuses in which a lamp could be when playing dynamics. | [optional]
**speed** | Option<**f64**> | speed of dynamic palette or effect. The speed is valid for the dynamic palette if the status is dynamic_palette or for the corresponding effect listed in status. In case of status none, the speed is not valid | [optional]
**speed_valid** | Option<**bool**> | Indicates whether the value presented in speed is valid | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


