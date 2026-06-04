# \SessionTokensApi

All URIs are relative to *https://api.quiltt.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**v1_users_session_delete**](SessionTokensApi.md#v1_users_session_delete) | **DELETE** /v1/users/session | Revoke current Session token
[**v1_users_sessions_post**](SessionTokensApi.md#v1_users_sessions_post) | **POST** /v1/users/sessions | Issue a Session token



## v1_users_session_delete

> v1_users_session_delete()
Revoke current Session token

Revokes the current Session token.  This can be used to power a logout action in your application.  You must pass the Session token in the Authorization header. 

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[SessionTokenBearerAuth](../README.md#SessionTokenBearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_users_sessions_post

> models::UserSession v1_users_sessions_post(v1_users_sessions_post_request)
Issue a Session token

Issues a Session token for one of your Profiles, using your API Key secret.  This endpoint supports authenticating existing Profiles, as well as creating new Profiles on the fly.  Note that the Session tokens are subject to a rate limit of 10 active sessions per hour and 20 active sessions per day. It's a good practice to revoke old session tokens when a user logs out of your application.  See the [Authentication guides](/authentication) for more information. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**v1_users_sessions_post_request** | Option<[**V1UsersSessionsPostRequest**](V1UsersSessionsPostRequest.md)> | To authenticate an existing Profile, supply their Profile ID or UUID via the `userId` body param.  To authenticate a new Profile, omit the `userId` body param. If you'd like to assign your own UUID, supply it in the `uuid` body param and it will be assigned to the new Profile.  Whether you're authenticating a new or existing Profile, you can supply optional Profile attributes to set or update on the authenticated Profile, such as their `email`, or `phone`.  For more information, see the Authentication guide [here](/authentication/issuing-session-tokens).  |  |

### Return type

[**models::UserSession**](UserSession.md)

### Authorization

[APISecretBearerAuth](../README.md#APISecretBearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

