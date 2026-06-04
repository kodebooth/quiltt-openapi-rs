# TreasuryPrimeTransaction

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ach_id** | Option<**String**> | The ID of the ACH object that originated this transaction, if any. Otherwise null. Filterable | [optional]
**amount** | Option<**String**> | Transaction amount. Positive values indicate a credit while negative values indicate a debit. Note that transactions with type hold have an amount, but they do not change the balance. | [optional]
**balance** | Option<**String**> | Account balance immediately after this transaction. Transactions of type hold do not affect the balance. | [optional]
**billpay_payment_id** | Option<**String**> | The ID of the Bill Pay object that originated this transaction, if any. Otherwise null. | [optional]
**book_id** | Option<**String**> | The ID of the Book Transfer object that originated this transaction, if any. Otherwise null. Filterable | [optional]
**card_id** | Option<**String**> | The ID of the Card Object that originated this transaction, if any. Otherwise null. Filterable | [optional]
**category** | Option<**String**> | One of: interest, fees, or null. PATCH this field to indicate if the transaction is an interest or fee payment. | [optional]
**check_id** | Option<**String**> | The ID of the Check Deposit object that originated this transaction, if any. Otherwise null. Filterable | [optional]
**check_number** | Option<**String**> | If this transaction is for an outbound check, the number of the check. Otherwise null. | [optional]
**date** | Option<**chrono::NaiveDate**> | Date of the transaction, ISO 8601 format (\"YYYY-MM-DD\"). Received directly from the bank, often without time zone adjustment. Might differ in date compared to extended_timestamp and extended_timestamp_precise due to timezones. | [optional]
**extended_timestamp** | Option<**chrono::DateTime<chrono::FixedOffset>**> | Timestamp of the transaction, ISO 8601 format (\"YYYY-MM-DDThh:mm:ssZ\"). Limited availability. Always in UTC. | [optional]
**extended_timestamp_precise** | Option<**chrono::DateTime<chrono::FixedOffset>**> | Timestamp of the transaction, ISO 8601 format with subsecond precision (\"YYYY-MM-DDThh:mm:ss.SSS[SSS]Z\"). The precision could be millisecond or microsecond depending on source data. Limited availability. Always in UTC. | [optional]
**desc** | Option<**String**> | Transaction description. | [optional]
**human_readable_description** | Option<**String**> | This field is updated every minute for new transactions to provide a user-friendly description. Note, there may be a short period (less than 60 seconds) after a transaction is accessible through our API where the human_readable_description field is not yet populated. | [optional]
**fingerprint** | Option<**String**> | A unique fingerprint for this transaction. | [optional]
**id** | Option<**String**> | ID for this transaction. | [optional]
**incoming_achid** | Option<**String**> | The ID of the Incoming ACH object that originated this transaction, if any. Otherwise null. Filterable | [optional]
**incoming_wire** | Option<**String**> | Data (see below for definition) related to the wire that originated this transaction, if any. Otherwise null. | [optional]
**incoming_wire_id** | Option<**String**> | The ID of the Incoming Wire object that originated this transaction, if any. Otherwise null. Filterable | [optional]
**issued_check_id** | Option<**String**> | The ID of the Issued Check Object that originated this transaction, if any. Otherwise null. Filterable | [optional]
**network_transfer_id** | Option<**String**> | The ID of the Network Transfer that originated this transaction, if any. Otherwise null. Filterable | [optional]
**related_transfer_ids** | Option<**Vec<String>**> | The IDs of the related transactions, if any. Otherwise null. | [optional]
**summary** | Option<**String**> | Summary description of the transaction. | [optional]
**trace_id** | Option<**String**> | A common id used to tie multiple, related transactions together. Currently used to unite hold andhold_release transactions to a debit and to link transactions to their related card events. Filterable | [optional]
**r#type** | Option<**String**> | Type of transaction. One of charge,deposit, hold,hold_release, interest,payment, reversal,withdrawal, or null. Filterable | [optional]
**type_source** | Option<**String**> | The type of payment that initiated this transaction, if any. One of ach,bank, book,card, check, orwire. Otherwise null. Limited availability | [optional]
**userdata** | Option<**std::collections::HashMap<String, serde_json::Value>**> | Optional arbitrary user data. | [optional]
**wire** | Option<**String**> | For wire transactions, the Fedwire description, if any. Otherwise null. | [optional]
**wire_id** | Option<**String**> | The ID of the Wire object that originated this transaction, if any. Otherwise null. Filterable | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


