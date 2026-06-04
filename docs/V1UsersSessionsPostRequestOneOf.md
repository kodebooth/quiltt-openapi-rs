# V1UsersSessionsPostRequestOneOf

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**email** | Option<**String**> | The email associated with the Profile.  This field can be used to power passwordless authentication in the Connector. | [optional]
**phone** | Option<**String**> | The phone number associated with the Profile, in E164 Format.  This field can be used to power passwordless authentication in the Connector. | [optional]
**name** | Option<**String**> | A common name or nickname for the Profile. | [optional]
**names** | Option<[**models::ProfileNames**](ProfileNames.md)> |  | [optional]
**date_of_birth** | Option<**chrono::NaiveDate**> | A physical person's date of birth. | [optional]
**address** | Option<[**models::ProfileAddress**](ProfileAddress.md)> |  | [optional]
**metadata** | Option<**serde_json::Value**> | Custom metadata about the Profile, stored in a 'key-value' format.  See the [Custom Metadata](https://quiltt.dev/api/custom-metadata) guide for more information and examples. | [optional]
**user_id** | [**models::ExistingProfileUserId**](ExistingProfileUserId.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


