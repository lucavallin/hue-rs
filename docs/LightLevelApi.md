# \LightLevelApi

All URIs are relative to *https://192.168.1.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_light_level**](LightLevelApi.md#get_light_level) | **GET** /clip/v2/resource/light_level/{lightId} | Get light
[**get_light_levels**](LightLevelApi.md#get_light_levels) | **GET** /clip/v2/resource/light_level | List light levels.
[**update_light_level**](LightLevelApi.md#update_light_level) | **PUT** /clip/v2/resource/light_level/{lightId} | Update light



## get_light_level

> models::GetLightLevels200Response get_light_level(light_id)
Get light

Get details of a single light from its given `{lightId}`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**light_id** | **String** | ID of the light | [required] |

### Return type

[**models::GetLightLevels200Response**](getLightLevels_200_response.md)

### Authorization

[HueApplicationKey](../README.md#HueApplicationKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_light_levels

> models::GetLightLevels200Response get_light_levels()
List light levels.

List all available light levels.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetLightLevels200Response**](getLightLevels_200_response.md)

### Authorization

[HueApplicationKey](../README.md#HueApplicationKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_light_level

> models::UpdateDevice200Response update_light_level(light_id, light_level_put)
Update light

Update a single light from its given `{lightId}`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**light_id** | **String** | ID of the light | [required] |
**light_level_put** | Option<[**LightLevelPut**](LightLevelPut.md)> |  |  |

### Return type

[**models::UpdateDevice200Response**](updateDevice_200_response.md)

### Authorization

[HueApplicationKey](../README.md#HueApplicationKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

