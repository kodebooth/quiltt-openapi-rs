# ConnectionEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The ID of the Event. | [readonly]
**at** | **chrono::DateTime<chrono::FixedOffset>** | The timestamp of when the Event occurred, in [ISO 8601 format](https://en.wikipedia.org/wiki/ISO_8601). | 
**r#type** | **Type** | The type of Connection Event that occurred. (enum: connection.created, connection.synced.successful, connection.synced.successful.initial, connection.synced.successful.historical, connection.synced.errored.repairable, connection.synced.errored.institution, connection.synced.errored.provider, connection.synced.errored.service, connection.disconnected) | [readonly]
**profile** | [**models::AccountEventProfile**](AccountEventProfile.md) |  | 
**record** | [**models::ConnectionEventRecord**](ConnectionEventRecord.md) |  | 
**metadata** | **serde_json::Value** | Additional context associated with the Connection Event. Currently the following Connection events expose context via `metadata`:  - `connection.synced.successful*`: may provide an optional `startDate`/`endDate` pair indicating the date range of new data synced for the Connection. | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


