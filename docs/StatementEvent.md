# StatementEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The ID of the Event. | [readonly]
**at** | **chrono::DateTime<chrono::FixedOffset>** | The timestamp of when the Event occurred, in [ISO 8601 format](https://en.wikipedia.org/wiki/ISO_8601). | 
**r#type** | **Type** | The type of Statement Event that occurred. (enum: statement.ready) | [readonly]
**profile** | [**models::AccountEventProfile**](AccountEventProfile.md) |  | 
**record** | [**models::StatementEventRecord**](StatementEventRecord.md) |  | 
**metadata** | **serde_json::Value** | Currently there is no metadata available for Statement events. | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


