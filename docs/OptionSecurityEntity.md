# OptionSecurityEntity

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**expire_date** | Option<**chrono::DateTime<chrono::FixedOffset>**> | Expiration date of option | [optional]
**option_type** | Option<**OptionType**> |  (enum: CALL, PUT) | [optional]
**secured** | Option<**Secured**> | How the option is secured (enum: COVERED, NAKED) | [optional]
**shares_per_contract** | Option<**f64**> | Shares per contract | [optional]
**strike_price** | Option<**f64**> | Strike price / Unit price | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


