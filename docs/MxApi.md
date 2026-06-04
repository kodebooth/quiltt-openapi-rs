# \MxApi

All URIs are relative to *https://api.quiltt.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**v1_remote_mx_accounts_account_id_get**](MxApi.md#v1_remote_mx_accounts_account_id_get) | **GET** /v1/remote/mx/accounts/{accountId} | Retrieve Account Data
[**v1_remote_mx_accounts_get**](MxApi.md#v1_remote_mx_accounts_get) | **GET** /v1/remote/mx/accounts | List all Accounts with MX Data
[**v1_remote_mx_connections_connection_id_get**](MxApi.md#v1_remote_mx_connections_connection_id_get) | **GET** /v1/remote/mx/connections/{connectionId} | Retrieve MX Connection Data
[**v1_remote_mx_connections_get**](MxApi.md#v1_remote_mx_connections_get) | **GET** /v1/remote/mx/connections | List all Connections with MX Data
[**v1_remote_mx_transactions_get**](MxApi.md#v1_remote_mx_transactions_get) | **GET** /v1/remote/mx/transactions | List all Transactions with MX Data
[**v1_remote_mx_transactions_transaction_id_get**](MxApi.md#v1_remote_mx_transactions_transaction_id_get) | **GET** /v1/remote/mx/transactions/{transactionId} | Retrieve Transaction Data



## v1_remote_mx_accounts_account_id_get

> models::RemoteMxAccount v1_remote_mx_accounts_account_id_get(account_id)
Retrieve Account Data

Retrieve MX Data by Account ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** |  | [required] |

### Return type

[**models::RemoteMxAccount**](RemoteMXAccount.md)

### Authorization

[APISecretBearerAuth](../README.md#APISecretBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_remote_mx_accounts_get

> Vec<models::RemoteAccount> v1_remote_mx_accounts_get(page, limit)
List all Accounts with MX Data

Returns a paginated list of Accounts with MX Data

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | Page to retrieve |  |[default to 1]
**limit** | Option<**i32**> | Number of items per page |  |[default to 25]

### Return type

[**Vec<models::RemoteAccount>**](RemoteAccount.md)

### Authorization

[APISecretBearerAuth](../README.md#APISecretBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_remote_mx_connections_connection_id_get

> models::RemoteMxConnection v1_remote_mx_connections_connection_id_get(connection_id)
Retrieve MX Connection Data

Retrieve MX Data by Connection ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**connection_id** | **String** |  | [required] |

### Return type

[**models::RemoteMxConnection**](RemoteMXConnection.md)

### Authorization

[APISecretBearerAuth](../README.md#APISecretBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_remote_mx_connections_get

> Vec<models::RemoteConnection> v1_remote_mx_connections_get(page, limit)
List all Connections with MX Data

Returns a paginated list of Connections with MX Data

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | Page to retrieve |  |[default to 1]
**limit** | Option<**i32**> | Number of items per page |  |[default to 25]

### Return type

[**Vec<models::RemoteConnection>**](RemoteConnection.md)

### Authorization

[APISecretBearerAuth](../README.md#APISecretBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_remote_mx_transactions_get

> Vec<models::RemoteTransaction> v1_remote_mx_transactions_get(page, limit)
List all Transactions with MX Data

Returns a paginated list of Transactions with MX Data

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


## v1_remote_mx_transactions_transaction_id_get

> models::RemoteMxTransaction v1_remote_mx_transactions_transaction_id_get(transaction_id)
Retrieve Transaction Data

Retrieve MX Data by Transaction ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**transaction_id** | **String** |  | [required] |

### Return type

[**models::RemoteMxTransaction**](RemoteMXTransaction.md)

### Authorization

[APISecretBearerAuth](../README.md#APISecretBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

