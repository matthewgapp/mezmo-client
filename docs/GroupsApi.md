# \GroupsApi

All URIs are relative to *https://api.mezmo.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_group**](GroupsApi.md#create_group) | **POST** /v1/config/groups | Create Group
[**delete_group**](GroupsApi.md#delete_group) | **DELETE** /v1/config/groups/{groupId} | Delete Group
[**list_group**](GroupsApi.md#list_group) | **GET** /v1/config/groups | List Groups
[**read_group**](GroupsApi.md#read_group) | **GET** /v1/config/groups/{groupId} | Get Group
[**update_group**](GroupsApi.md#update_group) | **PATCH** /v1/config/groups/{groupId} | Update Group



## create_group

> ::std::collections::HashMap<String, serde_json::Value> create_group(servicekey, groups_request)
Create Group

Use this method to create a log group where you can scope the data that is accessible by members in that group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**servicekey** | **String** | A provisioned service key for your account. | [required] |
**groups_request** | [**GroupsRequest**](GroupsRequest.md) | Request parameters | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[servicekeyAuth](../README.md#servicekeyAuth), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_group

> ::std::collections::HashMap<String, serde_json::Value> delete_group(servicekey, group_id)
Delete Group

Use this method to list the log groups.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**servicekey** | **String** | A provisioned service key for your account. | [required] |
**group_id** | **String** | ID of a group. | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[servicekeyAuth](../README.md#servicekeyAuth), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_group

> ::std::collections::HashMap<String, serde_json::Value> list_group()
List Groups

Use this method to list the log groups.

### Parameters

This endpoint does not need any parameter.

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[servicekeyAuth](../README.md#servicekeyAuth), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_group

> ::std::collections::HashMap<String, serde_json::Value> read_group(group_id)
Get Group

Use this method to get information on a group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | ID of a group. | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[servicekeyAuth](../README.md#servicekeyAuth), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_group

> ::std::collections::HashMap<String, serde_json::Value> update_group(servicekey, group_id, groups_request)
Update Group

Use this method to modify a log group. You can change the name of the group and the access scope that defines the data that is accessible by members in that group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**servicekey** | **String** | A provisioned service key for your account. | [required] |
**group_id** | **String** | ID of a group. | [required] |
**groups_request** | [**GroupsRequest**](GroupsRequest.md) | Request parameters | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[servicekeyAuth](../README.md#servicekeyAuth), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

