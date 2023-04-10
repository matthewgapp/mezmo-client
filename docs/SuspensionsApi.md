# \SuspensionsApi

All URIs are relative to *https://api.mezmo.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_v1_config_ingestion_status**](SuspensionsApi.md#get_v1_config_ingestion_status) | **GET** /v1/config/ingestion/status | Get Ingestion Status
[**post_v1_config_ingestion_resume**](SuspensionsApi.md#post_v1_config_ingestion_resume) | **POST** /v1/config/ingestion/resume | Resume Ingestion
[**post_v1_config_ingestion_suspend**](SuspensionsApi.md#post_v1_config_ingestion_suspend) | **POST** /v1/config/ingestion/suspend | Initiate Ingestion Suspension
[**post_v1_config_ingestion_suspend_confirm**](SuspensionsApi.md#post_v1_config_ingestion_suspend_confirm) | **POST** /v1/config/ingestion/suspend/confirm | Confirm Ingestion Suspension



## get_v1_config_ingestion_status

> crate::models::GetV1ConfigIngestionStatus200Response get_v1_config_ingestion_status()
Get Ingestion Status

Retrieve the status of ingestion

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GetV1ConfigIngestionStatus200Response**](get_v1_config_ingestion_status_200_response.md)

### Authorization

[servicekeyAuth](../README.md#servicekeyAuth), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_v1_config_ingestion_resume

> crate::models::PostV1ConfigIngestionSuspendConfirm200Response post_v1_config_ingestion_resume()
Resume Ingestion

Resumes ingestion if it has been previously suspended

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::PostV1ConfigIngestionSuspendConfirm200Response**](post_v1_config_ingestion_suspend_confirm_200_response.md)

### Authorization

[servicekeyAuth](../README.md#servicekeyAuth), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_v1_config_ingestion_suspend

> crate::models::PostV1ConfigIngestionSuspend200Response post_v1_config_ingestion_suspend()
Initiate Ingestion Suspension

First step in suspending ingestion for the instance. Returns a token for use in /suspend/confirm to actually stop ingestion

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::PostV1ConfigIngestionSuspend200Response**](post_v1_config_ingestion_suspend_200_response.md)

### Authorization

[servicekeyAuth](../README.md#servicekeyAuth), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_v1_config_ingestion_suspend_confirm

> crate::models::PostV1ConfigIngestionSuspendConfirm200Response post_v1_config_ingestion_suspend_confirm(post_v1_config_ingestion_suspend200_response)
Confirm Ingestion Suspension

After /suspend is called, a token is returned for use with this route as a confirmation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**post_v1_config_ingestion_suspend200_response** | [**PostV1ConfigIngestionSuspend200Response**](PostV1ConfigIngestionSuspend200Response.md) | Request parameters | [required] |

### Return type

[**crate::models::PostV1ConfigIngestionSuspendConfirm200Response**](post_v1_config_ingestion_suspend_confirm_200_response.md)

### Authorization

[servicekeyAuth](../README.md#servicekeyAuth), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

