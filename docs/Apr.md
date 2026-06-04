# Apr

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**apr_percentage** | **f64** | Annual Percentage Rate applied.  | 
**apr_type** | **AprType** | The type of balance to which the APR applies. (enum: balance_transfer_apr, cash_apr, purchase_apr, special) | 
**balance_subject_to_apr** | Option<**f64**> | Amount of money that is subjected to the APR if a balance was carried beyond payment due date. How it is calculated can vary by card issuer. It is often calculated as an average daily balance. | 
**interest_charge_amount** | Option<**f64**> | Amount of money charged due to interest from last statement. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


