# method Schema

```txt
undefined#/properties/datasource/properties/method
```

method of the datasource. The canister will call this method to get data.

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                         |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :--------------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Allowed               | none                | [snapshot\_indexer.json\*](../../out/snapshot_indexer.json "open original schema") |

## method Type

`object` ([method](snapshot_indexer-properties-datasource-properties-method.md))

# method Properties

| Property                  | Type     | Required | Nullable       | Defined by                                                                                                                                                                                                              |
| :------------------------ | :------- | :------- | :------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [identifier](#identifier) | `string` | Required | cannot be null | [Chainsight Snapshot Indexer specification](snapshot_indexer-properties-datasource-properties-method-properties-identifier-of-the-method.md "undefined#/properties/datasource/properties/method/properties/identifier") |
| [interface](#interface)   | `string` | Optional | cannot be null | [Chainsight Snapshot Indexer specification](snapshot_indexer-properties-datasource-properties-method-properties-interface.md "undefined#/properties/datasource/properties/method/properties/interface")                 |
| [args](#args)             | `array`  | Optional | cannot be null | [Chainsight Snapshot Indexer specification](snapshot_indexer-properties-datasource-properties-method-properties-args.md "undefined#/properties/datasource/properties/method/properties/args")                           |

## identifier

contract of candid function and its return values to call.

`identifier`

*   is required

*   Type: `string` ([identifier of the method](snapshot_indexer-properties-datasource-properties-method-properties-identifier-of-the-method.md))

*   cannot be null

*   defined in: [Chainsight Snapshot Indexer specification](snapshot_indexer-properties-datasource-properties-method-properties-identifier-of-the-method.md "undefined#/properties/datasource/properties/method/properties/identifier")

### identifier Type

`string` ([identifier of the method](snapshot_indexer-properties-datasource-properties-method-properties-identifier-of-the-method.md))

### identifier Examples

```json
"latestAnswer():(uint256)"
```

```json
"balanceOf(address):(uint256)"
```

```json
"get_last_snapshot : () -> (record { value : text; timestamp : nat64 })"
```

## interface

abi json file to use. This file must be in ./interfaces folder. It is required if type is contract.

`interface`

*   is optional

*   Type: `string` ([interface](snapshot_indexer-properties-datasource-properties-method-properties-interface.md))

*   cannot be null

*   defined in: [Chainsight Snapshot Indexer specification](snapshot_indexer-properties-datasource-properties-method-properties-interface.md "undefined#/properties/datasource/properties/method/properties/interface")

### interface Type

`string` ([interface](snapshot_indexer-properties-datasource-properties-method-properties-interface.md))

### interface Examples

```json
"AggregatorV3Interface.json"
```

```json
"IERC20.json"
```

## args

args for the method

`args`

*   is optional

*   Type: `array` ([args](snapshot_indexer-properties-datasource-properties-method-properties-args.md))

*   cannot be null

*   defined in: [Chainsight Snapshot Indexer specification](snapshot_indexer-properties-datasource-properties-method-properties-args.md "undefined#/properties/datasource/properties/method/properties/args")

### args Type

`array` ([args](snapshot_indexer-properties-datasource-properties-method-properties-args.md))

### args Examples

```json
1
```

```json
"0x9fE46736679d2D9a65F0992F2272dE9f3c7fa6e0"
```
