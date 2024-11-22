# \ZoneApi

All URIs are relative to *https://192.168.1.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_zone**](ZoneApi.md#create_zone) | **POST** /clip/v2/resource/zone | Create zone
[**delete_zone**](ZoneApi.md#delete_zone) | **DELETE** /clip/v2/resource/zone/{zoneId} | Delete Zone
[**get_zone**](ZoneApi.md#get_zone) | **GET** /clip/v2/resource/zone/{zoneId} | Get Zone.
[**get_zones**](ZoneApi.md#get_zones) | **GET** /clip/v2/resource/zone | List zones
[**update_zone**](ZoneApi.md#update_zone) | **PUT** /clip/v2/resource/zone/{zoneId} | Update Zone



## create_zone

> models::UpdateDevice200Response create_zone(room_put)
Create zone

Create a new zone

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**room_put** | Option<[**RoomPut**](RoomPut.md)> |  |  |

### Return type

[**models::UpdateDevice200Response**](updateDevice_200_response.md)

### Authorization

[HueApplicationKey](../README.md#HueApplicationKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_zone

> models::UpdateDevice200Response delete_zone(zone_id)
Delete Zone

Delete a single Zone from its given `{zoneId}`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone_id** | **String** | ID of the Zone | [required] |

### Return type

[**models::UpdateDevice200Response**](updateDevice_200_response.md)

### Authorization

[HueApplicationKey](../README.md#HueApplicationKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_zone

> models::GetRooms200Response get_zone(zone_id)
Get Zone.

Get details of a single Zone from its given `{zoneId}`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone_id** | **String** | ID of the Zone | [required] |

### Return type

[**models::GetRooms200Response**](getRooms_200_response.md)

### Authorization

[HueApplicationKey](../README.md#HueApplicationKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_zones

> models::GetRooms200Response get_zones()
List zones

List all available zones

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetRooms200Response**](getRooms_200_response.md)

### Authorization

[HueApplicationKey](../README.md#HueApplicationKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_zone

> models::UpdateDevice200Response update_zone(zone_id, room_put)
Update Zone

Update a single Zone from its given `{zoneId}`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone_id** | **String** | ID of the Zone | [required] |
**room_put** | Option<[**RoomPut**](RoomPut.md)> |  |  |

### Return type

[**models::UpdateDevice200Response**](updateDevice_200_response.md)

### Authorization

[HueApplicationKey](../README.md#HueApplicationKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

