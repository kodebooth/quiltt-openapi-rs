# MutualFundSecurityEntity

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**mutual_fund_type** | Option<**MutualFundType**> | Mutual fund type (enum: OPENEND, CLOSEEND, OTHER) | [optional]
**reinvest_capital_gains** | Option<**bool**> | Reinvest capital gains | [optional]
**reinvest_dividends** | Option<**bool**> | Reinvest dividends | [optional]
**units_street** | Option<**f64**> | Units in the FI's street name, positive quantity | [optional]
**units_user** | Option<**f64**> | Units in user's name directly, positive  quantity | [optional]
**r#yield** | Option<**f64**> | Current yield reported as portion of the fund's assets | [optional]
**yield_as_of_date** | Option<**chrono::DateTime<chrono::FixedOffset>**> | As-of date for yield value | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


