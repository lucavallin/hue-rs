# ScenePost

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | Option<**String**> |  | [optional]
**actions** | [**Vec<models::ActionPost>**](ActionPost.md) | List of actions to be executed synchronously on recall | 
**metadata** | [**models::SceneMetadata**](SceneMetadata.md) |  | 
**group** | [**models::ResourceIdentifier**](ResourceIdentifier.md) |  | 
**palette** | Option<[**models::ScenePalette**](ScenePalette.md)> |  | [optional]
**speed** | Option<**f64**> | Speed of dynamic palette for this scene | [optional]
**auto_dynamic** | Option<**bool**> | Indicates whether to automatically start the scene dynamically on active recall | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


