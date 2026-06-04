# VerificationInsights

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**account_number_format** | **AccountNumberFormat** | Indicator of account number format validity for institution.  `valid`: indicates that the account number has a correct format for the institution.  `invalid`: indicates that the account number has an incorrect format for the institution.  `unknown`: indicates that there was not enough information to determine whether the format is correct for the institution. (enum: valid, invalid, unknown) | 
**name_match_score** | Option<**i32**> | Indicates the score of the name match between the given name provided during database verification (available in the [`verification_name`](https://plaid.com/docs/api/products/auth/#auth-get-response-accounts-verification-name) field) and matched Plaid network accounts. If defined, will be a value between 0 and 100. Will be undefined if name matching was not enabled for the database verification session or if there were no eligible Plaid network matches to compare the given name with. | [optional]
**network_status** | [**models::VerificationInsightsNetworkStatus**](VerificationInsightsNetworkStatus.md) |  | 
**previous_returns** | Option<[**models::VerificationInsightsNetworkStatus1**](VerificationInsightsNetworkStatus1.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


