# BalanceEventRecord

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The ID of the Balance. | [readonly]
**account_id** | **String** | The ID of the Account. | [readonly]
**at** | **chrono::DateTime<chrono::FixedOffset>** | The timestamp of the Account Balance record. | [readonly]
**source** | **Source** | The source of the Balance data.  Note that this enum is extensible and new values may be added over time.  - `INITIAL` - Initial value from the provider.  - `SYNC` - Regular sync with the provider.  - `REFRESH` - Provider response from a triggered **Refresh Balance** call.  (enum: INITIAL, SYNC, REFRESH) | 
**available** | Option<**f64**> | The amount of funds accounting for pending Transactions. For asset accounts (checking, savings, investment), a positive balance indicates money in the account and a negative balance indicates the account holder owing the institution. For liability accounts (credit cards, loans), a negative balance indicates the amount owed and a positive amount indicates the lender owing the account holder. | 
**current** | Option<**f64**> | The amount of funds based on posted Transactions. For asset accounts (checking, savings, investment), a positive balance indicates money in the account and a negative balance indicates the account holder owing the institution. For liability accounts (credit cards, loans), a negative balance indicates the amount owed and a positive amount indicates the lender owing the account holder. | 
**limit** | Option<**f64**> | The amount of funds that may be overdraft or spent on credit. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


