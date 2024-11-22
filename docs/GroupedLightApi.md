# \GroupedLightApi

All URIs are relative to *https://192.168.1.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_grouped_light**](GroupedLightApi.md#get_grouped_light) | **GET** /clip/v2/resource/grouped_light/{groupedLightId} | Get grouped light
[**get_grouped_lights**](GroupedLightApi.md#get_grouped_lights) | **GET** /clip/v2/resource/grouped_light | List grouped lights
[**update_grouped_light**](GroupedLightApi.md#update_grouped_light) | **PUT** /clip/v2/resource/grouped_light/{groupedLightId} | Update grouped light



## get_grouped_light

> models::GetGroupedLights200Response get_grouped_light(grouped_light_id)
Get grouped light

Get details of a single grouped light from its given `{groupedLightId}`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**grouped_light_id** | **String** | ID of the grouped light | [required] |

### Return type

[**models::GetGroupedLights200Response**](getGroupedLights_200_response.md)

### Authorization

[HueApplicationKey](../README.md#HueApplicationKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_grouped_lights

> models::GetGroupedLights200Response get_grouped_lights()
List grouped lights

List all grouped lights

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetGroupedLights200Response**](getGroupedLights_200_response.md)

### Authorization

[HueApplicationKey](../README.md#HueApplicationKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_grouped_light

> models::UpdateDevice200Response update_grouped_light(grouped_light_id, grouped_light_put)
Update grouped light

Update a single grouped light from its given `{groupedLightId}`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**grouped_light_id** | **String** | ID of the light | [required] |
**grouped_light_put** | Option<[**GroupedLightPut**](GroupedLightPut.md)> |  |  |

### Return type

[**models::UpdateDevice200Response**](updateDevice_200_response.md)

### Authorization

[HueApplicationKey](../README.md#HueApplicationKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

