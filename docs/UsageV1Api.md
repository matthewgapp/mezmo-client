# \UsageV1Api

All URIs are relative to *https://api.mezmo.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_usage_app**](UsageV1Api.md#get_usage_app) | **GET** /v1/usage/apps/{name} | Get Usage By App
[**get_usage_host**](UsageV1Api.md#get_usage_host) | **GET** /v1/usage/hosts/{name} | Get Usage by Host
[**get_usage_tag**](UsageV1Api.md#get_usage_tag) | **GET** /v1/usage/tags/{name} | Get Usage by Tag
[**list_usage_app**](UsageV1Api.md#list_usage_app) | **GET** /v1/usage/apps | List Usage By App
[**list_usage_host**](UsageV1Api.md#list_usage_host) | **GET** /v1/usage/hosts | List Usage by Host
[**list_usage_tag**](UsageV1Api.md#list_usage_tag) | **GET** /v1/usage/tags | List Usage by Tag



## get_usage_app

> ::std::collections::HashMap<String, serde_json::Value> get_usage_app(name, from, to)
Get Usage By App

Gets the aggregated usage information for an app matching the name provided as a path parameter, during a time period. Returns null when not found.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | The name of the app from which to get the aggregated usage data | [required] |
**from** | **i32** | Start date. Set as UNIX timestamp in seconds. | [required] |
**to** | **i32** | End date. Set as UNIX timestamp in seconds. | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[servicekeyAuth](../README.md#servicekeyAuth), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_usage_host

> ::std::collections::HashMap<String, serde_json::Value> get_usage_host(name, from, to)
Get Usage by Host

Gets the aggregated usage information for a host matching the name provided as a path parameter, during a time period. Returns null when not found.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | The name of the app from which to get the aggregated usage data | [required] |
**from** | **i32** | Start date. Set as UNIX timestamp in seconds. | [required] |
**to** | **i32** | End date. Set as UNIX timestamp in seconds. | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[servicekeyAuth](../README.md#servicekeyAuth), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_usage_tag

> ::std::collections::HashMap<String, serde_json::Value> get_usage_tag(name, from, to)
Get Usage by Tag

Gets the aggregated usage information for a tag matching the name provided as a path parameter, during a time period. Returns null when not found.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | The name of the app from which to get the aggregated usage data | [required] |
**from** | **i32** | Start date. Set as UNIX timestamp in seconds. | [required] |
**to** | **i32** | End date. Set as UNIX timestamp in seconds. | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[servicekeyAuth](../README.md#servicekeyAuth), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_usage_app

> ::std::collections::HashMap<String, serde_json::Value> list_usage_app(from, to, limit)
List Usage By App

Lists aggregated usage information for all apps during a time period.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**from** | **i32** | Start date. Set as UNIX timestamp in seconds. | [required] |
**to** | **i32** | End date. Set as UNIX timestamp in seconds. | [required] |
**limit** | Option<**i32**> | Maximum amount of apps to retrieve. |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[servicekeyAuth](../README.md#servicekeyAuth), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_usage_host

> ::std::collections::HashMap<String, serde_json::Value> list_usage_host(from, to, limit)
List Usage by Host

Lists aggregated usage information for all hosts during a time period.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**from** | **i32** | Start date. Set as UNIX timestamp in seconds. | [required] |
**to** | **i32** | End date. Set as UNIX timestamp in seconds. | [required] |
**limit** | Option<**i32**> | Maximum amount of apps to retrieve. |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[servicekeyAuth](../README.md#servicekeyAuth), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_usage_tag

> ::std::collections::HashMap<String, serde_json::Value> list_usage_tag(from, to, limit)
List Usage by Tag

Lists aggregated usage information for all tags during a time period.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**from** | **i32** | Start date. Set as UNIX timestamp in seconds. | [required] |
**to** | **i32** | End date. Set as UNIX timestamp in seconds. | [required] |
**limit** | Option<**i32**> | Maximum amount of apps to retrieve. |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[servicekeyAuth](../README.md#servicekeyAuth), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

