# RemoteEnrichmentRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**profile_id** | **String** | The ID of the Profile. | 
**account_type** | **AccountType** | The Account type the payload is associated with. (enum: CHECKING, SAVINGS, CREDIT_CARD) | 
**provider** | **Provider** | Enum for the API Provider of the payload to be enriched.  If your provider is not listed, format your data to match the QuilttPayload schema, and supply `GENERIC`. (enum: AKOYA, FINICITY, GENERIC, MX, PLAID, TREASURY_PRIME) | 
**payload** | [**models::RemoteEnrichmentRequestPayload**](RemoteEnrichmentRequestPayload.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


