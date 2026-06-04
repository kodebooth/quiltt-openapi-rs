# AnInvestmentHolding

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**asset_classes** | Option<[**HashSet<models::AnInvestmentHoldingAssetClassesInner>**](AnInvestmentHoldingAssetClassesInner.md)> | Percent breakdown by asset class. | [optional]
**average_cost** | Option<**bool**> | Cost is average of all purchases for holding. | [optional]
**cash_account** | Option<**bool**> | If true, indicates that this holding is used to maintain proceeds from sales, dividends, and other cash postings to the investment account. | [optional]
**change_in_price** | Option<**f64**> | Change in current price compared to previous day's close | [optional]
**currency** | Option<[**models::CurrencyEntity**](CurrencyEntity.md)> |  | [optional]
**current_unit_price** | Option<**f64**> |  | [optional]
**current_unit_price_date** | Option<**chrono::DateTime<chrono::FixedOffset>**> | Current unit price as of date | [optional]
**debt_security** | Option<[**models::DebtSecurityEntity**](DebtSecurityEntity.md)> |  | [optional]
**description** | Option<**String**> | Description of the holding | [optional]
**expiration_date** | Option<**chrono::DateTime<chrono::FixedOffset>**> | For CDs, bonds, and other time-based holdings. | [optional]
**face_value** | Option<**f64**> | Required for bonds. Face value at the time of data retrieved. | [optional]
**fi_asset_classes** | Option<[**Vec<models::AnInvestmentHoldingFiAssetClassesInner>**](AnInvestmentHoldingFiAssetClassesInner.md)> | Percent breakdown by FI-specific asset class percentage breakdown | [optional]
**fi_attributes** | Option<[**HashSet<models::FiAttributeEntity>**](FiAttributeEntity.md)> |  | [optional]
**held_in_account** | Option<**HeldInAccount**> | Sub-account (enum: CASH, MARGIN, SHORT, OTHER) | [optional]
**holding_id** | Option<**String**> | Long term persistent identity of the holding | [optional]
**holding_name** | Option<**String**> | Holding name or security name | [optional]
**holding_sub_type** | Option<**HoldingSubType**> |  (enum: MONEYMARKET, CASH) | [optional]
**holding_type** | Option<**HoldingType**> |  (enum: STOCK, BOND, MUTUALFUND, CD, ANNUITY, OPTION, OTHER) | [optional]
**inv401k_surce** | Option<**Inv401kSurce**> | Source for money for this security.  (enum: PRETAX, AFTERTAX, MATCH, PROFITSHARING, ROLLOVER, OTHERVEST, OTHERNONVEST) | [optional]
**market_value** | Option<**f64**> | Market value at the time of data retrieved | [optional]
**mutual_fund_security** | Option<[**models::MutualFundSecurityEntity**](MutualFundSecurityEntity.md)> |  | [optional]
**option_security** | Option<[**models::OptionSecurityEntity**](OptionSecurityEntity.md)> |  | [optional]
**original_purchase_date** | Option<**chrono::DateTime<chrono::FixedOffset>**> | Date of original purchase | [optional]
**other_security** | Option<[**models::OtherSecurityEntity**](OtherSecurityEntity.md)> |  | [optional]
**position_type** | Option<**PositionType**> |  (enum: LONG, SHORT) | [optional]
**purchased_price** | Option<**f64**> | Price of holding at the time of purchase | [optional]
**rate** | Option<**f64**> | For CDs, bonds, and other rate based holdings. | [optional]
**security_id** | Option<**String**> | Unique identifier of security | [optional]
**security_id_type** | Option<**SecurityIdType**> | Security identifier type (enum: CUSIP, ISIN, SEDOL, SICC, VALOR, WKN) | [optional]
**stock_security** | Option<[**models::StockSecurityEntity**](StockSecurityEntity.md)> |  | [optional]
**sweep_security** | Option<[**models::SweepSecurityEntity**](SweepSecurityEntity.md)> |  | [optional]
**symbol** | Option<**String**> | Ticker / Market symbol | [optional]
**tax_lots** | Option<[**HashSet<models::AnInvestmentHoldingTaxLotsInner>**](AnInvestmentHoldingTaxLotsInner.md)> | Breakdown by tax lot. | [optional]
**units** | Option<**f64**> | Required for stock, mutual funds. Number of shares (with decimals). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


