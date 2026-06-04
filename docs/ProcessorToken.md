# ProcessorToken

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**token** | **String** | The Token to send to the 3rd party Processor. | [readonly]
**account** | [**models::ProcessorTokenAccount**](ProcessorTokenAccount.md) |  | 
**environment** | [**models::ProcessorTokenEnvironment**](ProcessorTokenEnvironment.md) |  | 
**issuer** | **Issuer** | The issuer of the Processor Token.  - `QUILTT` - Quiltt  - `MX` - MX  - `PLAID` - Plaid  (enum: QUILTT, MX, PLAID) | 
**processor** | **Processor** | The processor that will be receiving the Token. (enum: ACHQ, ADP_ROLL, ADYEN, ALLOY, ALPACA, ANSA, APEX_CLEARING, ARRAY, ASTRA, ATOMIC, BAKKT, BLOOM_CREDIT, BOND, BOOM, BRALE, CARDLESS, CARDLYTICS, CHECK, CHECKBOOK, CHECKOUT, CIRCLE, DRIVEWEALTH, DWOLLA, ESUSU, FIANT, FINIX, FORTRESS_TRUST, GAINBRIDGE, GALILEO, GEMINI, GUSTO, HIGHNOTE, I2C, KNOT, LAYER, LITHIC, LOANPRO, MARQETA, MOCK, MODERN_TREASURY, MOOV, NUVEI, OATFI, OCROLUS, OPEN_LEDGER, PARAFIN, PAYNOTE, PINWHEEL, RISKIFIED, RIZE, SARDINE, SCRIBEUP, SFOX, SILA, SILA_MONEY, SOLID, STAKE, STRADDLE, STRIPE, SVB_API, TABA_PAY, TEAL, THREAD_BANK, TREASURY_PRIME, UNIT, UTB, VALON, VESTA, VOPAY, WEDBUSH, WEPAY, WYRE, ZERO_HASH, ZUM_RAILS) | 
**expires_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> | The UTC date and time when the Processor Token will expire, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format.  Note that some Issuers may not provide an expiration date for the Processor Token. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


