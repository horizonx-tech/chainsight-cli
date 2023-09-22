# type of the canister Schema

```txt
undefined#/properties/metadata/properties/type
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                   |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :--------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [event\_indexer.json\*](../../out/event_indexer.json "open original schema") |

## type Type

`string` ([type of the canister](event_indexer-properties-metadata-properties-type-of-the-canister.md))

## type Constraints

**pattern**: the string must match the following regular expression:&#x20;

```regexp
^(event_indexer|algorithm_indexer|snapshot_indexer|snapshot_json_rpc|relayer|algorithm_lens)$
```

[try pattern](https://regexr.com/?expression=%5E\(event_indexer%7Calgorithm_indexer%7Csnapshot_indexer%7Csnapshot_json_rpc%7Crelayer%7Calgorithm_lens\)%24 "try regular expression with regexr.com")

## type Examples

```json
"event_indexer"
```

```json
"algorithm_indexer"
```

```json
"snapshot_indexer"
```

```json
"snapshot_json_rpc"
```

```json
"relayer"
```

```json
"algorithm_lens"
```
