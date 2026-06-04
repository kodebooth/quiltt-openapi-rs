# RemoteFinicityTransactionDocumentsApiFinicityComAggregationV4CustomersCustomerIdAccountsAccountIdTransactionsBodyCategorization

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**best_representation** | Option<**String**> | Combines the `description` and `memo` data together, removing duplicated information and numbers and special characters | [optional]
**category** | **String** | The different categories for transactions. * \"ATM Fee\"  * \"Advertising\"  * \"Air Travel\"  * \"Alcohol & Bars\"  * \"Allowance\"  * \"Amusement\"  * \"Arts\"  * \"Auto & Transport\"  * \"Auto Insurance\"  * \"Auto Payment\"  * \"Baby Supplies\"  * \"Babysitter & Daycare\"  * \"Bank Fee\"  * \"Bills & Utilities\"  * \"Bonus\"  * \"Books\"  * \"Books & Supplies\"  * \"Business Services\"  * \"Buy\"  * \"Cash & ATM\"  * \"Charity\"  * \"Check\"  * \"Child Support\"  * \"Clothing\"  * \"Coffee Shops\"  * \"Credit Card Payment\"  * \"Dentist\"  * \"Deposit\"  * \"Dividend & Cap Gains\"  * \"Doctor\"  * \"Education\"  * \"Electronics & Software\"  * \"Entertainment\"  * \"Eyecare\"  * \"Fast Food\"  * \"Federal Tax\"  * \"Fees & Charges\"  * \"Finance Charge\"  * \"Financial\"  * \"Financial Advisor\"  * \"Food & Dining\"  * \"Furnishings\"  * \"Gas & Fuel\"  * \"Gift\"  * \"Gifts & Donations\"  * \"Groceries\"  * \"Gym\"  * \"Hair\"  * \"Health & Fitness\"  * \"Health Insurance\"  * \"Hobbies\"  * \"Home\"  * \"Home Improvement\"  * \"Home Insurance\"  * \"Home Phone\"  * \"Home Services\"  * \"Home Supplies\"  * \"Hotel\"  * \"Income\"  * \"Interest Income\"  * \"Internet\"  * \"Investments\"  * \"Kids\"  * \"Kids Activities\"  * \"Late Fee\"  * \"Laundry\"  * \"Lawn & Garden\"  * \"Legal\"  * \"Life Insurance\"  * \"Loan Fees and Charges\"  * \"Loan Insurance\"  * \"Loan Interest\"  * \"Loan Payment\"  * \"Loan Principal\"  * \"Loans\"  * \"Local Tax\"  * \"Low Balance\"  * \"Mobile Phone\"  * \"Mortgage & Rent\"  * \"Movies & DVDs\"  * \"Music\"  * \"Newspapers & Magazines\"  * \"Office Supplies\"  * \"Parking\"  * \"Paycheck\"  * \"Personal Care\"  * \"Pet Food & Supplies\"  * \"Pet Grooming\"  * \"Pets\"  * \"Pharmacy\"  * \"Printing\"  * \"Property Tax\"  * \"Public Transportation\"  * \"Reimbursement\"  * \"Rental Car & Taxi\"  * \"Restaurants\"  * \"Sales Tax\"  * \"Sell\"  * \"Service & Parts\"  * \"Service Fee\"  * \"Shipping\"  * \"Shopping\"  * \"Spa & Massage\"  * \"Sporting Goods\"  * \"Sports\"  * \"State Tax\"  * \"Streaming Services\"  * \"Student Loan\"  * \"Taxes\"  * \"Television\"  * \"Toys\"  * \"Trade Commissions\"  * \"Transfer\"  * \"Transfer for Cash Spending\"  * \"Travel\"  * \"Tuition\"  * \"Uncategorized\"  * \"Utilities\"  * \"Vacation\"  * \"Veterinary\"  * \"Internet / Broadband Charges\" | 
**city** | Option<**String**> | City | [optional]
**country** | **String** | Country code is Iso3166-1 Alpha-2 code and Alpha 3 standard (max length 3). | 
**normalized_payee_name** | **String** | A normalized payee, derived from the transaction's description and memo fields | 
**postal_code** | Option<**String**> | A ZIP code | [optional]
**state** | Option<**String**> | State | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


