# \KeysApi

All URIs are relative to *https://api.mezmo.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_key**](KeysApi.md#create_key) | **POST** /v1/config/keys | Create a new key
[**delete_key**](KeysApi.md#delete_key) | **DELETE** /v1/config/keys/{id} | Delete a specific key
[**get_key**](KeysApi.md#get_key) | **GET** /v1/config/keys/{id} | Retrieve a specific key
[**list_key**](KeysApi.md#list_key) | **GET** /v1/config/keys | Retrieve a list of all the keys
[**put_key**](KeysApi.md#put_key) | **PUT** /v1/config/keys/{id} | Update one or more fields of a key resource



## create_key

> crate::models::KeyResource create_key(r#type, keys_put)
Create a new key

Create a new key of the type specified by the `type` parameter. If a name is not supplied, one will be auto generated.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**r#type** | **String** | The type of key to be created | [required] |
**keys_put** | Option<[**KeysPut**](KeysPut.md)> | Modifiable fields can be set in the request body. |  |

### Return type

[**crate::models::KeyResource**](keyResource.md)

### Authorization

[servicekeyAuth](../README.md#servicekeyAuth), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_key

> crate::models::DeleteKey200Response delete_key(id)
Delete a specific key

Delete a specific key by its `id`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The id of the key to delete | [required] |

### Return type

[**crate::models::DeleteKey200Response**](delete_key_200_response.md)

### Authorization

[servicekeyAuth](../README.md#servicekeyAuth), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_key

> crate::models::KeyResource get_key(id)
Retrieve a specific key

Retrieves a specific key by its `id`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The id of the key to retrieve | [required] |

### Return type

[**crate::models::KeyResource**](keyResource.md)

### Authorization

[servicekeyAuth](../README.md#servicekeyAuth), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_key

> Vec<crate::models::KeyResource> list_key(r#type)
Retrieve a list of all the keys

Retrieve a list of all the keys for an account. Supported key types include `ingestion` and `service`. Use the `type` parameter to filter which type of keys to retrieve.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**r#type** | Option<**String**> | The type of key to retrieve. Defaults to \"all\" |  |

### Return type

[**Vec<crate::models::KeyResource>**](keyResource.md)

### Authorization

[servicekeyAuth](../README.md#servicekeyAuth), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_key

> crate::models::KeyResource put_key(id, keys_put)
Update one or more fields of a key resource

Updates all of the fields described in the request body of the specified key. Fields left out of the body will remain unaffected.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The id of the key to update | [required] |
**keys_put** | [**KeysPut**](KeysPut.md) |  | [required] |

### Return type

[**crate::models::KeyResource**](keyResource.md)

### Authorization

[servicekeyAuth](../README.md#servicekeyAuth), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

