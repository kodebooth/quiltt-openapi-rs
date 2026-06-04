# ProfileEventRecord

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The ID of the Profile. | [readonly]
**uuid** | **uuid::Uuid** | The UUID of the Profile. | [readonly]
**metadata** | Option<**serde_json::Value**> | Custom metadata about the Profile, stored in a 'key-value' format.  See the [Custom Metadata](https://quiltt.dev/api/custom-metadata) guide for more information and examples. | 
**email** | Option<**String**> | The email associated with the Profile.  This field can be used to power passwordless authentication in the Connector. | 
**phone** | Option<**String**> | The phone number associated with the Profile, in E164 Format.  This field can be used to power passwordless authentication in the Connector. | 
**name** | Option<**String**> | A common name or nickname for the Profile. | 
**names** | Option<[**models::ProfileNames**](ProfileNames.md)> |  | 
**date_of_birth** | Option<**chrono::NaiveDate**> | A physical person's date of birth. | 
**address** | Option<[**models::ProfileAddress**](ProfileAddress.md)> |  | 
**at** | **chrono::DateTime<chrono::FixedOffset>** | The UTC date and time of the most recently known state of the Profile.  This typically represents the time when Profile was last updated or synced with an upstream provider. | [readonly]
**created_at** | **chrono::DateTime<chrono::FixedOffset>** | The UTC date and time when the Profile was created, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format. | [readonly]
**updated_at** | **chrono::DateTime<chrono::FixedOffset>** |  | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


