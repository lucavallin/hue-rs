# \SceneApi

All URIs are relative to *https://192.168.1.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_scene**](SceneApi.md#create_scene) | **POST** /clip/v2/resource/scene | Create a new scene
[**delete_scene**](SceneApi.md#delete_scene) | **DELETE** /clip/v2/resource/scene/{sceneId} | Delete a scene
[**get_scene**](SceneApi.md#get_scene) | **GET** /clip/v2/resource/scene/{sceneId} | Get a scene
[**get_scenes**](SceneApi.md#get_scenes) | **GET** /clip/v2/resource/scene | List scenes
[**update_scene**](SceneApi.md#update_scene) | **PUT** /clip/v2/resource/scene/{sceneId} | Update a scene



## create_scene

> models::UpdateDevice200Response create_scene(scene_post)
Create a new scene

Creates a new scene

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scene_post** | Option<[**ScenePost**](ScenePost.md)> |  |  |

### Return type

[**models::UpdateDevice200Response**](updateDevice_200_response.md)

### Authorization

[HueApplicationKey](../README.md#HueApplicationKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_scene

> models::UpdateDevice200Response delete_scene(scene_id)
Delete a scene

Delete a single scene from its given `{sceneId}`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scene_id** | **String** | ID of the scene. | [required] |

### Return type

[**models::UpdateDevice200Response**](updateDevice_200_response.md)

### Authorization

[HueApplicationKey](../README.md#HueApplicationKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_scene

> models::GetScenes200Response get_scene(scene_id)
Get a scene

Get details of a single scene from its given `{sceneId}`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scene_id** | **String** | ID of the scene. | [required] |

### Return type

[**models::GetScenes200Response**](getScenes_200_response.md)

### Authorization

[HueApplicationKey](../README.md#HueApplicationKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_scenes

> models::GetScenes200Response get_scenes()
List scenes

List all available scenes

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetScenes200Response**](getScenes_200_response.md)

### Authorization

[HueApplicationKey](../README.md#HueApplicationKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_scene

> models::UpdateDevice200Response update_scene(scene_id, scene_put)
Update a scene

Update a single scene from its given `{sceneId}`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scene_id** | **String** | ID of the scene. | [required] |
**scene_put** | Option<[**ScenePut**](ScenePut.md)> |  |  |

### Return type

[**models::UpdateDevice200Response**](updateDevice_200_response.md)

### Authorization

[HueApplicationKey](../README.md#HueApplicationKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

