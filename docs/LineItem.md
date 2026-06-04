# LineItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**amount** | Option<**f64**> | The amount of money attributable to this line item | [optional]
**check_number** | Option<**f64**> | Check number | [optional]
**description** | Option<**String**> | The description of the line item | [optional]
**image_ids** | Option<**Vec<String>**> | Array of image identifiers (unique to transaction) used to retrieve images of check or transaction receipt | [optional]
**links** | Option<[**Vec<models::HateoasLink>**](HATEOASLink.md)> | Links (unique to this Transaction) used to retrieve images of checks or transaction receipts, or invoke other APIs | [optional]
**memo** | Option<**String**> | Secondary item description | [optional]
**reference** | Option<**String**> | A reference number | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


