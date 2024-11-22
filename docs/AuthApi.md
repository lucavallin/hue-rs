# \AuthApi

All URIs are relative to *https://192.168.1.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**authenticate**](AuthApi.md#authenticate) | **POST** /api | Authenticate



## authenticate

> Vec<models::ResponseInner> authenticate(authenticate_request)
Authenticate

Authenticate to retrieve the HUE application key. Requires to go and press the button on the bridge

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authenticate_request** | Option<[**AuthenticateRequest**](AuthenticateRequest.md)> |  |  |

### Return type

[**Vec<models::ResponseInner>**](response_inner.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

