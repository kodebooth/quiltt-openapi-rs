# \PlaidApi

All URIs are relative to *https://api.quiltt.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**v1_remote_plaid_accounts_account_id_get**](PlaidApi.md#v1_remote_plaid_accounts_account_id_get) | **GET** /v1/remote/plaid/accounts/{accountId} | Retrieve Account data
[**v1_remote_plaid_accounts_get**](PlaidApi.md#v1_remote_plaid_accounts_get) | **GET** /v1/remote/plaid/accounts | List all Accounts with Plaid Data
[**v1_remote_plaid_connections_connection_id_get**](PlaidApi.md#v1_remote_plaid_connections_connection_id_get) | **GET** /v1/remote/plaid/connections/{connectionId} | Retrieve Plaid Connection Data
[**v1_remote_plaid_connections_get**](PlaidApi.md#v1_remote_plaid_connections_get) | **GET** /v1/remote/plaid/connections | List all Connections with Plaid Data
[**v1_remote_plaid_holdings_get**](PlaidApi.md#v1_remote_plaid_holdings_get) | **GET** /v1/remote/plaid/holdings | List all Holdings with Plaid Data
[**v1_remote_plaid_holdings_holding_id_get**](PlaidApi.md#v1_remote_plaid_holdings_holding_id_get) | **GET** /v1/remote/plaid/holdings/{holdingId} | Retrieve Holding Data
[**v1_remote_plaid_transactions_get**](PlaidApi.md#v1_remote_plaid_transactions_get) | **GET** /v1/remote/plaid/transactions | List all Transactions with Plaid Data
[**v1_remote_plaid_transactions_transaction_id_get**](PlaidApi.md#v1_remote_plaid_transactions_transaction_id_get) | **GET** /v1/remote/plaid/transactions/{transactionId} | Retrieve Transaction Data



## v1_remote_plaid_accounts_account_id_get

> models::RemotePlaidAccount v1_remote_plaid_accounts_account_id_get(account_id)
Retrieve Account data

Retrieve Plaid Data by Account ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** |  | [required] |

### Return type

[**models::RemotePlaidAccount**](RemotePlaidAccount.md)

### Authorization

[APISecretBearerAuth](../README.md#APISecretBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_remote_plaid_accounts_get

> Vec<models::RemoteAccount> v1_remote_plaid_accounts_get(page, limit)
List all Accounts with Plaid Data

Returns a paginated list of Accounts with Plaid Data

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


## v1_remote_plaid_connections_connection_id_get

> models::RemotePlaidConnection v1_remote_plaid_connections_connection_id_get(connection_id)
Retrieve Plaid Connection Data

Retrieve Plaid Data by Connection ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**connection_id** | **String** |  | [required] |

### Return type

[**models::RemotePlaidConnection**](RemotePlaidConnection.md)

### Authorization

[APISecretBearerAuth](../README.md#APISecretBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_remote_plaid_connections_get

> Vec<models::RemoteConnection> v1_remote_plaid_connections_get(page, limit)
List all Connections with Plaid Data

Returns a paginated list of Connections with Plaid Data

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


## v1_remote_plaid_holdings_get

> Vec<models::RemoteHolding> v1_remote_plaid_holdings_get(page, limit)
List all Holdings with Plaid Data

Returns a paginated list of Holdings with Plaid Data

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


## v1_remote_plaid_holdings_holding_id_get

> models::RemotePlaidHolding v1_remote_plaid_holdings_holding_id_get(holding_id)
Retrieve Holding Data

Retrieve Plaid Data by Holding ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**holding_id** | **String** |  | [required] |

### Return type

[**models::RemotePlaidHolding**](RemotePlaidHolding.md)

### Authorization

[APISecretBearerAuth](../README.md#APISecretBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_remote_plaid_transactions_get

> Vec<models::RemoteTransaction> v1_remote_plaid_transactions_get(page, limit)
List all Transactions with Plaid Data

Returns a paginated list of Transactions with Plaid Data

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


## v1_remote_plaid_transactions_transaction_id_get

> models::RemotePlaidTransaction v1_remote_plaid_transactions_transaction_id_get(transaction_id)
Retrieve Transaction Data

Retrieve Plaid Data by Transaction ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**transaction_id** | **String** |  | [required] |

### Return type

[**models::RemotePlaidTransaction**](RemotePlaidTransaction.md)

### Authorization

[APISecretBearerAuth](../README.md#APISecretBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

