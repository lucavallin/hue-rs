# \MotionApi

All URIs are relative to *https://192.168.1.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_motion_sensor**](MotionApi.md#get_motion_sensor) | **GET** /clip/v2/resource/motion/{motionId} | Get motion sensor.
[**get_motion_sensors**](MotionApi.md#get_motion_sensors) | **GET** /clip/v2/resource/motion | List motion sensors.
[**update_motion_sensor**](MotionApi.md#update_motion_sensor) | **PUT** /clip/v2/resource/motion/{motionId} | Update Motion Sensor



## get_motion_sensor

> models::GetMotionSensors200Response get_motion_sensor(motion_id)
Get motion sensor.

Get details of a single motion sensor from its given `{motionId}`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**motion_id** | **String** | ID of the motion sensor | [required] |

### Return type

[**models::GetMotionSensors200Response**](getMotionSensors_200_response.md)

### Authorization

[HueApplicationKey](../README.md#HueApplicationKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_motion_sensors

> models::GetMotionSensors200Response get_motion_sensors()
List motion sensors.

List all available motion sensors.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetMotionSensors200Response**](getMotionSensors_200_response.md)

### Authorization

[HueApplicationKey](../README.md#HueApplicationKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_motion_sensor

> models::UpdateDevice200Response update_motion_sensor(motion_id, motion_put)
Update Motion Sensor

Update a single motion sensor from its given `{motionId}`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**motion_id** | **String** | Id of the motion sensor | [required] |
**motion_put** | Option<[**MotionPut**](MotionPut.md)> |  |  |

### Return type

[**models::UpdateDevice200Response**](updateDevice_200_response.md)

### Authorization

[HueApplicationKey](../README.md#HueApplicationKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

