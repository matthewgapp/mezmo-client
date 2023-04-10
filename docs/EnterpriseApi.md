# \EnterpriseApi

All URIs are relative to *https://api.mezmo.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_enterprise_key**](EnterpriseApi.md#create_enterprise_key) | **POST** /v1/enterprise/keys | Generate a new Key for the Enterprise
[**create_org**](EnterpriseApi.md#create_org) | **POST** /v1/enterprise/account | Attach a child organization to an enterprise
[**delete_entperise_key**](EnterpriseApi.md#delete_entperise_key) | **DELETE** /v1/enterprise/keys/{id} | Delete an Enterprise ServiceKey
[**delete_org**](EnterpriseApi.md#delete_org) | **DELETE** /v1/enterprise/account/{accountId} | Detach a child org from Enterprise
[**get_entperise_key**](EnterpriseApi.md#get_entperise_key) | **GET** /v1/enterprise/keys/{id} | Retrieve a specific entperise key
[**get_org**](EnterpriseApi.md#get_org) | **GET** /v1/enterprise/account/{accountId} | Get details of a child organization
[**list_enterprise_keys**](EnterpriseApi.md#list_enterprise_keys) | **GET** /v1/enterprise/keys | List all enterprise keys
[**list_orgs**](EnterpriseApi.md#list_orgs) | **GET** /v1/enterprise/account | List all accounts
[**put_org**](EnterpriseApi.md#put_org) | **PUT** /v1/enterprise/account/{accountId} | Update Child Organization Contract



## create_enterprise_key

> crate::models::KeyResource create_enterprise_key()
Generate a new Key for the Enterprise

Generate a new ServiceKey for the enterprise 

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::KeyResource**](keyResource.md)

### Authorization

[servicekeyAuth](../README.md#servicekeyAuth), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_org

> crate::models::EnterpriseResource create_org(enterprise_put)
Attach a child organization to an enterprise

Attaches an existing organization to an enterprise. The `retention` and `owner` field are optional, and will be defaulted if not specified.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**enterprise_put** | [**EnterprisePut**](EnterprisePut.md) | Request parameters. | [required] |

### Return type

[**crate::models::EnterpriseResource**](enterpriseResource.md)

### Authorization

[servicekeyAuth](../README.md#servicekeyAuth), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_entperise_key

> crate::models::DeleteEntperiseKey200Response delete_entperise_key(id)
Delete an Enterprise ServiceKey

Delete an Enterprise ServiceKey from list of keys for the Enterprise

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::DeleteEntperiseKey200Response**](delete_entperiseKey_200_response.md)

### Authorization

[servicekeyAuth](../README.md#servicekeyAuth), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_org

> crate::models::DeleteOrg200Response delete_org(account_id)
Detach a child org from Enterprise

Detaches a child organization from the enterprise. The child organization is not deleted and can continue to work afterwards.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** |  | [required] |

### Return type

[**crate::models::DeleteOrg200Response**](delete_org_200_response.md)

### Authorization

[servicekeyAuth](../README.md#servicekeyAuth), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_entperise_key

> crate::models::KeyResource get_entperise_key(id)
Retrieve a specific entperise key

Retrieves a specific enterprise key by its `id`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::KeyResource**](keyResource.md)

### Authorization

[servicekeyAuth](../README.md#servicekeyAuth), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_org

> crate::models::EnterpriseResource get_org(account_id)
Get details of a child organization

Retrieves a specific child organization by its id. If the organization does not belong to the enterprise, a 404 error is returned.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** |  | [required] |

### Return type

[**crate::models::EnterpriseResource**](enterpriseResource.md)

### Authorization

[servicekeyAuth](../README.md#servicekeyAuth), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_enterprise_keys

> Vec<crate::models::KeyResource> list_enterprise_keys()
List all enterprise keys

Retrieves all keys of the enterprise

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::KeyResource>**](keyResource.md)

### Authorization

[servicekeyAuth](../README.md#servicekeyAuth), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_orgs

> Vec<crate::models::EnterpriseResource> list_orgs()
List all accounts

Retrieves a list of all child organizations that belong to the enterprise.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::EnterpriseResource>**](enterpriseResource.md)

### Authorization

[servicekeyAuth](../README.md#servicekeyAuth), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_org

> crate::models::EnterpriseResource put_org(account_id, enterprise_put)
Update Child Organization Contract

Makes an update to the specified child organization. Used to modify the current retention with the `retention` field or the current owner with the `owner` field.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** |  | [required] |
**enterprise_put** | [**EnterprisePut**](EnterprisePut.md) |  | [required] |

### Return type

[**crate::models::EnterpriseResource**](enterpriseResource.md)

### Authorization

[servicekeyAuth](../README.md#servicekeyAuth), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

