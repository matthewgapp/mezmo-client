# \ConfigurationApi

All URIs are relative to *https://api.mezmo.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_alert**](ConfigurationApi.md#create_alert) | **POST** /v1/config/presetalert | Create Preset Alert
[**create_category**](ConfigurationApi.md#create_category) | **POST** /v1/config/categories/{type} | Create New Category
[**create_view**](ConfigurationApi.md#create_view) | **POST** /v1/config/view | Create View
[**delete_alert**](ConfigurationApi.md#delete_alert) | **DELETE** /v1/config/presetalert/{presetId} | Delete Alert
[**delete_category**](ConfigurationApi.md#delete_category) | **DELETE** /v1/config/categories/{type}/{id} | Delete Category
[**delete_view**](ConfigurationApi.md#delete_view) | **DELETE** /v1/config/view/{viewId} | Delete View
[**get_alert**](ConfigurationApi.md#get_alert) | **GET** /v1/config/presetalert/{presetId} | Get Preset Alert
[**get_category**](ConfigurationApi.md#get_category) | **GET** /v1/config/categories/{type}/{id} | Get Category
[**get_view**](ConfigurationApi.md#get_view) | **GET** /v1/config/view/{viewId} | Get View
[**list_alerts**](ConfigurationApi.md#list_alerts) | **GET** /v1/config/presetalert | List Preset Alerts
[**list_categories**](ConfigurationApi.md#list_categories) | **GET** /v1/config/categories/{type} | List Categories
[**list_view**](ConfigurationApi.md#list_view) | **GET** /v1/config/view | List Views
[**update_category**](ConfigurationApi.md#update_category) | **PUT** /v1/config/categories/{type}/{id} | Update Category
[**update_preset**](ConfigurationApi.md#update_preset) | **PUT** /v1/config/presetalert/{presetId} | Update Alert
[**update_view**](ConfigurationApi.md#update_view) | **PUT** /v1/config/view/{viewId} | Update View



## create_alert

> ::std::collections::HashMap<String, serde_json::Value> create_alert(preset_request)
Create Preset Alert

Use this method to create a preset alert.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**preset_request** | [**PresetRequest**](PresetRequest.md) | Request parameters | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[servicekeyAuth](../README.md#servicekeyAuth), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_category

> ::std::collections::HashMap<String, serde_json::Value> create_category(r#type, category_name)
Create New Category

Creates a new category of this type.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**r#type** | **String** | Category type.  Must be [views, boards, screens] | [required] |
**category_name** | [**CategoryName**](CategoryName.md) | Request parameters | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[servicekeyAuth](../README.md#servicekeyAuth), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_view

> ::std::collections::HashMap<String, serde_json::Value> create_view(config_request)
Create View

Use this method to create a view and attach alerts to the view.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**config_request** | [**ConfigRequest**](ConfigRequest.md) | Request parameters | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[servicekeyAuth](../README.md#servicekeyAuth), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_alert

> ::std::collections::HashMap<String, serde_json::Value> delete_alert(preset_id)
Delete Alert

Use this method to delete a preset alert.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**preset_id** | **String** | ID of a preset alert. | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[servicekeyAuth](../README.md#servicekeyAuth), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_category

> ::std::collections::HashMap<String, serde_json::Value> delete_category(r#type, id)
Delete Category

Use this method to delete a category.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**r#type** | **String** | Category type.  Must be [views, boards, screens] | [required] |
**id** | **String** | ID of category. | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[servicekeyAuth](../README.md#servicekeyAuth), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_view

> ::std::collections::HashMap<String, serde_json::Value> delete_view(view_id)
Delete View

Use this method to delete a view and any attached alerts.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**view_id** | **String** | ID of a view. | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[servicekeyAuth](../README.md#servicekeyAuth), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_alert

> ::std::collections::HashMap<String, serde_json::Value> get_alert(preset_id)
Get Preset Alert

Get a specific preset alert

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**preset_id** | **String** | ID of a preset alert. | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[servicekeyAuth](../README.md#servicekeyAuth), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_category

> ::std::collections::HashMap<String, serde_json::Value> get_category(r#type, id)
Get Category

Get a specific category

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**r#type** | **String** | Category type.  Must be [views, boards, screens] | [required] |
**id** | **String** | ID of category. | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[servicekeyAuth](../README.md#servicekeyAuth), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_view

> ::std::collections::HashMap<String, serde_json::Value> get_view(view_id)
Get View

Gets the configuration for a specific view.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**view_id** | **String** | ID of a view. | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[servicekeyAuth](../README.md#servicekeyAuth), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_alerts

> ::std::collections::HashMap<String, serde_json::Value> list_alerts()
List Preset Alerts

Returns all Preset Alerts in the account.

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


## list_categories

> ::std::collections::HashMap<String, serde_json::Value> list_categories(r#type)
List Categories

Returns all Categories of a type.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**r#type** | **String** | Category type.  Must be [views, boards, screens] | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[servicekeyAuth](../README.md#servicekeyAuth), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_view

> ::std::collections::HashMap<String, serde_json::Value> list_view()
List Views

Get a list of views.

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


## update_category

> ::std::collections::HashMap<String, serde_json::Value> update_category(r#type, id, category_name)
Update Category

Update a specific category.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**r#type** | **String** | Category type.  Must be [views, boards, screens] | [required] |
**id** | **String** | ID of category. | [required] |
**category_name** | [**CategoryName**](CategoryName.md) | Request parameters | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[servicekeyAuth](../README.md#servicekeyAuth), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_preset

> ::std::collections::HashMap<String, serde_json::Value> update_preset(preset_id, preset_request)
Update Alert

Update a specific preset alert.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**preset_id** | **String** | ID of a preset alert. | [required] |
**preset_request** | [**PresetRequest**](PresetRequest.md) | Request parameters | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[servicekeyAuth](../README.md#servicekeyAuth), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_view

> ::std::collections::HashMap<String, serde_json::Value> update_view(view_id, config_request)
Update View

Use this method to update a view. You can change the view configuration details; add or remove view specific alerts; or attach and detach preset alerts.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**view_id** | **String** | ID of a view. | [required] |
**config_request** | [**ConfigRequest**](ConfigRequest.md) | Request parameters | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[servicekeyAuth](../README.md#servicekeyAuth), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

