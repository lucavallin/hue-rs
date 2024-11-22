# \DeviceApi

All URIs are relative to *https://192.168.1.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_device**](DeviceApi.md#delete_device) | **DELETE** /clip/v2/resource/device/{deviceId} | Delete Device
[**get_device**](DeviceApi.md#get_device) | **GET** /clip/v2/resource/device/{deviceId} | Get device
[**get_devices**](DeviceApi.md#get_devices) | **GET** /clip/v2/resource/device | List devices
[**update_device**](DeviceApi.md#update_device) | **PUT** /clip/v2/resource/device/{deviceId} | Update device



## delete_device

> models::UpdateDevice200Response delete_device(device_id)
Delete Device

Delete a single Device from its given `{deviceId}`. The `bridge` device cannot be deleted.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**device_id** | **String** | ID of the Device | [required] |

### Return type

[**models::UpdateDevice200Response**](updateDevice_200_response.md)

### Authorization

[HueApplicationKey](../README.md#HueApplicationKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_device

> models::GetDevices200Response get_device(device_id)
Get device

Get details of a single device from its given `{deviceId}`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**device_id** | **String** | ID of the device | [required] |

### Return type

[**models::GetDevices200Response**](getDevices_200_response.md)

### Authorization

[HueApplicationKey](../README.md#HueApplicationKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_devices

> models::GetDevices200Response get_devices()
List devices

List all available devices

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetDevices200Response**](getDevices_200_response.md)

### Authorization

[HueApplicationKey](../README.md#HueApplicationKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_device

> models::UpdateDevice200Response update_device(device_id, device_put)
Update device

Update a single device from its given `{deviceId}`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**device_id** | **String** | ID of the device | [required] |
**device_put** | Option<[**DevicePut**](DevicePut.md)> |  |  |

### Return type

[**models::UpdateDevice200Response**](updateDevice_200_response.md)

### Authorization

[HueApplicationKey](../README.md#HueApplicationKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

