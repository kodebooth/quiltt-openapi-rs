# RemoteFinicityTransactionDocumentsApiFinicityComAggregationV4CustomersCustomerIdAccountsAccountIdTransactionsBody

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**account_id** | **i64** | An account ID represented as a number | 
**amount** | **f64** | The total amount of the transaction. Transactions for deposits are positive values, withdrawals and debits are negative values. | 
**categorization** | Option<[**models::RemoteFinicityTransactionDocumentsApiFinicityComAggregationV4CustomersCustomerIdAccountsAccountIdTransactionsBodyCategorization**](RemoteFinicityTransactionDocumentsApiFinicityComAggregationV4CustomersCustomerIdAccountsAccountIdTransactionsBodyCategorization.md)> |  | [optional]
**check_num** | Option<**String**> | The reference for the transaction provided by the originating institution. | [optional]
**commission_amount** | Option<**f32**> | Transaction commission | [optional]
**created_date** | **i64** | A date in Unix epoch time (in seconds). Represents the timestamp of the transaction when it was added to our platform. See: [Handling Epoch Dates and Times](https://developer.mastercard.com/open-finance-us/documentation/errors/best-practices/). | 
**currency_symbol** | Option<**String**> | If the foreign amount value is present then this is the currency code of that foreign amount | [optional]
**customer_id** | **i64** | A customer ID represented as a number. See Add Customer API for how to create a customer ID. | 
**description** | **String** | The description value is from the financial institution (FI), often known as the payee. The value \"No description provided by institution\" is returned when the FI doesn't provide one | 
**effective_date** | Option<**i64**> | A date in Unix epoch time (in seconds). Represents the timestamp of the transaction when it became effective on an account by an institution. See: [Handling Epoch Dates and Times](https://developer.mastercard.com/open-finance-us/documentation/errors/best-practices/). | [optional]
**escrow_amount** | Option<**f64**> | The portion of the transaction allocated to escrow | [optional]
**fee_amount** | Option<**f64**> | The portion of the overall transaction amount applied to fees | [optional]
**first_effective_date** | Option<**i64**> | A date in Unix epoch time (in seconds). Represents the first timestamp of the transaction recorded in the `effectiveDate` field. See: [Handling Epoch Dates and Times](https://developer.mastercard.com/open-finance-us/documentation/errors/best-practices/). | [optional]
**id** | **i64** | A transaction ID | 
**income_type** | Option<**String**> | Capital gains applied in short, long, or miscellaneous terms for tax purposes | [optional]
**interest_amount** | Option<**f64**> | The portion of the transaction allocated to interest | [optional]
**investment_transaction_type** | Option<**String**> | Keywords in the `description` and `memo` fields were used to translate investment transactions into these types.  Possible values: * \"cancel\"  * \"purchaseToClose\"  * \"purchaseToCover\"  * \"contribution\"  * \"optionExercise\"  * \"optionExpiration\"  * \"fee\"  * \"soldToClose\"  * \"soldToOpen\"  * \"split\"  * \"transfer\"  * \"returnOfCapital\"  * \"income\"  * \"purchased\"  * \"sold\"  * \"dividendReinvest\"  * \"tax\"  * \"dividend\"  * \"reinvestOfIncome\"  * \"interest\"  * \"deposit\"  * \"otherInfo\" | [optional]
**memo** | Option<**String**> | The institution must provide either a description, a memo, or both. We recommended concatenating the two fields into a single value. | [optional]
**option_expire_date** | Option<**i64**> | A date in Unix epoch time (in seconds). Represents the timestamp of the transaction expiration date when it became expires on an account by an institution. See: [Handling Epoch Dates and Times](https://developer.mastercard.com/open-finance-us/documentation/errors/best-practices/). | [optional]
**option_strike_price** | Option<**f64**> | The strike price of the option contract | [optional]
**posted_date** | Option<**i64**> | A date in Unix epoch time (in seconds). Represents the timestamp of the transaction when it was posted or cleared by the institution. This value isn't required for student loan transaction data. See: [Handling Epoch Dates and Times](https://developer.mastercard.com/open-finance-us/documentation/errors/best-practices/). | [optional]
**principal_amount** | Option<**f64**> | The portion of the transaction allocated to principal | [optional]
**running_balance_amount** | Option<**f64**> | The ending balance after the transaction was posted | [optional]
**security_id** | Option<**String**> | The security ID of the transaction | [optional]
**security_id_type** | Option<**String**> | The security type. This field is related to the `securityId` field. Possible values: * \"CUSIP\"  * \"ISIN\"  * \"SEDOL\"  * \"SICC\"  * \"VALOR\"  * \"WKN\" | [optional]
**shares_per_contract** | Option<**f64**> | Shares per contract of the underlying stock option | [optional]
**split_denominator** | Option<**f64**> | Denominator of the stock split for the transaction | [optional]
**split_numerator** | Option<**f64**> | Numerator of the stock split for the transaction | [optional]
**status** | **String** | One of \"active\", \"pending\", or \"shadow\" (see [Transaction Status](https://developer.mastercard.com/open-finance-us/documentation/products/manage/transaction-data/understanding-transaction-data/#transaction-status)) | 
**sub_account_fund** | Option<**String**> | The sub account where the funds came from | [optional]
**subaccount_security_type** | Option<**String**> | The type of sub account the funds came from | [optional]
**suspense_amount** | Option<**f64**> | Temporarily hold funds if you overpay or underpay your monthly payment | [optional]
**taxes_amount** | Option<**f32**> | Taxes applicable to the investment trade | [optional]
**ticker** | Option<**String**> | Ticker symbol for the investment related to the transaction | [optional]
**transaction_date** | Option<**i64**> | A date in Unix epoch time (in seconds). Represents the timestamp of the transaction when it occurred. See: [Handling Epoch Dates and Times](https://developer.mastercard.com/open-finance-us/documentation/errors/best-practices/). | [optional]
**r#type** | Option<**String**> | If provided by the institution, the following values may be returned in the field of a record: * \"atm\"  * \"cash\"  * \"check\"  * \"credit\"  * \"debit\"  * \"deposit\"  * \"directDebit\"  * \"directDeposit\"  * \"dividend\"  * \"fee\"  * \"interest\"  * \"other\"  * \"payment\"  * \"pointOfSale\"  * \"repeatPayment\"  * \"serviceCharge\"  * \"transfer\" | [optional]
**unit_price** | Option<**f64**> | Share price for the investment unit: stocks, mutual funds, ETFs | [optional]
**unit_quantity** | Option<**f32**> | The number of units (individual shares) in the transaction | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


