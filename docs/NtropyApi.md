# \NtropyApi

All URIs are relative to *https://api.quiltt.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**v1_remote_ntropy_transactions_get**](NtropyApi.md#v1_remote_ntropy_transactions_get) | **GET** /v1/remote/ntropy/transactions | List all Transactions with Ntropy Data
[**v1_remote_ntropy_transactions_transaction_id_get**](NtropyApi.md#v1_remote_ntropy_transactions_transaction_id_get) | **GET** /v1/remote/ntropy/transactions/{transactionId} | Retrieve Transaction Data



## v1_remote_ntropy_transactions_get

> Vec<models::RemoteTransaction> v1_remote_ntropy_transactions_get(page, limit)
List all Transactions with Ntropy Data

Returns a paginated list of Transactions that have Ntropy Data

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


## v1_remote_ntropy_transactions_transaction_id_get

> models::RemoteNtropyTransaction v1_remote_ntropy_transactions_transaction_id_get(transaction_id)
Retrieve Transaction Data

Retrieve Ntropy Data by Transaction ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**transaction_id** | **String** |  | [required] |

### Return type

[**models::RemoteNtropyTransaction**](RemoteNtropyTransaction.md)

### Authorization

[APISecretBearerAuth](../README.md#APISecretBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

