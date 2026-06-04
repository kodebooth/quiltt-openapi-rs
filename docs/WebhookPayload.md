# WebhookPayload

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**environment** | [**models::Environment**](Environment.md) |  | 
**event_types** | **Vec<EventTypes>** | The types of events included in the Payload. (enum: connection.created, connection.synced.successful, connection.synced.successful.initial, connection.synced.successful.historical, connection.synced.errored.repairable, connection.synced.errored.institution, connection.synced.errored.provider, connection.synced.errored.service, connection.disconnected, account.created, account.owners_verified, account.reconnected, account.verified, balance.created, statement.ready, profile.created, profile.ready, profile.deleted) | [readonly]
**events** | [**Vec<models::WebhookPayloadEventsInner>**](WebhookPayloadEventsInner.md) | The events included in this Payload. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


