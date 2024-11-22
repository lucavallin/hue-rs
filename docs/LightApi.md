# \LightApi

All URIs are relative to *https://192.168.1.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_light**](LightApi.md#get_light) | **GET** /clip/v2/resource/light/{lightId} | Get light
[**get_lights**](LightApi.md#get_lights) | **GET** /clip/v2/resource/light | List lights.
[**update_light**](LightApi.md#update_light) | **PUT** /clip/v2/resource/light/{lightId} | Update light



## get_light

> models::GetLights200Response get_light(light_id)
Get light

Get details of a single light from its given `{lightId}`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**light_id** | **String** | ID of the light | [required] |

### Return type

[**models::GetLights200Response**](getLights_200_response.md)

### Authorization

[HueApplicationKey](../README.md#HueApplicationKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lights

> models::GetLights200Response get_lights()
List lights.

List all available lights.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetLights200Response**](getLights_200_response.md)

### Authorization

[HueApplicationKey](../README.md#HueApplicationKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_light

> models::UpdateDevice200Response update_light(light_id, light_put)
Update light

Update a single light from its given `{lightId}`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**light_id** | **String** | ID of the light | [required] |
**light_put** | Option<[**LightPut**](LightPut.md)> |  |  |

### Return type

[**models::UpdateDevice200Response**](updateDevice_200_response.md)

### Authorization

[HueApplicationKey](../README.md#HueApplicationKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

