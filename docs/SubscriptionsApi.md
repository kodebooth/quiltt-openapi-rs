# \SubscriptionsApi

All URIs are relative to *https://api.quiltt.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**v1_webhooks_subscriptions_get**](SubscriptionsApi.md#v1_webhooks_subscriptions_get) | **GET** /v1/webhooks/subscriptions | List all Subscriptions
[**v1_webhooks_subscriptions_id_delete**](SubscriptionsApi.md#v1_webhooks_subscriptions_id_delete) | **DELETE** /v1/webhooks/subscriptions/{id} | Delete a Subscription
[**v1_webhooks_subscriptions_id_get**](SubscriptionsApi.md#v1_webhooks_subscriptions_id_get) | **GET** /v1/webhooks/subscriptions/{id} | Retrieve a Subscription
[**v1_webhooks_subscriptions_id_patch**](SubscriptionsApi.md#v1_webhooks_subscriptions_id_patch) | **PATCH** /v1/webhooks/subscriptions/{id} | Update a Subscription
[**v1_webhooks_subscriptions_post**](SubscriptionsApi.md#v1_webhooks_subscriptions_post) | **POST** /v1/webhooks/subscriptions | Create a Subscription



## v1_webhooks_subscriptions_get

> Vec<models::WebhookSubscription> v1_webhooks_subscriptions_get(page, limit)
List all Subscriptions

Returns a paginated list of your Subscriptions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | Page to retrieve |  |[default to 1]
**limit** | Option<**i32**> | Number of items per page |  |[default to 25]

### Return type

[**Vec<models::WebhookSubscription>**](WebhookSubscription.md)

### Authorization

[APISecretBearerAuth](../README.md#APISecretBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_webhooks_subscriptions_id_delete

> v1_webhooks_subscriptions_id_delete(id)
Delete a Subscription

Deletes a Subscription

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[APISecretBearerAuth](../README.md#APISecretBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_webhooks_subscriptions_id_get

> models::WebhookSubscription v1_webhooks_subscriptions_id_get(id)
Retrieve a Subscription

Retrieves a Subscription by its ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::WebhookSubscription**](WebhookSubscription.md)

### Authorization

[APISecretBearerAuth](../README.md#APISecretBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_webhooks_subscriptions_id_patch

> models::WebhookSubscription v1_webhooks_subscriptions_id_patch(id, webhook_subscription_update_request)
Update a Subscription

Updates a Subscription

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**webhook_subscription_update_request** | Option<[**WebhookSubscriptionUpdateRequest**](WebhookSubscriptionUpdateRequest.md)> |  |  |

### Return type

[**models::WebhookSubscription**](WebhookSubscription.md)

### Authorization

[APISecretBearerAuth](../README.md#APISecretBearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_webhooks_subscriptions_post

> models::WebhookSubscription v1_webhooks_subscriptions_post(webhook_subscription_create_request)
Create a Subscription

Creates a Subscription

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_subscription_create_request** | Option<[**WebhookSubscriptionCreateRequest**](WebhookSubscriptionCreateRequest.md)> |  |  |

### Return type

[**models::WebhookSubscription**](WebhookSubscription.md)

### Authorization

[APISecretBearerAuth](../README.md#APISecretBearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

