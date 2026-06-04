# \BalancesApi

All URIs are relative to *https://api.quiltt.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**v1_accounts_account_id_balances_refresh_post**](BalancesApi.md#v1_accounts_account_id_balances_refresh_post) | **POST** /v1/accounts/{accountId}/balances/refresh | Trigger a Balance Refresh



## v1_accounts_account_id_balances_refresh_post

> v1_accounts_account_id_balances_refresh_post(account_id)
Trigger a Balance Refresh

Triggers a Balance Refresh for a given Account.  Successful requests trigger a balance refresh from the Connection provider and return a `202 Accepted` response.  As soon as Quiltt obtains the latest balance, you will receive it via the [`balance.created` webhook event](#tag/Event-Schemas/BalanceEvent-Schema). The new Balance record will have the `REFRESH` source and also be available in GraphQL.  Note that some provider-specific limitations apply based on the Account kind: - Finicity only supports Balance Refresh for `DEPOSITORY` accounts. - Plaid does not support Balance Refresh for `LOAN` accounts. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[APISecretBearerAuth](../README.md#APISecretBearerAuth), [ProcessorTokenBasicAuth](../README.md#ProcessorTokenBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

