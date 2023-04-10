# \ExportApi

All URIs are relative to *https://api.mezmo.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**export**](ExportApi.md#export) | **GET** /v1/export | Export log lines
[**exportv2**](ExportApi.md#exportv2) | **GET** /v2/export | Export log lines



## export

> ::std::collections::HashMap<String, serde_json::Value> export(from, to, size, hosts, apps, levels, query, prefer, email, email_subject)
Export log lines

Use this method to export logs in JSON format from a logging instance.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**from** | **String** | Start time. Set as UNIX timestamp in seconds or milliseconds. | [required] |
**to** | **String** | End time. Set as UNIX timestamp in seconds or milliseconds. | [required] |
**size** | Option<**String**> | Number of log lines to include in the export. |  |
**hosts** | Option<**String**> | Comma-separated list of hosts. |  |
**apps** | Option<**String**> | Comma-separated list of applications. |  |
**levels** | Option<**String**> | Comma-separated list of log levels. |  |
**query** | Option<**String**> | Search query. |  |
**prefer** | Option<**String**> | Defines the log lines that you want to export. Valid values are head, first log lines, and tail, last log lines. If not specified, defaults to tail. |  |
**email** | Option<**String**> | Specifies the email with the downloadable link of your export. By default, the log lines are streamed. |  |
**email_subject** | Option<**String**> | Use to set the subject of the email. Use to represent a space. For example, a sample value is Export logs. |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[servicekeyAuth](../README.md#servicekeyAuth), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## exportv2

> ::std::collections::HashMap<String, serde_json::Value> exportv2(from, to, size, hosts, apps, levels, query, prefer, pagination_id)
Export log lines

Use this method to export logs in JSON format from a logging instance.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**from** | **String** | Start time (inclusive). Set as UNIX timestamp in seconds or milliseconds. | [required] |
**to** | **String** | End time (inclusive). Set as UNIX timestamp in seconds or milliseconds. | [required] |
**size** | Option<**String**> | Number of log lines to include in the export. |  |
**hosts** | Option<**String**> | Comma-separated list of hosts. |  |
**apps** | Option<**String**> | Comma-separated list of applications. |  |
**levels** | Option<**String**> | Comma-separated list of log levels. |  |
**query** | Option<**String**> | Search query. |  |
**prefer** | Option<**String**> | Defines the log lines that you want to export. Valid values are head, first log lines, and tail, last log lines. If not specified, defaults to tail. |  |
**pagination_id** | Option<**String**> | ID that indicates which page of results to be retrieved. Leave empty for the initial export request. |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[servicekeyAuth](../README.md#servicekeyAuth), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

