# \UsageV2Api

All URIs are relative to *https://api.mezmo.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_usage_app_v2**](UsageV2Api.md#get_usage_app_v2) | **GET** /v2/usage/apps/{name} | Get Usage By App
[**get_usage_host_v2**](UsageV2Api.md#get_usage_host_v2) | **GET** /v2/usage/hosts/{name} | Get Usage by Host
[**get_usage_tag_v2**](UsageV2Api.md#get_usage_tag_v2) | **GET** /v2/usage/tags/{name} | Get Usage by Tag
[**list_usage_app_v2**](UsageV2Api.md#list_usage_app_v2) | **GET** /v2/usage/apps | List Usage By App
[**list_usage_host_v2**](UsageV2Api.md#list_usage_host_v2) | **GET** /v2/usage/hosts | List Usage by Host
[**list_usage_tag_v2**](UsageV2Api.md#list_usage_tag_v2) | **GET** /v2/usage/tags | List Usage by Tag
[**list_usage_v2**](UsageV2Api.md#list_usage_v2) | **GET** /v2/usage | Retrieve account usage totals



## get_usage_app_v2

> crate::models::ListUsageAppV2200Response get_usage_app_v2(name, from, to, retention)
Get Usage By App

Gets the aggregated usage information for an app matching the name provided as a path parameter during a time period. The `results` will either have a singular item, or be an empty array.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | The name of the app from which to get the aggregated usage data. | [required] |
**from** | **String** | Start time. A date-time string as defined by RFC 3339 §5.6 (\"T\" is required). This date-time will be used to calculate the starting day; the exact time will not be used. | [required] |
**to** | **String** | End time. A date-time string as defined by RFC 3339 §5.6 (\"T\" is required). This date-time will be used to calculate the ending day; the exact time will not be used. | [required] |
**retention** | Option<**String**> | Limit report to apps that contribute to a variable retention pool. Value must be a string indicating the retention pool duration, e.g. `7d` for a seven day retention pool. |  |

### Return type

[**crate::models::ListUsageAppV2200Response**](list_usage_app_v2_200_response.md)

### Authorization

[servicekeyAuth](../README.md#servicekeyAuth), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_usage_host_v2

> crate::models::ListUsageHostV2200Response get_usage_host_v2(name, from, to, retention)
Get Usage by Host

Gets the aggregated usage information for a host matching the name provided as a path parameter during a time period. The `results` will either have a singular item, or be an empty array.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | The name of the host from which to get the aggregated usage data. | [required] |
**from** | **String** | Start time. A date-time string as defined by RFC 3339 §5.6 (\"T\" is required). This date-time will be used to calculate the starting day; the exact time will not be used. | [required] |
**to** | **String** | End time. A date-time string as defined by RFC 3339 §5.6 (\"T\" is required). This date-time will be used to calculate the ending day; the exact time will not be used. | [required] |
**retention** | Option<**String**> | Limit report to hosts that contribute to a variable retention pool. Value must be a string indicating the retention pool duration, e.g. `7d` for a seven day retention pool. |  |

### Return type

[**crate::models::ListUsageHostV2200Response**](list_usage_host_v2_200_response.md)

### Authorization

[servicekeyAuth](../README.md#servicekeyAuth), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_usage_tag_v2

> crate::models::ListUsageTagV2200Response get_usage_tag_v2(name, from, to, retention)
Get Usage by Tag

Gets the aggregated usage information for a tag matching the name provided as a path parameter during a time period. The `results` will either have a singular item, or be an empty array.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | The name of the tag from which to get the aggregated usage data. | [required] |
**from** | **String** | Start time. A date-time string as defined by RFC 3339 §5.6 (\"T\" is required). This date-time will be used to calculate the starting day; the exact time will not be used. | [required] |
**to** | **String** | End time. A date-time string as defined by RFC 3339 §5.6 (\"T\" is required). This date-time will be used to calculate the ending day; the exact time will not be used. | [required] |
**retention** | Option<**String**> | Limit report to tags that contribute to a variable retention pool. Value must be a string indicating the retention pool duration, e.g. `7d` for a seven day retention pool. |  |

### Return type

[**crate::models::ListUsageTagV2200Response**](list_usage_tag_v2_200_response.md)

### Authorization

[servicekeyAuth](../README.md#servicekeyAuth), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_usage_app_v2

> crate::models::ListUsageAppV2200Response list_usage_app_v2(from, to, limit, retention)
List Usage By App

Lists aggregated usage information for all apps during a time period.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**from** | **String** | Start time. A date-time string as defined by RFC 3339 §5.6 (\"T\" is required). This date-time will be used to calculate the starting day; the exact time will not be used. | [required] |
**to** | **String** | End time. A date-time string as defined by RFC 3339 §5.6 (\"T\" is required). This date-time will be used to calculate the ending day; the exact time will not be used. | [required] |
**limit** | Option<**i32**> | Maximum amount of apps to retrieve. |  |
**retention** | Option<**String**> | Limit report to apps that contribute to a variable retention pool. Value must be a string indicating the retention pool duration, e.g. `7d` for a seven day retention pool. |  |

### Return type

[**crate::models::ListUsageAppV2200Response**](list_usage_app_v2_200_response.md)

### Authorization

[servicekeyAuth](../README.md#servicekeyAuth), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_usage_host_v2

> crate::models::ListUsageHostV2200Response list_usage_host_v2(from, to, limit, retention)
List Usage by Host

Lists aggregated usage information for all hosts during a time period.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**from** | **String** | Start time. A date-time string as defined by RFC 3339 §5.6 (\"T\" is required). This date-time will be used to calculate the starting day; the exact time will not be used. | [required] |
**to** | **String** | End time. A date-time string as defined by RFC 3339 §5.6 (\"T\" is required). This date-time will be used to calculate the ending day; the exact time will not be used. | [required] |
**limit** | Option<**i32**> | Maximum amount of hosts to retrieve. |  |
**retention** | Option<**String**> | Limit report to hosts that contribute to a variable retention pool. Value must be a string indicating the retention pool duration, e.g. `7d` for a seven day retention pool. |  |

### Return type

[**crate::models::ListUsageHostV2200Response**](list_usage_host_v2_200_response.md)

### Authorization

[servicekeyAuth](../README.md#servicekeyAuth), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_usage_tag_v2

> crate::models::ListUsageTagV2200Response list_usage_tag_v2(from, to, limit, retention)
List Usage by Tag

Lists aggregated usage information for all tags during a time period.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**from** | **String** | Start time. A date-time string as defined by RFC 3339 §5.6 (\"T\" is required). This date-time will be used to calculate the starting day; the exact time will not be used. | [required] |
**to** | **String** | End time. A date-time string as defined by RFC 3339 §5.6 (\"T\" is required). This date-time will be used to calculate the ending day; the exact time will not be used. | [required] |
**limit** | Option<**i32**> | Maximum amount of tags to retrieve. |  |
**retention** | Option<**String**> | Limit report to tags that contribute to a variable retention pool. Value must be a string indicating the retention pool duration, e.g. `7d` for a seven day retention pool. |  |

### Return type

[**crate::models::ListUsageTagV2200Response**](list_usage_tag_v2_200_response.md)

### Authorization

[servicekeyAuth](../README.md#servicekeyAuth), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_usage_v2

> crate::models::ListUsageV2200Response list_usage_v2(from, to)
Retrieve account usage totals

Get aggregated usage information for an account's data during a time period.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**from** | **String** | Start time. A date-time string as defined by RFC 3339 §5.6 (\"T\" is required). This date-time will be used to calculate the starting day; the exact time will not be used. | [required] |
**to** | **String** | End time. A date-time string as defined by RFC 3339 §5.6 (\"T\" is required). This date-time will be used to calculate the ending day; the exact time will not be used. | [required] |

### Return type

[**crate::models::ListUsageV2200Response**](list_usage_v2_200_response.md)

### Authorization

[servicekeyAuth](../README.md#servicekeyAuth), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

