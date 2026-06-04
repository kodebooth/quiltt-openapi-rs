# RemoteAkoyaTransactionDocumentsProductsDdpAkoyaComTransactionsV2ProviderIdAccountIdBodyAnyOf4InsuranceTransaction

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**account_id** | Option<**String**> | Corresponds to AccountId in Account | [optional]
**amount** | Option<**f64**> | The amount of money in the account currency.  If balanceType is `ASSET`:    1. If `debitCreditMemo` = `DEBIT`, sign is \"+\" or not present   2. If `CREDIT`, sign is \"-\"  If balanceType is `LIABILITY`:    1. If `debitCreditMemo` = `DEBIT`, sign is \"-\"   2. If `CREDIT`, sign is \"+\" or not present | [optional]
**category** | Option<**String**> | Transaction category, preferably MCC or SIC. | [optional]
**debit_credit_memo** | Option<**DebitCreditMemo**> | Akoya will ensure that this is correctly populated with one of DEBIT or CREDIT and matches the sign of the status field. (enum: DEBIT, CREDIT) | [optional]
**description** | Option<**String**> | The description of the transaction | [optional]
**fi_attributes** | Option<[**HashSet<models::FiAttributeEntity>**](FiAttributeEntity.md)> | Array of FI-specific attributes | [optional]
**foreign_amount** | Option<**f64**> | The amount of money in the foreign currency | [optional]
**foreign_currency** | Option<**String**> | The ISO 4217 code of the foreign currency | [optional]
**image_ids** | Option<**Vec<String>**> | Array of image identifiers (unique to transaction) used to retrieve images of check or transaction receipt. | [optional]
**line_item** | Option<[**Vec<models::LineItem>**](LineItem.md)> | Breakdown of the transaction details | [optional]
**links** | Option<[**Vec<models::HateoasLink>**](HATEOASLink.md)> | Links (unique to this Transaction) used to retrieve images of checks or transaction receipts, or invoke other APIs | [optional]
**memo** | Option<**String**> | Secondary transaction description | [optional]
**posted_timestamp** | Option<**chrono::DateTime<chrono::FixedOffset>**> | The date and time that the transaction was posted to the account. If not provided then TransactionTimestamp can be used as PostedTimeStamp. | [optional]
**reference** | Option<**String**> | A tracking reference identifier | [optional]
**reference_transaction_id** | Option<**String**> | Akoya ensures that this field is populated for all transactions which are reversals, otherwise it is null. Either way it is always present.  For reverse postings, the identity of the transaction being reversed. For the correction transaction, the identity of the reversing post. For credit card posting transactions, the identity of the authorization transaction. | [optional]
**status** | Option<**Status**> | AUTHORIZATION, MEMO, PENDING, or POSTED (enum: PENDING, MEMO, POSTED, AUTHORIZATION) | [optional]
**sub_category** | Option<**String**> | Transaction category detail | [optional]
**transaction_id** | Option<**String**> | Long term persistent identity of the transaction (unique to account). Transaction IDs should:    1. be the same for pending and posted   2. be different for reversed transactions   3. `referenceTransactionId` should be present for reversed transactions' | [optional]
**transaction_timestamp** | Option<**chrono::DateTime<chrono::FixedOffset>**> | The date and time that the transaction was added to the server backend systems.  Akoya ensures that this field is populated for all transactions to which it applies, otherwise it is null. Either way it is always present. | [optional]
**transaction_type** | Option<**TransactionType**> | InsuranceTransaction Type (enum: PAYMENT, FEE, ADJUSTMENT, INTEREST) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


