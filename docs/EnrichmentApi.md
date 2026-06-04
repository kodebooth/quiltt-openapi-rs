# \EnrichmentApi

All URIs are relative to *https://api.quiltt.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**v1_remote_enrichment_sync_post**](EnrichmentApi.md#v1_remote_enrichment_sync_post) | **POST** /v1/remote/enrichment/sync | Synchronously Enrich Transactions



## v1_remote_enrichment_sync_post

> models::RemoteEnrichment v1_remote_enrichment_sync_post(remote_enrichment_request)
Synchronously Enrich Transactions

This endpoint is used to perform on-demand enrichment of transactions from one of Quiltt's enrichment providers.  Note that transactions from connected accounts are enriched automatically, with Enrichment responses made available via Transaction [Remote Data](https://quiltt.dev/api/remote-data).  If your Transaction data provider is not supported, use the `GENERIC` provider and format your payload to Quiltt's schema. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**remote_enrichment_request** | Option<[**RemoteEnrichmentRequest**](RemoteEnrichmentRequest.md)> |  |  |

### Return type

[**models::RemoteEnrichment**](RemoteEnrichment.md)

### Authorization

[APISecretBearerAuth](../README.md#APISecretBearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

