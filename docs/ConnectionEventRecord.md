# ConnectionEventRecord

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The ID of the Connection. | [readonly]
**at** | **chrono::DateTime<chrono::FixedOffset>** | The UTC date and time of the most recently known state of the Connection.  For active Connections, this is when data was last successfully synced with the upstream provider. For disconnected Connections, this is the time of the disconnection from the upstream provider. | 
**provider** | **Provider** | Represents the data provider for the Connection.  - `AKOYA` - Akoya  - `FINICITY` - Finicity  - `MOCK` - Mock data  - `MX` - MX  - `PLAID` - Plaid  - `DISCONNECTED` - Disconnected  (enum: AKOYA, FINICITY, MOCK, MX, PLAID, DISCONNECTED) | 
**features** | **Vec<Features>** | The products currently enabled on the Connection. (enum: ACCOUNT_BALANCES_AND_TRANSACTIONS, ACCOUNT_NUMBERS, ACCOUNT_OWNERS, ACCOUNT_STATEMENTS, INVESTMENTS, LIABILITIES, ACCOUNT_BALANCE_REFRESHES, ACCOUNT_TRANSACTIONS_REFRESHES) | 
**products** | **Vec<Products>** | The products currently enabled on the Connection. (enum: ACCOUNT_BALANCES_AND_TRANSACTIONS, ACCOUNT_NUMBERS, ACCOUNT_OWNERS, ACCOUNT_STATEMENTS, INVESTMENTS, LIABILITIES, ACCOUNT_BALANCE_REFRESHES, ACCOUNT_TRANSACTIONS_REFRESHES) | 
**status** | **Status** | Represents the current state of a Connection.  See the [Connection docs](https://quiltt.dev/api/connections) for help troubleshooting error statuses.  Note that this enum is extensible and new values may be added over time.  - `INITIALIZING` - The Connection is being initialized and will begin syncing soon.  - `SYNCING` - The Connection is currently syncing with the provider.  - `SYNCED` - The Connection is synced and up to date with the provider.  - `ERROR_REPAIRABLE` - The Connection must be re-authenticated to resume syncing. Quiltt will keep trying to sync this Connection until it is re-authenticated.  - `ERROR_SERVICE` - Quiltt is experiencing an internal error attempting to sync this Connection. The Quiltt team has been notified and the Connection will be retried automatically once the issue is resolved.  - `ERROR_PROVIDER` - The Connection provider is reporting an error with the Connection. Quiltt will keep trying to sync this Connection until the error is resolved.  - `ERROR_INSTITUTION` - The Connection provider is reporting an error from the user's Institution. Quiltt will keep trying to sync this Connection until the error is resolved.  - `DISCONNECTING` - The Connection is in the process of being disconnected from its provider.  - `DISCONNECTED` - The Connection has been fully disconnected from the provider.  Accounts and associated data will remain on the Profile, but no new data will be synced until another Connection is established to the same Institution.  You can leverage the Connector [Reconnect flow](https://quiltt.dev/connector/reconnect) to prompt the user to re-establish the Connection.  (enum: INITIALIZING, SYNCING, SYNCED, ERROR_REPAIRABLE, ERROR_SERVICE, ERROR_PROVIDER, ERROR_INSTITUTION, DISCONNECTING, DISCONNECTED) | [readonly]
**metadata** | Option<**serde_json::Value**> | Custom metadata about the Connection, stored in a 'key-value' format.  See the [Custom Metadata](https://quiltt.dev/api/custom-metadata) guide for more information and examples. | 
**created_at** | **chrono::DateTime<chrono::FixedOffset>** |  | [readonly]
**updated_at** | **chrono::DateTime<chrono::FixedOffset>** |  | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


