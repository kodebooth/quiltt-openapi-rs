# OptionContract

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**contract_type** | **String** | The type of this option contract. It is one of:  `put`: for Put option contracts  `call`: for Call option contracts | 
**expiration_date** | **chrono::NaiveDate** | The expiration date for this option contract, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format. | 
**strike_price** | **f64** | The strike price for this option contract, per share of security. | 
**underlying_security_ticker** | **String** | The ticker of the underlying security for this option contract. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


