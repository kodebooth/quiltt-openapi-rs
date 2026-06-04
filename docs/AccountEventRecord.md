# AccountEventRecord

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The ID of the Account. | [readonly]
**connection_id** | **String** | The ID of the Connection. | [readonly]
**at** | **chrono::DateTime<chrono::FixedOffset>** | The UTC date and time of the most recently known state of the Account.  This typically represents the time when data was last successfully synced with the upstream provider. | [readonly]
**state** | **State** | Represents the current state of an Account.  Accounts that are marked as `CLOSED` by the provider should not be expected to sync.  (enum: OPEN, CLOSED) | [readonly]
**metadata** | Option<**serde_json::Value**> | Custom metadata about the Account, stored in a 'key-value' format.  See the [Custom Metadata](https://quiltt.dev/api/custom-metadata) guide for more information and examples. | 
**created_at** | **chrono::DateTime<chrono::FixedOffset>** |  | [readonly]
**updated_at** | **chrono::DateTime<chrono::FixedOffset>** |  | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


