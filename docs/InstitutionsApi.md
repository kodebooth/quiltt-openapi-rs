# \InstitutionsApi

All URIs are relative to *https://api.quiltt.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**v1_institutions_get**](InstitutionsApi.md#v1_institutions_get) | **GET** /v1/institutions | Search Institutions by Routing Number



## v1_institutions_get

> Vec<models::Institution> v1_institutions_get(routing_number)
Search Institutions by Routing Number

Retrieves Institutions based on US Fed routing number. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**routing_number** | **String** | The routing number to look up | [required] |

### Return type

[**Vec<models::Institution>**](Institution.md)

### Authorization

[APISecretBearerAuth](../README.md#APISecretBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

