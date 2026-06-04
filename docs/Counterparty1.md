# Counterparty1

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**account_numbers** | Option<[**models::CounterpartyNumbers**](CounterpartyNumbers.md)> |  | [optional]
**confidence_level** | Option<**String**> | A description of how confident we are that the provided counterparty is involved in the transaction.  `VERY_HIGH`: We recognize this counterparty and we are more than 98% confident that it is involved in this transaction. `HIGH`: We recognize this counterparty and we are more than 90% confident that it is involved in this transaction. `MEDIUM`: We are moderately confident that this counterparty was involved in this transaction, but some details may differ from our records. `LOW`: We didn’t find a matching counterparty in our records, so we are returning a cleansed name parsed out of the request description. `UNKNOWN`: We don’t know the confidence level for this counterparty. | [optional]
**entity_id** | Option<**String**> | A unique, stable, Plaid-generated ID that maps to the counterparty. | [optional]
**logo_url** | Option<**String**> | The URL of a logo associated with the counterparty, if available. The logo will always be 100×100 pixel PNG file. | 
**name** | **String** | The name of the counterparty, such as the merchant or the financial institution, as extracted by Plaid from the raw description. | 
**r#type** | **Type** | The counterparty type.  `merchant`: a provider of goods or services for purchase `financial_institution`: a financial entity (bank, credit union, BNPL, fintech) `payment_app`: a transfer or P2P app (e.g. Zelle) `marketplace`: a marketplace (e.g DoorDash, Google Play Store) `payment_terminal`: a point-of-sale payment terminal (e.g Square, Toast) `income_source`: the payer in an income transaction (e.g., an employer, client, or government agency) (enum: merchant, financial_institution, payment_app, marketplace, payment_terminal, income_source) | 
**website** | Option<**String**> | The website associated with the counterparty. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


