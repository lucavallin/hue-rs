# SmartSceneGet

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | Option<**String**> |  | [optional]
**id** | Option<**String**> | Unique identifier representing a specific resource instance | [optional]
**id_v1** | Option<**String**> | Clip v1 resource identifier | [optional]
**metadata** | [**models::SmartSceneMetadataWithImage**](SmartSceneMetadataWithImage.md) |  | 
**group** | [**models::ResourceIdentifier**](ResourceIdentifier.md) |  | 
**week_timeslots** | [**Vec<models::DayTimeslotsGet>**](DayTimeslotsGet.md) | information on what is the light state for every timeslot of the day | 
**transition_duration** | **i32** | duration of the transition from on one timeslot's scene to the other (defaults to 60000ms) | 
**active_timeslot** | Option<[**models::SmartSceneGetAllOfActiveTimeslot**](SmartSceneGet_allOf_active_timeslot.md)> |  | [optional]
**state** | **String** | the current state of the smart scene. The default state is inactive if no recall is provided | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


