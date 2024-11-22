# \BridgeHomeApi

All URIs are relative to *https://192.168.1.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_bridge_home**](BridgeHomeApi.md#get_bridge_home) | **GET** /clip/v2/resource/bridge_home/{bridgeHomeId} | Get bridge home.
[**get_bridge_homes**](BridgeHomeApi.md#get_bridge_homes) | **GET** /clip/v2/resource/bridge_home | List bridge homes.



## get_bridge_home

> models::GetBridgeHomes200Response get_bridge_home(bridge_home_id)
Get bridge home.

Get details of a single bridge home from its given `{bridgeHomeId}`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bridge_home_id** | **String** | ID of the bridge home. | [required] |

### Return type

[**models::GetBridgeHomes200Response**](getBridgeHomes_200_response.md)

### Authorization

[HueApplicationKey](../README.md#HueApplicationKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_bridge_homes

> models::GetBridgeHomes200Response get_bridge_homes()
List bridge homes.

List all available bridge homes.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetBridgeHomes200Response**](getBridgeHomes_200_response.md)

### Authorization

[HueApplicationKey](../README.md#HueApplicationKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

