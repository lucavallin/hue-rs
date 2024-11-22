# SceneGet

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | Option<**String**> |  | [optional]
**id** | Option<**String**> | Unique identifier representing a specific resource instance | [optional]
**id_v1** | Option<**String**> | Clip v1 resource identifier | [optional]
**owner** | Option<[**models::ResourceIdentifier**](ResourceIdentifier.md)> |  | [optional]
**actions** | Option<[**Vec<models::ActionGet>**](ActionGet.md)> | List of actions to be executed synchronously on recall | [optional]
**metadata** | Option<[**models::SceneMetadata**](SceneMetadata.md)> |  | [optional]
**group** | Option<[**models::ResourceIdentifier**](ResourceIdentifier.md)> |  | [optional]
**palette** | Option<[**models::ScenePalette**](ScenePalette.md)> |  | [optional]
**speed** | Option<**f64**> | Speed of dynamic palette for this scene | [optional]
**auto_dynamic** | Option<**bool**> | Indicates whether to automatically start the scene dynamically on active recall | [optional]
**status** | Option<[**models::SceneGetAllOfStatus**](SceneGet_allOf_status.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


