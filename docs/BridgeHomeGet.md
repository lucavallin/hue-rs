# BridgeHomeGet

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | Option<**String**> |  | [optional]
**id** | Option<**String**> | Unique identifier representing a specific resource instance | [optional]
**id_v1** | Option<**String**> | Clip v1 resource identifier | [optional]
**children** | Option<[**Vec<models::ResourceIdentifier>**](ResourceIdentifier.md)> | Child devices/services to group by the derived group. | [optional]
**services** | Option<[**Vec<models::ResourceIdentifier>**](ResourceIdentifier.md)> | References all services aggregating control and state of children in the group. This includes all services grouped in the group hierarchy given by child relation. This includes all services of a device grouped in the group hierarchy given by child relation. Aggregation is per service type, ie every service type which can be grouped has a corresponding definition of grouped type Supported types: – grouped_light  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


