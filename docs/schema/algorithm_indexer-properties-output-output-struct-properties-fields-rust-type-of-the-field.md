# rust type of the field Schema

```txt
undefined#/properties/output/items/properties/fields/additionalProperties
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                           |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [algorithm\_indexer.json\*](../../out/algorithm_indexer.json "open original schema") |

## additionalProperties Type

`string` ([rust type of the field](algorithm_indexer-properties-output-output-struct-properties-fields-rust-type-of-the-field.md))

## additionalProperties Constraints

**pattern**: the string must match the following regular expression:&#x20;

```regexp
^[a-zA-Z0-9_<>]+$
```

[try pattern](https://regexr.com/?expression=%5E%5Ba-zA-Z0-9_%3C%3E%5D%2B%24 "try regular expression with regexr.com")

## additionalProperties Examples

```json
"String"
```

```json
"u128"
```
