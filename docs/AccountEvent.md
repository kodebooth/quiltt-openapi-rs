# AccountEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The ID of the Event. | [readonly]
**at** | **chrono::DateTime<chrono::FixedOffset>** | The timestamp of when the Event occurred, in [ISO 8601 format](https://en.wikipedia.org/wiki/ISO_8601). | 
**r#type** | **Type** | The type of Account Event that occurred. (enum: account.created, account.owners_verified, account.reconnected, account.verified) | [readonly]
**profile** | [**models::AccountEventProfile**](AccountEventProfile.md) |  | 
**record** | [**models::AccountEventRecord**](AccountEventRecord.md) |  | 
**metadata** | **serde_json::Value** | Additional context associated with the Account Event. Currently the following Account events expose context via `metadata`:  - `account.reconnected`: will provide `from` and `to` containing objects for the old and new Connections respectively. The Connection objects will contain an `id` and `provider`. | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


