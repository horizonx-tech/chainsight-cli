# location of the datasource Schema

```txt
undefined#/properties/datasource/properties/location
```

location of the datasource. For contract, it is the contract address. For canister, it is the canister id or canister name defined in dfx.json.

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                         |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :--------------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Allowed               | none                | [snapshot\_indexer.json\*](../../out/snapshot_indexer.json "open original schema") |

## location Type

`object` ([location of the datasource](snapshot_indexer-properties-datasource-properties-location-of-the-datasource.md))

# location Properties

| Property      | Type     | Required | Nullable       | Defined by                                                                                                                                                                                                                        |
| :------------ | :------- | :------- | :------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [id](#id)     | `string` | Required | cannot be null | [Chainsight Snapshot Indexer specification](snapshot_indexer-properties-datasource-properties-location-of-the-datasource-properties-id-of-the-datasource.md "undefined#/properties/datasource/properties/location/properties/id") |
| [args](#args) | `object` | Required | cannot be null | [Chainsight Snapshot Indexer specification](snapshot_indexer-properties-datasource-properties-location-of-the-datasource-properties-args.md "undefined#/properties/datasource/properties/location/properties/args")               |

## id

id of the datasource. For contract, it is the contract address. For canister, it is the canister id or canister name defined in dfx.json.

`id`

*   is required

*   Type: `string` ([id of the datasource](snapshot_indexer-properties-datasource-properties-location-of-the-datasource-properties-id-of-the-datasource.md))

*   cannot be null

*   defined in: [Chainsight Snapshot Indexer specification](snapshot_indexer-properties-datasource-properties-location-of-the-datasource-properties-id-of-the-datasource.md "undefined#/properties/datasource/properties/location/properties/id")

### id Type

`string` ([id of the datasource](snapshot_indexer-properties-datasource-properties-location-of-the-datasource-properties-id-of-the-datasource.md))

### id Examples

```json
"9fE46736679d2D9a65F0992F2272dE9f3c7fa6e0"
```

```json
"0x9fE46736679d2D9a65F0992F2272dE9f3c7fa6e0"
```

```json
"pj_snapshot_chain"
```

```json
"rrkah-fqaaa-aaaaa-aaaaq-cai"
```

## args

args for the datasource.

`args`

*   is required

*   Type: `object` ([args](snapshot_indexer-properties-datasource-properties-location-of-the-datasource-properties-args.md))

*   cannot be null

*   defined in: [Chainsight Snapshot Indexer specification](snapshot_indexer-properties-datasource-properties-location-of-the-datasource-properties-args.md "undefined#/properties/datasource/properties/location/properties/args")

### args Type

`object` ([args](snapshot_indexer-properties-datasource-properties-location-of-the-datasource-properties-args.md))
