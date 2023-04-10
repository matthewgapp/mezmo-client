# \ArchivingApi

All URIs are relative to *https://api.mezmo.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_v1_config_archiving**](ArchivingApi.md#delete_v1_config_archiving) | **DELETE** /v1/config/archiving | Delete Archive Configuration
[**get_v1_config_archiving**](ArchivingApi.md#get_v1_config_archiving) | **GET** /v1/config/archiving | Get Archive Configuration
[**post_v1_config_archiving**](ArchivingApi.md#post_v1_config_archiving) | **POST** /v1/config/archiving | Create Archive Configuration
[**put_v1_config_archiving**](ArchivingApi.md#put_v1_config_archiving) | **PUT** /v1/config/archiving | Update Archive Configuration



## delete_v1_config_archiving

> delete_v1_config_archiving()
Delete Archive Configuration

Use this method to delete the archiving configuration.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[servicekeyAuth](../README.md#servicekeyAuth), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_v1_config_archiving

> crate::models::ArchivingConfigRequest get_v1_config_archiving()
Get Archive Configuration

Use this method to get an existing archiving configuration.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ArchivingConfigRequest**](archivingConfigRequest.md)

### Authorization

[servicekeyAuth](../README.md#servicekeyAuth), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_v1_config_archiving

> crate::models::ArchivingConfigRequest post_v1_config_archiving(config_request)
Create Archive Configuration

Use this method to configure archiving for an instance. Only one archiving configuration may exist at any time.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**config_request** | [**ArchivingConfigRequest**](ArchivingConfigRequest.md) | Request parameters | [required] |

### Return type

[**crate::models::ArchivingConfigRequest**](archivingConfigRequest.md)

### Authorization

[servicekeyAuth](../README.md#servicekeyAuth), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_v1_config_archiving

> crate::models::ArchivingConfigRequest put_v1_config_archiving(config_request)
Update Archive Configuration

Use this method to update an existing archiving configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**config_request** | [**ArchivingConfigRequest**](ArchivingConfigRequest.md) | Request parameters | [required] |

### Return type

[**crate::models::ArchivingConfigRequest**](archivingConfigRequest.md)

### Authorization

[servicekeyAuth](../README.md#servicekeyAuth), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

