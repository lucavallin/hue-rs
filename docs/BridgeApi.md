# \BridgeApi

All URIs are relative to *https://192.168.1.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_bridge**](BridgeApi.md#get_bridge) | **GET** /clip/v2/resource/bridge/{bridgeId} | Get bridge
[**get_bridges**](BridgeApi.md#get_bridges) | **GET** /clip/v2/resource/bridge | List bridges
[**update_bridge**](BridgeApi.md#update_bridge) | **PUT** /clip/v2/resource/bridge/{bridgeId} | Update bridge



## get_bridge

> models::GetBridges200Response get_bridge(bridge_id)
Get bridge

Get details of a single bridge from its given `{bridgeId}`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bridge_id** | **String** | ID of the bridge | [required] |

### Return type

[**models::GetBridges200Response**](getBridges_200_response.md)

### Authorization

[HueApplicationKey](../README.md#HueApplicationKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_bridges

> models::GetBridges200Response get_bridges()
List bridges

List all available bridges

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetBridges200Response**](getBridges_200_response.md)

### Authorization

[HueApplicationKey](../README.md#HueApplicationKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_bridge

> models::UpdateDevice200Response update_bridge(bridge_id, bridge_put)
Update bridge

Update a single bridge from its given `{bridgeId}`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bridge_id** | **String** | ID of the bridge | [required] |
**bridge_put** | Option<[**BridgePut**](BridgePut.md)> |  |  |

### Return type

[**models::UpdateDevice200Response**](updateDevice_200_response.md)

### Authorization

[HueApplicationKey](../README.md#HueApplicationKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

