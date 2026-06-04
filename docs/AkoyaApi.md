# \AkoyaApi

All URIs are relative to *https://api.quiltt.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**v1_remote_akoya_accounts_account_id_get**](AkoyaApi.md#v1_remote_akoya_accounts_account_id_get) | **GET** /v1/remote/akoya/accounts/{accountId} | Retrieve Account data
[**v1_remote_akoya_accounts_get**](AkoyaApi.md#v1_remote_akoya_accounts_get) | **GET** /v1/remote/akoya/accounts | List all Accounts with Akoya Data
[**v1_remote_akoya_connections_connection_id_get**](AkoyaApi.md#v1_remote_akoya_connections_connection_id_get) | **GET** /v1/remote/akoya/connections/{connectionId} | Retrieve Akoya Connection Data
[**v1_remote_akoya_connections_get**](AkoyaApi.md#v1_remote_akoya_connections_get) | **GET** /v1/remote/akoya/connections | List all Connections with Akoya Data
[**v1_remote_akoya_holdings_get**](AkoyaApi.md#v1_remote_akoya_holdings_get) | **GET** /v1/remote/akoya/holdings | List all Holdings with Akoya Data
[**v1_remote_akoya_holdings_holding_id_get**](AkoyaApi.md#v1_remote_akoya_holdings_holding_id_get) | **GET** /v1/remote/akoya/holdings/{holdingId} | Retrieve Holding Data
[**v1_remote_akoya_transactions_get**](AkoyaApi.md#v1_remote_akoya_transactions_get) | **GET** /v1/remote/akoya/transactions | List all Transactions with Akoya Data
[**v1_remote_akoya_transactions_transaction_id_get**](AkoyaApi.md#v1_remote_akoya_transactions_transaction_id_get) | **GET** /v1/remote/akoya/transactions/{transactionId} | Retrieve Transaction Data



## v1_remote_akoya_accounts_account_id_get

> models::RemoteAkoyaAccount v1_remote_akoya_accounts_account_id_get(account_id)
Retrieve Account data

Retrieve Akoya Data by Account ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** |  | [required] |

### Return type

[**models::RemoteAkoyaAccount**](RemoteAkoyaAccount.md)

### Authorization

[APISecretBearerAuth](../README.md#APISecretBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_remote_akoya_accounts_get

> Vec<models::RemoteAccount> v1_remote_akoya_accounts_get(page, limit)
List all Accounts with Akoya Data

Returns a paginated list of Accounts with Akoya Data

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


## v1_remote_akoya_connections_connection_id_get

> models::RemoteAkoyaConnection v1_remote_akoya_connections_connection_id_get(connection_id)
Retrieve Akoya Connection Data

Retrieve Akoya Data by Connection ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**connection_id** | **String** |  | [required] |

### Return type

[**models::RemoteAkoyaConnection**](RemoteAkoyaConnection.md)

### Authorization

[APISecretBearerAuth](../README.md#APISecretBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_remote_akoya_connections_get

> Vec<models::RemoteConnection> v1_remote_akoya_connections_get(page, limit)
List all Connections with Akoya Data

Returns a paginated list of Connections with Akoya Data

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


## v1_remote_akoya_holdings_get

> Vec<models::RemoteHolding> v1_remote_akoya_holdings_get(page, limit)
List all Holdings with Akoya Data

Returns a paginated list of Holdings with Akoya Data

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | Page to retrieve |  |[default to 1]
**limit** | Option<**i32**> | Number of items per page |  |[default to 25]

### Return type

[**Vec<models::RemoteHolding>**](RemoteHolding.md)

### Authorization

[APISecretBearerAuth](../README.md#APISecretBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_remote_akoya_holdings_holding_id_get

> models::RemoteAkoyaHolding v1_remote_akoya_holdings_holding_id_get(holding_id)
Retrieve Holding Data

Retrieve Akoya Data by Holding ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**holding_id** | **String** |  | [required] |

### Return type

[**models::RemoteAkoyaHolding**](RemoteAkoyaHolding.md)

### Authorization

[APISecretBearerAuth](../README.md#APISecretBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_remote_akoya_transactions_get

> Vec<models::RemoteTransaction> v1_remote_akoya_transactions_get(page, limit)
List all Transactions with Akoya Data

Returns a paginated list of Transactions with Akoya Data

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


## v1_remote_akoya_transactions_transaction_id_get

> models::RemoteAkoyaTransaction v1_remote_akoya_transactions_transaction_id_get(transaction_id)
Retrieve Transaction Data

Retrieve Akoya Data by Transaction ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**transaction_id** | **String** |  | [required] |

### Return type

[**models::RemoteAkoyaTransaction**](RemoteAkoyaTransaction.md)

### Authorization

[APISecretBearerAuth](../README.md#APISecretBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

