# WebhookSubscription

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The ID of the Webhook Subscription. | [readonly]
**event_types** | **Vec<EventTypes>** | The list of subscribed Event types. Note that parent scopes like `connection` will include child Events like `connection.created`, `connection.synced.successful`, etc. (enum: profile, profile.created, profile.ready, profile.deleted, connection, connection.created, connection.synced, connection.synced.successful, connection.synced.successful.initial, connection.synced.successful.historical, connection.synced.errored, connection.synced.errored.repairable, connection.synced.errored.institution, connection.synced.errored.provider, connection.synced.errored.service, connection.disconnected, account, account.created, account.owners_verified, account.reconnected, account.verified, balance, balance.created, statement, statement.ready, charge.execute) | 
**name** | **String** | The name of the Webhook Subscription. | 
**target_url** | **String** | The URL that will receive the Webhook Payloads. | 
**disabled** | **bool** | Specifies whether the Webhook Subscription is disabled. | 
**created_at** | **chrono::DateTime<chrono::FixedOffset>** | The UTC date and time when the Webhook Subscription was created, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format. | [readonly]
**updated_at** | **chrono::DateTime<chrono::FixedOffset>** | The UTC date and time when the Webhook Subscription was last updated, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format. | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


