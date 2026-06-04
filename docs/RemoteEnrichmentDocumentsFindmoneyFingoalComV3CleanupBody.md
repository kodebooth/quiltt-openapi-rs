# RemoteEnrichmentDocumentsFindmoneyFingoalComV3CleanupBody

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**account_type** | Option<**String**> | The type of account associated with the transaction (e.g., 'checking', 'savings') | [optional]
**accountid** | Option<**String**> | The ID of the account associated with the transaction | [optional]
**address** | Option<**String**> | The address associated with the transaction | [optional]
**amountnum** | Option<**f64**> | The transaction's USD amount | [optional]
**category** | Option<**String**> | The most applicable categorization for the transaction | [optional]
**category_id** | Option<**f64**> | The numeric ID of the transaction's category | [optional]
**category_label** | Option<**Vec<String>**> | A cascading hierarchy of the transaction's categories, from high-level to detail-level categorization. This field is deprecated and not recommended for use, as it may not reflect more correct information available in other 'category' fields. | [optional]
**city** | Option<**String**> | The city associated with the transaction | [optional]
**client_id** | Option<**String**> | Your FinGoal client ID | [optional]
**container** | Option<**String**> | A high-level categorization of the account type. Eg, 'bank' | [optional]
**date** | Option<**chrono::DateTime<chrono::FixedOffset>**> | The date on which the transaction took place | [optional]
**detail_category_id** | Option<**f64**> | The numeric ID of the transaction's detail category | [optional]
**guid** | Option<**String**> | The transaction's globally unique FinSight API issued ID | [optional]
**high_level_category_id** | Option<**f64**> | The numeric ID of the transaction's high level category | [optional]
**is_physical** | Option<**bool**> | Whether the transaction was made at a physical location, or online | [optional]
**is_recurring** | Option<**bool**> | Whether the transaction is set to recur on a fixed interval | [optional]
**merchant_address1** | Option<**String**> | The street address of the merchant associated with the transaction | [optional]
**merchant_city** | Option<**String**> | The name of the city where the merchant is located | [optional]
**merchant_country** | Option<**String**> | The name of the country where the merchant is located | [optional]
**merchant_latitude** | Option<**String**> | The latitude of the merchant | [optional]
**merchant_logo_url** | Option<**String**> | The URL resource for the merchant's logo | [optional]
**merchant_longitude** | Option<**String**> | The longitude of the merchant | [optional]
**merchant_name** | Option<**String**> | The name of the merchant associated with the transaction | [optional]
**merchant_phone_number** | Option<**String**> | The phone number of the merchant associated with the transaction | [optional]
**merchant_state** | Option<**String**> | The name of the state where the merchant is located | [optional]
**merchant_type** | Option<**String**> | The merchant's type | [optional]
**merchant_zip** | Option<**String**> | The ZIP code where the merchant is located | [optional]
**original_description** | Option<**String**> | The transaction description as received. This will not change | [optional]
**receipt_date** | Option<**chrono::DateTime<chrono::FixedOffset>**> | The date on which FinSight API first received the transaction | [optional]
**request_id** | Option<**String**> | A unique ID for the request the transaction came in with, for debugging purposes | [optional]
**settlement** | Option<**String**> | The settlement type of the transaction (e.g., 'debit' or 'credit') | [optional]
**simple_description** | Option<**String**> | An easy-to-understand, plain-language transaction description | [optional]
**source_id** | Option<**String**> | The source of the transaction | [optional]
**state** | Option<**String**> | The state associated with the transaction | [optional]
**sub_type** | Option<**String**> | A more detailed classification that provides further information on the type of transaction. | [optional]
**subtype** | Option<**String**> | A more detailed classification of the transaction | [optional]
**tenant_id** | Option<**String**> | The ID of the tenant associated with this transaction, if one was included. | [optional]
**transaction_tags** | Option<**Vec<String>**> | The FinSight API issued tags for the transaction | [optional]
**transaction_tags_id** | Option<**Vec<i32>**> | The numeric IDs corresponding to the transaction tags | [optional]
**transactionid** | Option<**String**> | The ID of the transaction as it was originally submitted | [optional]
**r#type** | Option<**String**> | An attribute describing the nature of the intent behind the transaction. | [optional]
**uid** | Option<**String**> | The ID of the user associated with the transaction, as originally submitted | [optional]
**website** | Option<**String**> | The merchant's website URL | [optional]
**zip_code** | Option<**String**> | The ZIP code associated with the transaction | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


