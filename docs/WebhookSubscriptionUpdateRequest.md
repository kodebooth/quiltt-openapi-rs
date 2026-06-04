# WebhookSubscriptionUpdateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | The name of the Webhook Subscription. | [optional]
**target_url** | Option<**String**> | The URL that will receive the Webhook Payloads. | [optional]
**event_types** | Option<**Vec<EventTypes>**> | The list of subscribed Event types. Note that parent scopes like `connection` will include child Events like `connection.created`, `connection.synced.successful`, etc. (enum: profile, profile.created, profile.ready, profile.deleted, connection, connection.created, connection.synced, connection.synced.successful, connection.synced.successful.initial, connection.synced.successful.historical, connection.synced.errored, connection.synced.errored.repairable, connection.synced.errored.institution, connection.synced.errored.provider, connection.synced.errored.service, connection.disconnected, account, account.created, account.owners_verified, account.reconnected, account.verified, balance, balance.created, statement, statement.ready, charge.execute) | [optional]
**disabled** | Option<**bool**> | Specifies whether the Webhook Subscription is disabled. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


