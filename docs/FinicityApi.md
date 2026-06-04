# \FinicityApi

All URIs are relative to *https://api.quiltt.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**v1_remote_finicity_accounts_account_id_get**](FinicityApi.md#v1_remote_finicity_accounts_account_id_get) | **GET** /v1/remote/finicity/accounts/{accountId} | Retrieve Account data
[**v1_remote_finicity_accounts_get**](FinicityApi.md#v1_remote_finicity_accounts_get) | **GET** /v1/remote/finicity/accounts | List all Accounts with Finicity Data
[**v1_remote_finicity_connections_connection_id_get**](FinicityApi.md#v1_remote_finicity_connections_connection_id_get) | **GET** /v1/remote/finicity/connections/{connectionId} | Retrieve Finicity Connection Data
[**v1_remote_finicity_connections_get**](FinicityApi.md#v1_remote_finicity_connections_get) | **GET** /v1/remote/finicity/connections | List all Connections with Finicity Data
[**v1_remote_finicity_holdings_get**](FinicityApi.md#v1_remote_finicity_holdings_get) | **GET** /v1/remote/finicity/holdings | List all Holdings with Finicity Data
[**v1_remote_finicity_holdings_holding_id_get**](FinicityApi.md#v1_remote_finicity_holdings_holding_id_get) | **GET** /v1/remote/finicity/holdings/{holdingId} | Retrieve Holding Data
[**v1_remote_finicity_transactions_get**](FinicityApi.md#v1_remote_finicity_transactions_get) | **GET** /v1/remote/finicity/transactions | List all Transactions with Finicity Data
[**v1_remote_finicity_transactions_transaction_id_get**](FinicityApi.md#v1_remote_finicity_transactions_transaction_id_get) | **GET** /v1/remote/finicity/transactions/{transactionId} | Retrieve Transaction Data



## v1_remote_finicity_accounts_account_id_get

> models::RemoteFinicityAccount v1_remote_finicity_accounts_account_id_get(account_id)
Retrieve Account data

Retrieve Finicity Data by Account ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** |  | [required] |

### Return type

[**models::RemoteFinicityAccount**](RemoteFinicityAccount.md)

### Authorization

[APISecretBearerAuth](../README.md#APISecretBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_remote_finicity_accounts_get

> Vec<models::RemoteAccount> v1_remote_finicity_accounts_get(page, limit)
List all Accounts with Finicity Data

Returns a paginated list of Accounts with Finicity Data

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


## v1_remote_finicity_connections_connection_id_get

> models::RemoteFinicityConnection v1_remote_finicity_connections_connection_id_get(connection_id)
Retrieve Finicity Connection Data

Retrieve Finicity Data by Connection ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**connection_id** | **String** |  | [required] |

### Return type

[**models::RemoteFinicityConnection**](RemoteFinicityConnection.md)

### Authorization

[APISecretBearerAuth](../README.md#APISecretBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_remote_finicity_connections_get

> Vec<models::RemoteConnection> v1_remote_finicity_connections_get(page, limit)
List all Connections with Finicity Data

Returns a paginated list of Connections with Finicity Data

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


## v1_remote_finicity_holdings_get

> Vec<models::RemoteHolding> v1_remote_finicity_holdings_get(page, limit)
List all Holdings with Finicity Data

Returns a paginated list of Holdings with Finicity Data

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


## v1_remote_finicity_holdings_holding_id_get

> models::RemoteFinicityHolding v1_remote_finicity_holdings_holding_id_get(holding_id)
Retrieve Holding Data

Retrieve Finicity Data by Holding ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**holding_id** | **String** |  | [required] |

### Return type

[**models::RemoteFinicityHolding**](RemoteFinicityHolding.md)

### Authorization

[APISecretBearerAuth](../README.md#APISecretBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_remote_finicity_transactions_get

> Vec<models::RemoteTransaction> v1_remote_finicity_transactions_get(page, limit)
List all Transactions with Finicity Data

Returns a paginated list of Transactions with Finicity Data

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


## v1_remote_finicity_transactions_transaction_id_get

> models::RemoteFinicityTransaction v1_remote_finicity_transactions_transaction_id_get(transaction_id)
Retrieve Transaction Data

Retrieve Finicity Data by Transaction ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**transaction_id** | **String** |  | [required] |

### Return type

[**models::RemoteFinicityTransaction**](RemoteFinicityTransaction.md)

### Authorization

[APISecretBearerAuth](../README.md#APISecretBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

