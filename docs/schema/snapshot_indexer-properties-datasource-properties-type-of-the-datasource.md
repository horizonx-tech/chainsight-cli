# type of the datasource Schema

```txt
undefined#/properties/datasource/properties/type
```

type of the datasource.If you want to get data from HTTP outcall, you can use HTTP Snapshot Indexer

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                         |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :--------------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [snapshot\_indexer.json\*](../../out/snapshot_indexer.json "open original schema") |

## type Type

`string` ([type of the datasource](snapshot_indexer-properties-datasource-properties-type-of-the-datasource.md))

## type Constraints

**enum**: the value of this property must be equal to one of the following values:

| Value        | Explanation |
| :----------- | :---------- |
| `"contract"` |             |
| `"canister"` |             |

**pattern**: the string must match the following regular expression:&#x20;

```regexp
^(contract|canister)$
```

[try pattern](https://regexr.com/?expression=%5E\(contract%7Ccanister\)%24 "try regular expression with regexr.com")

## type Examples

```json
"contract"
```

```json
"canister"
```
