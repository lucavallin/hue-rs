# \TemperatureApi

All URIs are relative to *https://192.168.1.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_temperature**](TemperatureApi.md#get_temperature) | **GET** /clip/v2/resource/temperature/{temperatureId} | Get temperature sensor information
[**get_temperatures**](TemperatureApi.md#get_temperatures) | **GET** /clip/v2/resource/temperature | List temperatures
[**update_temperature**](TemperatureApi.md#update_temperature) | **PUT** /clip/v2/resource/temperature/{temperatureId} | Update temperature sensor



## get_temperature

> models::GetTemperatures200Response get_temperature(temperature_id)
Get temperature sensor information

Get details of a single temperature sensor from its given `{temperatureId}`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**temperature_id** | **String** | ID of the temperature sensor | [required] |

### Return type

[**models::GetTemperatures200Response**](getTemperatures_200_response.md)

### Authorization

[HueApplicationKey](../README.md#HueApplicationKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_temperatures

> models::GetTemperatures200Response get_temperatures()
List temperatures

List all temperatures

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetTemperatures200Response**](getTemperatures_200_response.md)

### Authorization

[HueApplicationKey](../README.md#HueApplicationKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_temperature

> models::UpdateDevice200Response update_temperature(temperature_id, temperature_put)
Update temperature sensor

Update a temperature sensor from its given `{temperatureId}`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**temperature_id** | **String** | ID of the temperature sensor | [required] |
**temperature_put** | Option<[**TemperaturePut**](TemperaturePut.md)> |  |  |

### Return type

[**models::UpdateDevice200Response**](updateDevice_200_response.md)

### Authorization

[HueApplicationKey](../README.md#HueApplicationKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

