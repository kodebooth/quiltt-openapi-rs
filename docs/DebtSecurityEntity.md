# DebtSecurityEntity

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**bond_maturity_date** | Option<**chrono::DateTime<chrono::FixedOffset>**> | Bond Maturity date | [optional]
**call_date** | Option<**chrono::DateTime<chrono::FixedOffset>**> | Next call date | [optional]
**call_price** | Option<**f64**> | Bond call price | [optional]
**call_type** | Option<**CallType**> | Type of next call (enum: CALL, PUT, PREFUND, MATURITY) | [optional]
**coupon_date** | Option<**chrono::DateTime<chrono::FixedOffset>**> | Maturity date for next coupon | [optional]
**coupon_mature_frequency** | Option<**CouponMatureFrequency**> | When coupons mature (enum: MONTHLY, QUARTERLY, SEMIANNUAL, ANNUAL, OTHER) | [optional]
**coupon_rate** | Option<**f64**> | Bond coupon rate for next closest call date | [optional]
**debt_class** | Option<**DebtClass**> | Classification of debt (enum: TREASURY, MUNICIPAL, CORPORATE, OTHER) | [optional]
**debt_type** | Option<**DebtType**> | Debt type (enum: ASSET, COUPON) | [optional]
**par_value** | Option<**f64**> | Par value amount | [optional]
**yield_to_call** | Option<**f64**> | Yield to next call | [optional]
**yield_to_maturity** | Option<**f64**> | Yield to maturity | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


