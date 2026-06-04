# RemoteFinicityAccountDocumentsApiFinicityComAggregationV1CustomersCustomerIdInstitutionLoginsInstitutionLoginIdAccountsBodyPositionInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**asset_class** | Option<**String**> | An asset class is a grouping of comparable financial securities. These include equities (stocks), fixed income (bonds), and cash equivalent or money market instruments. (DOMESTICBOND, LARGESTOCK, INTLSTOCK, MONEYMRKT, OTHER) | [optional]
**change_percent** | Option<**f64**> | The percent change in value since the previous day | [optional]
**cost_basis** | Option<**f64**> | The total cost of acquiring the security | [optional]
**cost_basis_per_share** | Option<**f64**> | The per share cost of acquiring the security | [optional]
**currency_rate** | Option<**f64**> | Currency rate, ratio of currency to original currency | [optional]
**current_price** | Option<**f64**> | The current price of the investment holding | [optional]
**current_price_date** | Option<**i64**> | A date in Unix epoch time (in seconds). See: [Handling Epoch Dates and Times](https://developer.mastercard.com/open-finance-us/documentation/errors/best-practices/). | [optional]
**daily_change** | Option<**f64**> | The value amount change since the previous day | [optional]
**description** | Option<**String**> | The description of the holding | [optional]
**fi_asset_class** | Option<**String**> | Financial Institution (FI) defined asset class (COMMON STOCK, COMNEQTY, EQUITY/STOCK, CMA-ISA, CONVERTIBLE PREFERREDS, CORPORATE BONDS, OTHER MONEY FUNDS, ALLOCATION FUNDS, CMA-TAXABLE, FOREIGNEQUITYADRS, COMMONSTOCK, PREFERRED STOCKS, STABLE VALUE, FOREIGN EQUITY ADRS) | [optional]
**hold_type** | Option<**String**> | The type of the holding | [optional]
**id** | Option<**i64**> | The ID of the investment position | [optional]
**inv_security_type** | Option<**String**> | The security type for the investment holding | [optional]
**market_value** | Option<**f64**> | Market value of an investment position at the time of retrieval | [optional]
**mf_type** | Option<**String**> | Type of mutual fund, such as open ended | [optional]
**option_expire_date** | Option<**chrono::NaiveDate**> | Expiration date of option | [optional]
**option_shares_per_contract** | Option<**f64**> | The number of shares per option contract | [optional]
**option_strike_price** | Option<**f64**> | The strike price of the option contract | [optional]
**option_type** | Option<**String**> | The type of option contract (PUT or CALL) | [optional]
**pos_type** | Option<**String**> | Fund type assigned by the FI (long or short) | [optional]
**security_currency** | Option<**String**> | Symbol for the currency that the account is being converted into | [optional]
**security_id** | Option<**String**> | The security ID of the transaction | [optional]
**security_id_type** | Option<**String**> | The security type. This field is related to the `securityId` field. Possible values: * \"CUSIP\"  * \"ISIN\"  * \"SEDOL\"  * \"SICC\"  * \"VALOR\"  * \"WKN\" | [optional]
**security_name** | Option<**String**> | The security name for the investment holding | [optional]
**security_type** | Option<**String**> | Type of security for the investment position | [optional]
**status** | Option<**String**> | The status of the holding | [optional]
**sub_account_type** | Option<**String**> | The subaccount's type, such as cash | [optional]
**symbol** | Option<**String**> | The investment position's market ticker symbol | [optional]
**today_gl_dollar** | Option<**f64**> | The current day's gain and loss of the position at the time of aggregation in dollars | [optional]
**today_gl_percent** | Option<**f64**> | The current day's gain and loss of the position at the time of aggregation in percentage | [optional]
**total_gl_dollar** | Option<**f64**> | Total gain and loss of the position at the time of aggregation in dollars | [optional]
**total_gl_percent** | Option<**f64**> | Total gain and loss of the position at the time of aggregation in percentage | [optional]
**transaction_type** | Option<**String**> | The transaction type of the holding, such as cash, margin, and more | [optional]
**units** | Option<**f64**> | The number of units of the holding | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


