# \MembersApi

All URIs are relative to *https://api.mezmo.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_member**](MembersApi.md#create_member) | **POST** /v1/config/members | Creates a new member
[**delete_member**](MembersApi.md#delete_member) | **DELETE** /v1/config/members/{email} | Delete a member
[**get_member**](MembersApi.md#get_member) | **GET** /v1/config/members/{email} | Retrieve a member
[**list_members**](MembersApi.md#list_members) | **GET** /v1/config/members | Lists all members
[**put_member**](MembersApi.md#put_member) | **PUT** /v1/config/members/{email} | Update a member



## create_member

> crate::models::MemberResource create_member(member_resource)
Creates a new member

Create a new user and add to the team by providing 'email', 'role' and an optional list of RBAC groups. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**member_resource** | [**MemberResource**](MemberResource.md) | Request parameters. | [required] |

### Return type

[**crate::models::MemberResource**](memberResource.md)

### Authorization

[servicekeyAuth](../README.md#servicekeyAuth), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_member

> crate::models::DeleteMember200Response delete_member(email)
Delete a member

Removes a member from the organization. This does not delete the user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email** | **String** |  | [required] |

### Return type

[**crate::models::DeleteMember200Response**](delete_member_200_response.md)

### Authorization

[servicekeyAuth](../README.md#servicekeyAuth), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_member

> crate::models::MemberResource get_member(email)
Retrieve a member

Returns the role and groups of the specified member.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email** | **String** |  | [required] |

### Return type

[**crate::models::MemberResource**](memberResource.md)

### Authorization

[servicekeyAuth](../README.md#servicekeyAuth), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_members

> Vec<crate::models::MemberResource> list_members()
Lists all members

Retrieves all of the current members in an organization and returns their email, role, and groups.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::MemberResource>**](memberResource.md)

### Authorization

[servicekeyAuth](../README.md#servicekeyAuth), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_member

> crate::models::MemberResource put_member(email, members_put)
Update a member

Updates all the fields described in the request body of the specified member. Creates a member if the member doesn't exist already. Fields left out of the body will remain unaffected.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email** | **String** |  | [required] |
**members_put** | [**MembersPut**](MembersPut.md) |  | [required] |

### Return type

[**crate::models::MemberResource**](memberResource.md)

### Authorization

[servicekeyAuth](../README.md#servicekeyAuth), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

