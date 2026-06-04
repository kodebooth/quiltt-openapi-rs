# \ProfilesApi

All URIs are relative to *https://api.quiltt.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_profile**](ProfilesApi.md#create_profile) | **POST** /v1/profiles | Create a Profile
[**delete_profile**](ProfilesApi.md#delete_profile) | **DELETE** /v1/profiles/{id} | Delete a Profile
[**get_profile_by_id**](ProfilesApi.md#get_profile_by_id) | **GET** /v1/profiles/{id} | Retrieve a Profile
[**get_profiles**](ProfilesApi.md#get_profiles) | **GET** /v1/profiles | List all Profiles
[**update_profile**](ProfilesApi.md#update_profile) | **PATCH** /v1/profiles/{id} | Update a Profile



## create_profile

> models::Profile create_profile(profile_create_request)
Create a Profile

Creates a Profile with supplied attributes

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile_create_request** | Option<[**ProfileCreateRequest**](ProfileCreateRequest.md)> |  |  |

### Return type

[**models::Profile**](Profile.md)

### Authorization

[APISecretBearerAuth](../README.md#APISecretBearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_profile

> delete_profile(id)
Delete a Profile

Permanently deletes a Profile and all its associated data. Please note that this process can take up to 15 minutes to complete. This action cannot be undone.  **What Gets Deleted:** - Profile information (name, email, phone, etc.) - All Connections and their credentials - All Accounts and their balances - All Transactions and transaction data - All Identities and identity verification data - Custom metadata associated with the Profile  **Important Notes:** - This process can take up to 15 minutes to complete - A `profile.deleted` webhook event will be sent when deletion is complete - Any Quiltt-managed Connections will be disabled 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[APISecretBearerAuth](../README.md#APISecretBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_profile_by_id

> models::Profile get_profile_by_id(id)
Retrieve a Profile

Retrieves a Profile by its ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::Profile**](Profile.md)

### Authorization

[APISecretBearerAuth](../README.md#APISecretBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_profiles

> Vec<models::Profile> get_profiles(page, limit)
List all Profiles

Returns a paginated list of all Profiles in your environment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | Page to retrieve |  |[default to 1]
**limit** | Option<**i32**> | Number of items per page |  |[default to 25]

### Return type

[**Vec<models::Profile>**](Profile.md)

### Authorization

[APISecretBearerAuth](../README.md#APISecretBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_profile

> models::Profile update_profile(id, profile_update_request)
Update a Profile

Updates Profile attributes

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**profile_update_request** | Option<[**ProfileUpdateRequest**](ProfileUpdateRequest.md)> |  |  |

### Return type

[**models::Profile**](Profile.md)

### Authorization

[APISecretBearerAuth](../README.md#APISecretBearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

