# \ExclusionsApi

All URIs are relative to *https://api.mezmo.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_v1_config_exclusions_single**](ExclusionsApi.md#delete_v1_config_exclusions_single) | **DELETE** /v1/config/ingestion/exclusions/{id} | Delete Exclusion Rule
[**get_v1_config_exclusions**](ExclusionsApi.md#get_v1_config_exclusions) | **GET** /v1/config/ingestion/exclusions | List Exclusion Rules
[**get_v1_config_exclusions_single**](ExclusionsApi.md#get_v1_config_exclusions_single) | **GET** /v1/config/ingestion/exclusions/{id} | Get Exclusion Rule
[**patch_v1_config_exclusions_single**](ExclusionsApi.md#patch_v1_config_exclusions_single) | **PATCH** /v1/config/ingestion/exclusions/{id} | Update Exclusion Rule
[**post_v1_config_exclusions**](ExclusionsApi.md#post_v1_config_exclusions) | **POST** /v1/config/ingestion/exclusions | Create Exclusion Rule



## delete_v1_config_exclusions_single

> delete_v1_config_exclusions_single(id)
Delete Exclusion Rule

Use this method to delete an exclusion rule.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID of an exclusion rule. | [required] |

### Return type

 (empty response body)

### Authorization

[servicekeyAuth](../README.md#servicekeyAuth), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_v1_config_exclusions

> Vec<crate::models::StreamExclusionRequest> get_v1_config_exclusions()
List Exclusion Rules

Returns a list of exclusion rules. Note: This does not return usage quota rules.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::StreamExclusionRequest>**](streamExclusionRequest.md)

### Authorization

[servicekeyAuth](../README.md#servicekeyAuth), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_v1_config_exclusions_single

> Vec<crate::models::StreamExclusionRequest> get_v1_config_exclusions_single(id)
Get Exclusion Rule

Returns an exclusion rule with the id specified.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID of an exclusion rule. | [required] |

### Return type

[**Vec<crate::models::StreamExclusionRequest>**](streamExclusionRequest.md)

### Authorization

[servicekeyAuth](../README.md#servicekeyAuth), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_v1_config_exclusions_single

> crate::models::StreamExclusionRequest patch_v1_config_exclusions_single(id, stream_exclusion_request)
Update Exclusion Rule

Updates an existing exclusion rule

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID of an exclusion rule. | [required] |
**stream_exclusion_request** | [**StreamExclusionRequest**](StreamExclusionRequest.md) | Request parameters | [required] |

### Return type

[**crate::models::StreamExclusionRequest**](streamExclusionRequest.md)

### Authorization

[servicekeyAuth](../README.md#servicekeyAuth), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_v1_config_exclusions

> crate::models::StreamExclusionRequest post_v1_config_exclusions(stream_exclusion_request)
Create Exclusion Rule

Create a new ingestion exclusion rule to help reduce log volume

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stream_exclusion_request** | [**StreamExclusionRequest**](StreamExclusionRequest.md) | Request parameters | [required] |

### Return type

[**crate::models::StreamExclusionRequest**](streamExclusionRequest.md)

### Authorization

[servicekeyAuth](../README.md#servicekeyAuth), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

