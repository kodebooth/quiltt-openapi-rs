# EnrichedTransaction

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**categories** | Option<[**models::Categories**](Categories.md)> |  | [optional]
**created_at** | **chrono::DateTime<chrono::FixedOffset>** | The timestamp of when the account holder was created. | 
**entities** | Option<[**models::Entities**](Entities.md)> |  | [optional]
**error** | Option<[**models::TransactionError**](TransactionError.md)> |  | [optional]
**id** | **String** | A unique identifier for the transaction. If two transactions are submitted with the same `id` the most recent one will replace the previous one. | 
**location** | Option<[**models::Location**](Location.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


