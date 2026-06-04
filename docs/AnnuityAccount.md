# AnnuityAccount

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**account_id** | **String** | Long-term persistent identity of the account. Not an account number. This identity must be unique to the owning institution. | 
**account_number_display** | Option<**String**> | Account display number for the end user’s handle at owning institution. This is to be displayed by the Interface Provider. | [optional]
**account_type** | Option<**String**> | The type of an account. For instance, CHECKING, SAVINGS, 401K, etc. | [optional]
**annuity_product_type** | Option<**AnnuityProductType**> |  (enum: CURRENCY, SHARES) | [optional]
**annuity_value_basis** | Option<**AnnuityValueBasis**> |  (enum: FIXED, VARIABLE) | [optional]
**balance_type** | Option<**BalanceType**> | ASSET (positive transaction amount increases balance), LIABILITY (positive transaction amount decreases balance) (enum: ASSET, LIABILITY) | [optional]
**currency** | Option<[**models::CurrencyEntity**](CurrencyEntity.md)> |  | [optional]
**description** | Option<**String**> |  | [optional]
**error** | Option<[**models::ErrorEntity**](ErrorEntity.md)> |  | [optional]
**fi_attributes** | Option<[**Vec<models::FiAttributeEntity>**](FiAttributeEntity.md)> |  | [optional]
**interest_rate** | Option<**f64**> | Interest Rate of Account | [optional]
**interest_rate_as_of** | Option<**chrono::DateTime<chrono::FixedOffset>**> | Date of account’s interest rate | [optional]
**interest_rate_type** | Option<**InterestRateType**> | The type of interest rate. FIXED or VARIABLE. (enum: FIXED, VARIABLE) | [optional]
**last_activity_date** | Option<**chrono::DateTime<chrono::FixedOffset>**> | Date that last transaction occurred on account | [optional]
**line_of_business** | Option<**String**> | The line of business, such as consumer, consumer joint, small business, corporate, etc. | [optional]
**micr_number** | Option<**String**> | MICR Number | [optional]
**nickname** | Option<**String**> | Name given by the user. Used in UIs to assist in account selection | [optional]
**parent_account_id** | Option<**String**> | Long-term persistent identity of the parent account. This is used to group accounts. | [optional]
**payment_frequency** | Option<**PaymentFrequency**> |  (enum: ANNUALLY, QUARTERLY, MONTHLY, WEEKLY) | [optional]
**prior_interest_rate** | Option<**f64**> | Previous Interest Rate of Account | [optional]
**product_name** | Option<**String**> | Marketed product name for this account.  Used in UIs to assist in account selection           | [optional]
**status** | Option<**Status**> | The status of an account. (enum: OPEN, CLOSED, PENDINGOPEN, PENDINGCLOSE, DELINQUENT, PAID, NEGATIVECURRENTBALANCE) | [optional]
**transfer_in** | Option<**bool**> | Account is eligible for incoming transfers | [optional]
**transfer_out** | Option<**bool**> | Account is eligible for outgoing transfers | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


