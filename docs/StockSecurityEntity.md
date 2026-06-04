# StockSecurityEntity

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**reinvest_dividends** | Option<**bool**> | Reinvest dividends | [optional]
**stock_type** | Option<**StockType**> |  (enum: BOND, DEBT, MUTUALFUND, OPTION, OTHER, STOCK, SWEEP) | [optional]
**units_street** | Option<**f64**> | Units in the FI's street name, positive quantity | [optional]
**units_user** | Option<**f64**> | Units in user's name directly, positive  quantity | [optional]
**r#yield** | Option<**f64**> | Current yield | [optional]
**yield_as_of_date** | Option<**chrono::DateTime<chrono::FixedOffset>**> | Yield as-of date | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


