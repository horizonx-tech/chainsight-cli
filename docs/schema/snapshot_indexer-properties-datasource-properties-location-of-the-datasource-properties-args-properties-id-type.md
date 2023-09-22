# id type Schema

```txt
undefined#/properties/datasource/properties/location/properties/args/properties/id_type
```

type of the id. It is required if type is canister.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                         |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :--------------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [snapshot\_indexer.json\*](../../out/snapshot_indexer.json "open original schema") |

## id\_type Type

`string` ([id type](snapshot_indexer-properties-datasource-properties-location-of-the-datasource-properties-args-properties-id-type.md))

## id\_type Constraints

**enum**: the value of this property must be equal to one of the following values:

| Value             | Explanation |
| :---------------- | :---------- |
| `"canister_id"`   |             |
| `"canister_name"` |             |

**pattern**: the string must match the following regular expression:&#x20;

```regexp
^(canister_id|canister_name)$
```

[try pattern](https://regexr.com/?expression=%5E\(canister_id%7Ccanister_name\)%24 "try regular expression with regexr.com")

## id\_type Examples

```json
"canister_id"
```

```json
"canister_name"
```
