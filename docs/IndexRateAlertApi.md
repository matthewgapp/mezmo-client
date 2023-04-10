# \IndexRateAlertApi

All URIs are relative to *https://api.mezmo.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_index_rate_alert**](IndexRateAlertApi.md#get_index_rate_alert) | **GET** /v1/config/index-rate | Get Index Rate Alert
[**update_index_rate_alert**](IndexRateAlertApi.md#update_index_rate_alert) | **PUT** /v1/config/index-rate | Update Index Rate Alert



## get_index_rate_alert

> crate::models::IndexRateAlertConfigResponse get_index_rate_alert()
Get Index Rate Alert

Gets the configuration for an Index Rate Alert

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::IndexRateAlertConfigResponse**](indexRateAlertConfigResponse.md)

### Authorization

[servicekeyAuth](../README.md#servicekeyAuth), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_index_rate_alert

> crate::models::IndexRateAlertConfigResponse update_index_rate_alert(config_request)
Update Index Rate Alert

Use this method to update an index rate alert. You can change the alert's configuration details.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**config_request** | [**IndexRateAlertConfigResponse**](IndexRateAlertConfigResponse.md) | Request parameters | [required] |

### Return type

[**crate::models::IndexRateAlertConfigResponse**](indexRateAlertConfigResponse.md)

### Authorization

[servicekeyAuth](../README.md#servicekeyAuth), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

