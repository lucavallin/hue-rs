# DeviceGet

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | Option<**String**> |  | [optional]
**id** | Option<**String**> | Unique identifier representing a specific resource instance | [optional]
**id_v1** | Option<**String**> | Clip v1 resource identifier | [optional]
**owner** | Option<[**models::ResourceIdentifier**](ResourceIdentifier.md)> |  | [optional]
**product_data** | Option<[**models::ProductData**](ProductData.md)> |  | [optional]
**metadata** | Option<[**models::DeviceGetAllOfMetadata**](DeviceGet_allOf_metadata.md)> |  | [optional]
**usertest** | Option<[**models::DeviceGetAllOfUsertest**](DeviceGet_allOf_usertest.md)> |  | [optional]
**services** | Option<[**Vec<models::ResourceIdentifier>**](ResourceIdentifier.md)> | References all services providing control and state of the device. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


