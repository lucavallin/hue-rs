# \SmartSceneApi

All URIs are relative to *https://192.168.1.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_smart_scene**](SmartSceneApi.md#create_smart_scene) | **POST** /clip/v2/resource/smart_scene | Create a new smart scene
[**delete_smart_scene**](SmartSceneApi.md#delete_smart_scene) | **DELETE** /clip/v2/resource/smart_scene/{sceneId} | Delete a smart scene
[**get_smart_scene**](SmartSceneApi.md#get_smart_scene) | **GET** /clip/v2/resource/smart_scene/{sceneId} | Get a smart scene
[**get_smart_scenes**](SmartSceneApi.md#get_smart_scenes) | **GET** /clip/v2/resource/smart_scene | List smart scenes
[**update_smart_scene**](SmartSceneApi.md#update_smart_scene) | **PUT** /clip/v2/resource/smart_scene/{sceneId} | Update a smart scene



## create_smart_scene

> models::UpdateDevice200Response create_smart_scene(smart_scene_post)
Create a new smart scene

Creates a new smart scene

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**smart_scene_post** | Option<[**SmartScenePost**](SmartScenePost.md)> |  |  |

### Return type

[**models::UpdateDevice200Response**](updateDevice_200_response.md)

### Authorization

[HueApplicationKey](../README.md#HueApplicationKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_smart_scene

> models::UpdateDevice200Response delete_smart_scene(scene_id)
Delete a smart scene

Delete a single smart scene from its given `{sceneId}`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scene_id** | **String** | ID of the smart scene. | [required] |

### Return type

[**models::UpdateDevice200Response**](updateDevice_200_response.md)

### Authorization

[HueApplicationKey](../README.md#HueApplicationKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_smart_scene

> models::GetSmartScenes200Response get_smart_scene(scene_id)
Get a smart scene

Get details of a single smart scene from its given `{sceneId}`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scene_id** | **String** | ID of the smart scene. | [required] |

### Return type

[**models::GetSmartScenes200Response**](getSmartScenes_200_response.md)

### Authorization

[HueApplicationKey](../README.md#HueApplicationKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_smart_scenes

> models::GetSmartScenes200Response get_smart_scenes()
List smart scenes

List all available smart scenes

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetSmartScenes200Response**](getSmartScenes_200_response.md)

### Authorization

[HueApplicationKey](../README.md#HueApplicationKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_smart_scene

> models::UpdateDevice200Response update_smart_scene(scene_id, smart_scene_put)
Update a smart scene

Update a single smart scene from its given `{sceneId}`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scene_id** | **String** | ID of the scene. | [required] |
**smart_scene_put** | Option<[**SmartScenePut**](SmartScenePut.md)> |  |  |

### Return type

[**models::UpdateDevice200Response**](updateDevice_200_response.md)

### Authorization

[HueApplicationKey](../README.md#HueApplicationKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

