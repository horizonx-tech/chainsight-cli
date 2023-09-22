# output\_type Schema

```txt
#/properties/output/items/properties/output_type#/properties/output/items/properties/output_type
```

type of the output. KeyValues: HashMap\<Key, Vec<Values>>, KeyValue: HashMap\<Key, Value>

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                           |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [algorithm\_indexer.json\*](../../out/algorithm_indexer.json "open original schema") |

## output\_type Type

`string` ([output\_type](algorithm_indexer-properties-output-output-struct-properties-output_type.md))

## output\_type Constraints

**pattern**: the string must match the following regular expression:&#x20;

```regexp
^(key_values|key_value)$
```

[try pattern](https://regexr.com/?expression=%5E\(key_values%7Ckey_value\)%24 "try regular expression with regexr.com")

## output\_type Examples

```json
"key_values"
```

```json
"key_value"
```
