# identifier of the method Schema

```txt
undefined#/properties/datasource/properties/method/properties/identifier
```

contract of candid function and its return values to call.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                         |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :--------------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [snapshot\_indexer.json\*](../../out/snapshot_indexer.json "open original schema") |

## identifier Type

`string` ([identifier of the method](snapshot_indexer-properties-datasource-properties-method-properties-identifier-of-the-method.md))

## identifier Examples

```json
"latestAnswer():(uint256)"
```

```json
"balanceOf(address):(uint256)"
```

```json
"get_last_snapshot : () -> (record { value : text; timestamp : nat64 })"
```
