# SmartScenePost

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | Option<**String**> |  | [optional]
**metadata** | [**models::SmartSceneMetadataWithImage**](SmartSceneMetadataWithImage.md) |  | 
**group** | [**models::ResourceIdentifier**](ResourceIdentifier.md) |  | 
**week_timeslots** | [**Vec<models::DayTimeslotsGet>**](DayTimeslotsGet.md) | information on what is the light state for every timeslot of the day | 
**transition_duration** | Option<**i32**> | duration of the transition from on one timeslot's scene to the other (defaults to 60000ms) | [optional]
**recall** | Option<[**models::SmartSceneRecall**](SmartSceneRecall.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


