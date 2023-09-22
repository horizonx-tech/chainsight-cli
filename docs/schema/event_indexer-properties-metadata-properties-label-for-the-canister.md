# label for the canister Schema

```txt
undefined#/properties/metadata/properties/label
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                   |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :--------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [event\_indexer.json\*](../../out/event_indexer.json "open original schema") |

## label Type

`string` ([label for the canister](event_indexer-properties-metadata-properties-label-for-the-canister.md))

## label Constraints

**pattern**: the string must match the following regular expression:&#x20;

```regexp
^[a-z0-9_]+$
```

[try pattern](https://regexr.com/?expression=%5E%5Ba-z0-9_%5D%2B%24 "try regular expression with regexr.com")

## label Examples

```json
"relayer_ethusd"
```
