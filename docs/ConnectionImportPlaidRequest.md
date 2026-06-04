# ConnectionImportPlaidRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**access_token** | **String** | The token to import from Plaid. | 
**mode** | Option<**Mode**> | The mode of the import operation. Use `BACKGROUND` when importing batches of Connections, or when you don't need immediate access to synced data.  - `REALTIME`: Syncing process will be kicked off immediately. - `BACKGROUND`: Syncing will occur shortly after import, with a slight delay. (enum: REALTIME, BACKGROUND) | [optional][default to Realtime]
**externally_managed** | Option<**bool**> | Specifies whether this Connection is managed by an external system.  When a Connection is imported as externally-managed, Quiltt will sync it, but disconnecting it will not revoke access from the upstream provider. | [optional][default to false]
**migrate_webhook** | Option<**bool**> | Whether to migrate the webhook registered with Plaid to be consumed by Quiltt.  When set to `false`, Quiltt will preserve the existing webhook registered with Plaid and the Connection will sync less frequently. | [optional][default to true]
**created_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> | The timestamp when the Plaid Item was created, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format. | [optional]
**products** | Option<**Vec<Products>**> | The authorized Plaid products to sync on the Connection.  These products will be added to the products Quiltt can detect automatically. Note that `balance`, `investments` and `liabilities` are always detected automatically. (enum: auth, identity, transactions) | [optional]
**institution_id** | Option<**String**> | The Plaid Institution ID, to be supplied only if directed by Quiltt | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


