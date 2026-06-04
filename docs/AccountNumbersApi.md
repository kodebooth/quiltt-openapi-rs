# \AccountNumbersApi

All URIs are relative to *https://api.quiltt.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**v1_accounts_account_id_ach_numbers_get**](AccountNumbersApi.md#v1_accounts_account_id_ach_numbers_get) | **GET** /v1/accounts/{accountId}/ach_numbers | Retrieve ACH Numbers



## v1_accounts_account_id_ach_numbers_get

> models::AchNumbers v1_accounts_account_id_ach_numbers_get(account_id)
Retrieve ACH Numbers

Retrieves full ACH routing and account numbers associated with a connected account.  These numbers can be sent to a payment processor to enable direct deposits and withdrawals from the account.  Please note that for the numbers be available, the account holder must have verified the account through a Connector enabled with the **Account Numbers** product. To be notified when the numbers are available, subscribe to the [`account.verified` webhook event](#tag/Event-Schemas/AccountEvent-Schema).  For Finicity accounts where ACH numbers have not been cached yet, a `202 Accepted` response will be returned while the numbers are fetched asynchronously. Retry the request. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** |  | [required] |

### Return type

[**models::AchNumbers**](ACHNumbers.md)

### Authorization

[APISecretBearerAuth](../README.md#APISecretBearerAuth), [ProcessorTokenBasicAuth](../README.md#ProcessorTokenBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

