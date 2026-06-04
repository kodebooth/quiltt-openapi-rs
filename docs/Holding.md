# Holding

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**account_id** | **String** | The Plaid `account_id` associated with the holding. | 
**cost_basis** | Option<**f64**> | The total cost basis of the holding (e.g., the total amount spent to acquire all assets currently in the holding). | 
**institution_price** | **f64** | The last price given by the institution for this security. | 
**institution_price_as_of** | Option<**chrono::NaiveDate**> | The date at which `institution_price` was current. | [optional]
**institution_price_datetime** | Option<**chrono::DateTime<chrono::FixedOffset>**> | Date and time at which `institution_price` was current, in ISO 8601 format (YYYY-MM-DDTHH:mm:ssZ).  This field is returned for select financial institutions and comes as provided by the institution. It may contain default time values (such as 00:00:00).  | [optional]
**institution_value** | **f64** | The value of the holding, as reported by the institution. | 
**iso_currency_code** | Option<**String**> | The ISO-4217 currency code of the holding. Always `null` if `unofficial_currency_code` is non-`null`. | 
**quantity** | **f64** | The total quantity of the asset held, as reported by the financial institution. If the security is an option, `quantity` will reflect the total number of options (typically the number of contracts multiplied by 100), not the number of contracts. | 
**security_id** | **String** | The Plaid `security_id` associated with the holding. Security data is not specific to a user's account; any user who held the same security at the same financial institution at the same time would have identical security data. The `security_id` for the same security will typically be the same across different institutions, but this is not guaranteed. The `security_id` does not typically change, but may change if inherent details of the security change due to a corporate action, for example, in the event of a ticker symbol change or CUSIP change. | 
**unofficial_currency_code** | Option<**String**> | The unofficial currency code associated with the holding. Always `null` if `iso_currency_code` is non-`null`. Unofficial currency codes are used for currencies that do not have official ISO currency codes, such as cryptocurrencies and the currencies of certain countries.  See the [currency code schema](https://plaid.com/docs/api/accounts#currency-code-schema) for a full listing of supported `iso_currency_code`s.  | 
**vested_quantity** | Option<**f64**> | The total quantity of vested assets held, as reported by the financial institution. Vested assets are only associated with [equities](https://plaid.com/docs/api/products/investments/#investments-holdings-get-response-securities-type). | [optional]
**vested_value** | Option<**f64**> | The value of the vested holdings as reported by the institution. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


