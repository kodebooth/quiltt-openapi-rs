# Statement

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The ID of the Statement. | [readonly]
**account_id** | **String** | The ID of the Account. | [readonly]
**start_on** | Option<**chrono::NaiveDate**> | The start date of the Statement period. | 
**end_on** | Option<**chrono::NaiveDate**> | The end date of the Statement period. | 
**url** | **String** | The URL of the Statement PDF, with a 1-hour expiration. | 
**at** | **chrono::DateTime<chrono::FixedOffset>** | The UTC date and time of the most recently known state of the Statement.  This typically represents the time when data was last successfully synced with the upstream provider. | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


