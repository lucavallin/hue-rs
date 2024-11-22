# SmartScenePut

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | Option<**String**> |  | [optional]
**metadata** | Option<[**models::SmartSceneMetadata**](SmartSceneMetadata.md)> |  | [optional]
**week_timeslots** | Option<[**Vec<models::DayTimeslotsGet>**](DayTimeslotsGet.md)> | information on what is the light state for every timeslot of the day | [optional]
**transition_duration** | Option<**i32**> | duration of the transition from on one timeslot's scene to the other (defaults to 60000ms) | [optional]
**recall** | Option<[**models::SmartSceneOptionalRecall**](SmartSceneOptionalRecall.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


