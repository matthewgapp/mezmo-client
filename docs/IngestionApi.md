# \IngestionApi

All URIs are relative to *https://api.mezmo.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**ingest**](IngestionApi.md#ingest) | **POST** /logs/ingest | Send Log Lines



## ingest

> ::std::collections::HashMap<String, serde_json::Value> ingest(hostname, ingestlines, mac, ip, now, tags)
Send Log Lines

Use this method to send logs to a logging instance.  ### Authentication You can find instructions on authentication [here](/log-analysis-api/ref#authentication).  Ingestion is similar to the other APIs, but instead of `servicekey` you will use `apikey` if you are using the header style authentication.  ### Metadata Meta is a field reserved for custom information associated with a log line. To add  metadata to an API call, specify the `meta` field under the lines object. Metadata can be viewed inside that line's context  WARNING: If inconsistent value types are used, that line's metadata, will not be parsed. For example, if a line is passed with a meta object, such as `meta.myfield` of type String, any subsequent lines with `meta.myfield` must have a String as the value type for `meta.myfield`. > Please be aware of [service limits](https://docs.mezmo.com/docs/Mezmo-ingestion-service-limits) > on this endpoint 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**hostname** | **String** | Host name of the source. | [required] |
**ingestlines** | [**Ingestlines**](Ingestlines.md) | You can send multiple log lines in a request. | [required] |
**mac** | Option<**String**> | The network mac address of the host computer. |  |
**ip** | Option<**String**> | The local IP address of the host computer. |  |
**now** | Option<**String**> | The source UNIX timestamp in milliseconds at the time of the request. Used to calculate time drift. |  |
**tags** | Option<**String**> | Tags that are used to dynamically group hosts. |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[apikeyAuth](../README.md#apikeyAuth), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

