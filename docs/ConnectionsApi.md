# \ConnectionsApi

All URIs are relative to *https://api.quiltt.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**v1_connections_connection_id_transactions_refresh_post**](ConnectionsApi.md#v1_connections_connection_id_transactions_refresh_post) | **POST** /v1/connections/{connectionId}/transactions/refresh | Trigger a Transactions Refresh
[**v1_profiles_profile_id_connections_connection_id_export_get**](ConnectionsApi.md#v1_profiles_profile_id_connections_connection_id_export_get) | **GET** /v1/profiles/{profileId}/connections/{connectionId}/export | Export Connection Details
[**v1_profiles_profile_id_connections_import_plaid_post**](ConnectionsApi.md#v1_profiles_profile_id_connections_import_plaid_post) | **POST** /v1/profiles/{profileId}/connections/import/plaid | Import from Plaid



## v1_connections_connection_id_transactions_refresh_post

> v1_connections_connection_id_transactions_refresh_post(connection_id)
Trigger a Transactions Refresh

Triggers a Transactions Refresh for a given Connection.  Successful requests trigger a transaction sync from the Connection provider and return a `202 Accepted` response.  If transactions are added, removed, or updated, you will be notified via webhook events and the changes will be available in GraphQL. If there are no changes, no webhook will be fired — this mirrors the behavior of upstream providers, which only emit events when data changes.  **This feature is currently in beta.** Please contact [Quiltt Support](https://quiltt.io/support) to have it enabled for your environment. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**connection_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[APISecretBearerAuth](../README.md#APISecretBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_profiles_profile_id_connections_connection_id_export_get

> models::ConnectionExport v1_profiles_profile_id_connections_connection_id_export_get(profile_id, connection_id)
Export Connection Details

Exports the Connection details needed to make direct API calls to the underlying provider.  Contact Quiltt Support to enable this feature. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile_id** | **String** |  | [required] |
**connection_id** | **String** |  | [required] |

### Return type

[**models::ConnectionExport**](ConnectionExport.md)

### Authorization

[APISecretBearerAuth](../README.md#APISecretBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_profiles_profile_id_connections_import_plaid_post

> models::Connection v1_profiles_profile_id_connections_import_plaid_post(profile_id, connection_import_plaid_request)
Import from Plaid

Imports a Connection from an existing Plaid record. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile_id** | **String** |  | [required] |
**connection_import_plaid_request** | Option<[**ConnectionImportPlaidRequest**](ConnectionImportPlaidRequest.md)> |  |  |

### Return type

[**models::Connection**](Connection.md)

### Authorization

[APISecretBearerAuth](../README.md#APISecretBearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

