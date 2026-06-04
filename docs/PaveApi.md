# \PaveApi

All URIs are relative to *https://api.quiltt.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**v1_remote_pave_profiles_get**](PaveApi.md#v1_remote_pave_profiles_get) | **GET** /v1/remote/pave/profiles | List all Profiles with Pave Data
[**v1_remote_pave_profiles_profile_id_get**](PaveApi.md#v1_remote_pave_profiles_profile_id_get) | **GET** /v1/remote/pave/profiles/{profileId} | Retrieve Profile Data
[**v1_remote_pave_transactions_get**](PaveApi.md#v1_remote_pave_transactions_get) | **GET** /v1/remote/pave/transactions | List all Transactions with Pave Data
[**v1_remote_pave_transactions_transaction_id_get**](PaveApi.md#v1_remote_pave_transactions_transaction_id_get) | **GET** /v1/remote/pave/transactions/{transactionId} | Retrieve Transaction Data



## v1_remote_pave_profiles_get

> Vec<models::RemoteProfile> v1_remote_pave_profiles_get(page, limit)
List all Profiles with Pave Data

Returns a paginated list of Profiles that have Pave Data

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | Page to retrieve |  |[default to 1]
**limit** | Option<**i32**> | Number of items per page |  |[default to 25]

### Return type

[**Vec<models::RemoteProfile>**](RemoteProfile.md)

### Authorization

[APISecretBearerAuth](../README.md#APISecretBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_remote_pave_profiles_profile_id_get

> models::RemotePaveProfile v1_remote_pave_profiles_profile_id_get(profile_id)
Retrieve Profile Data

Retrieve Pave Data by Profile ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile_id** | **String** |  | [required] |

### Return type

[**models::RemotePaveProfile**](RemotePaveProfile.md)

### Authorization

[APISecretBearerAuth](../README.md#APISecretBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_remote_pave_transactions_get

> Vec<models::RemoteTransaction> v1_remote_pave_transactions_get(page, limit)
List all Transactions with Pave Data

Returns a paginated list of Transactions that have Pave Data

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | Page to retrieve |  |[default to 1]
**limit** | Option<**i32**> | Number of items per page |  |[default to 25]

### Return type

[**Vec<models::RemoteTransaction>**](RemoteTransaction.md)

### Authorization

[APISecretBearerAuth](../README.md#APISecretBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_remote_pave_transactions_transaction_id_get

> models::RemotePaveTransaction v1_remote_pave_transactions_transaction_id_get(transaction_id)
Retrieve Transaction Data

Retrieve Pave Data by Transaction ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**transaction_id** | **String** |  | [required] |

### Return type

[**models::RemotePaveTransaction**](RemotePaveTransaction.md)

### Authorization

[APISecretBearerAuth](../README.md#APISecretBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

