# source\_type Schema

```txt
#/properties/datasource/properties/source_type#/properties/datasource/properties/source_type
```

type of the source canister

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                           |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [algorithm\_indexer.json\*](../../out/algorithm_indexer.json "open original schema") |

## source\_type Type

`string` ([source\_type](algorithm_indexer-properties-datasource-properties-source_type.md))

## source\_type Constraints

**pattern**: the string must match the following regular expression:&#x20;

```regexp
^(event_indexer|key_value|key_values)$
```

[try pattern](https://regexr.com/?expression=%5E\(event_indexer%7Ckey_value%7Ckey_values\)%24 "try regular expression with regexr.com")

## source\_type Examples

```json
"event_indexer"
```

```json
"key_value"
```

```json
"key_values"
```
