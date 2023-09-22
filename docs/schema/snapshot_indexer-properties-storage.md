# storage Schema

```txt
undefined#/properties/storage
```

storage properties for the canister

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                         |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :--------------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Allowed               | none                | [snapshot\_indexer.json\*](../../out/snapshot_indexer.json "open original schema") |

## storage Type

`object` ([storage](snapshot_indexer-properties-storage.md))

# storage Properties

| Property                           | Type      | Required | Nullable       | Defined by                                                                                                                                                                  |
| :--------------------------------- | :-------- | :------- | :------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [with\_timestamp](#with_timestamp) | `boolean` | Required | cannot be null | [Chainsight Snapshot Indexer specification](snapshot_indexer-properties-storage-properties-with_timestamptamp.md "undefined#/properties/storage/properties/with_timestamp") |

## with\_timestamp

whether to store the data with timestamp

`with_timestamp`

*   is required

*   Type: `boolean` ([with\_timestamptamp](snapshot_indexer-properties-storage-properties-with_timestamptamp.md))

*   cannot be null

*   defined in: [Chainsight Snapshot Indexer specification](snapshot_indexer-properties-storage-properties-with_timestamptamp.md "undefined#/properties/storage/properties/with_timestamp")

### with\_timestamp Type

`boolean` ([with\_timestamptamp](snapshot_indexer-properties-storage-properties-with_timestamptamp.md))

### with\_timestamp Examples

```json
true
```
