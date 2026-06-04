# WebhookPayloadEventsInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The ID of the Event. | [readonly]
**at** | **chrono::DateTime<chrono::FixedOffset>** | The timestamp of when the Event occurred, in [ISO 8601 format](https://en.wikipedia.org/wiki/ISO_8601). | 
**r#type** | **Type** | The type of Connection Event that occurred. (enum: connection.created, connection.synced.successful, connection.synced.successful.initial, connection.synced.successful.historical, connection.synced.errored.repairable, connection.synced.errored.institution, connection.synced.errored.provider, connection.synced.errored.service, connection.disconnected, account.created, account.owners_verified, account.reconnected, account.verified, balance.created, statement.ready, profile.created, profile.ready, profile.deleted) | [readonly]
**profile** | [**models::AccountEventProfile**](AccountEventProfile.md) |  | 
**record** | [**models::ProfileEventRecord**](ProfileEventRecord.md) |  | 
**metadata** | **serde_json::Value** | Additional context associated with the Profile Event. Currently the following Profile events expose context via `metadata`:  - `profile.ready`: may provide an optional `connectionId` and an optional `startDate`/`endDate` pair indicating the scope of data changed on the Profile. | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


