# UserSession

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**token** | **String** | A Session token to access the GraphQL API or load the Quiltt SDK. | [readonly]
**user_id** | **String** | The ID of the Profile. | [readonly]
**user_uuid** | **uuid::Uuid** | The UUID of the Profile. | [readonly]
**environment_id** | **String** | The ID of the Environment. | [readonly]
**expiration** | **f64** | The UTC date and time when the User Session will expire, in [UNIX format](https://en.wikipedia.org/wiki/Unix_time). | 
**expires_at** | **chrono::DateTime<chrono::FixedOffset>** | The UTC date and time when the User Session will expire, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format. | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


