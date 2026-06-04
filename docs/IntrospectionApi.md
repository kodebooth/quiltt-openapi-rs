# \IntrospectionApi

All URIs are relative to *https://api.quiltt.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**v1_introspect_api_key_post**](IntrospectionApi.md#v1_introspect_api_key_post) | **POST** /v1/introspect/api_key | Check API Key
[**v1_introspect_processor_token_post**](IntrospectionApi.md#v1_introspect_processor_token_post) | **POST** /v1/introspect/processor_token | Check Processor Token
[**v1_introspect_session_token_post**](IntrospectionApi.md#v1_introspect_session_token_post) | **POST** /v1/introspect/session_token | Check Session Token



## v1_introspect_api_key_post

> models::ApiKey v1_introspect_api_key_post()
Check API Key

Checks that your API Key is valid.  Successful requests will return information about the API Key. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ApiKey**](APIKey.md)

### Authorization

[APISecretBearerAuth](../README.md#APISecretBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_introspect_processor_token_post

> models::ProcessorToken v1_introspect_processor_token_post()
Check Processor Token

Checks that the current Processor Token is valid.  Successful requests will return information about the Processor Token. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ProcessorToken**](ProcessorToken.md)

### Authorization

[ProcessorTokenBasicAuth](../README.md#ProcessorTokenBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_introspect_session_token_post

> models::UserSession v1_introspect_session_token_post()
Check Session Token

Checks that the current Session Token is valid.  Successful requests will return information about the Session. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::UserSession**](UserSession.md)

### Authorization

[SessionTokenBearerAuth](../README.md#SessionTokenBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

