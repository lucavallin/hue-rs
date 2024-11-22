# \DevicePowerApi

All URIs are relative to *https://192.168.1.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_device_power**](DevicePowerApi.md#get_device_power) | **GET** /clip/v2/resource/device_power/{deviceId} | Get device power
[**get_device_powers**](DevicePowerApi.md#get_device_powers) | **GET** /clip/v2/resource/device_power | List device powers



## get_device_power

> models::GetDevicePowers200Response get_device_power(device_id)
Get device power

Get power details of a single device from its given `{deviceId}`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**device_id** | **String** | ID of the device | [required] |

### Return type

[**models::GetDevicePowers200Response**](getDevicePowers_200_response.md)

### Authorization

[HueApplicationKey](../README.md#HueApplicationKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_device_powers

> models::GetDevicePowers200Response get_device_powers()
List device powers

List all available device powers

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetDevicePowers200Response**](getDevicePowers_200_response.md)

### Authorization

[HueApplicationKey](../README.md#HueApplicationKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

