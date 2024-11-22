# \RoomApi

All URIs are relative to *https://192.168.1.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_room**](RoomApi.md#create_room) | **POST** /clip/v2/resource/room | Create room
[**delete_room**](RoomApi.md#delete_room) | **DELETE** /clip/v2/resource/room/{roomId} | Delete room
[**get_room**](RoomApi.md#get_room) | **GET** /clip/v2/resource/room/{roomId} | Get room.
[**get_rooms**](RoomApi.md#get_rooms) | **GET** /clip/v2/resource/room | List rooms
[**update_room**](RoomApi.md#update_room) | **PUT** /clip/v2/resource/room/{roomId} | Update room



## create_room

> models::UpdateDevice200Response create_room(room_put)
Create room

Create a new room

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


## delete_room

> models::UpdateDevice200Response delete_room(room_id)
Delete room

Delete a single room from its given `{roomId}`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**room_id** | **String** | ID of the room | [required] |

### Return type

[**models::UpdateDevice200Response**](updateDevice_200_response.md)

### Authorization

[HueApplicationKey](../README.md#HueApplicationKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_room

> models::GetRooms200Response get_room(room_id)
Get room.

Get details of a single room from its given `{roomId}`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**room_id** | **String** | ID of the room | [required] |

### Return type

[**models::GetRooms200Response**](getRooms_200_response.md)

### Authorization

[HueApplicationKey](../README.md#HueApplicationKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_rooms

> models::GetRooms200Response get_rooms()
List rooms

List all available rooms

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


## update_room

> models::UpdateDevice200Response update_room(room_id, room_put)
Update room

Update a single room from its given `{roomId}`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**room_id** | **String** | ID of the room | [required] |
**room_put** | Option<[**RoomPut**](RoomPut.md)> |  |  |

### Return type

[**models::UpdateDevice200Response**](updateDevice_200_response.md)

### Authorization

[HueApplicationKey](../README.md#HueApplicationKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

