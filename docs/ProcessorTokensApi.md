# \ProcessorTokensApi

All URIs are relative to *https://api.quiltt.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**v1_accounts_account_id_processor_tokens_post**](ProcessorTokensApi.md#v1_accounts_account_id_processor_tokens_post) | **POST** /v1/accounts/{accountId}/processor_tokens | Create a Processor Token



## v1_accounts_account_id_processor_tokens_post

> models::ProcessorToken v1_accounts_account_id_processor_tokens_post(account_id, v1_accounts_account_id_processor_tokens_post_request)
Create a Processor Token

Generates a Processor Token for a given account to pass to a 3rd party payment or data processor.  Please double check that your preferred Processor is configured to consume the token from the token issuer. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** |  | [required] |
**v1_accounts_account_id_processor_tokens_post_request** | Option<[**V1AccountsAccountIdProcessorTokensPostRequest**](V1AccountsAccountIdProcessorTokensPostRequest.md)> |  |  |

### Return type

[**models::ProcessorToken**](ProcessorToken.md)

### Authorization

[APISecretBearerAuth](../README.md#APISecretBearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

