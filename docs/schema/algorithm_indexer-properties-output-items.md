# Untitled object in undefined Schema

```txt
undefined#/properties/output/items
```



| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                           |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Allowed               | none                | [algorithm\_indexer.json\*](../../out/algorithm_indexer.json "open original schema") |

## items Type

`object` ([Details](algorithm_indexer-properties-output-items.md))

# items Properties

| Property                     | Type     | Required | Nullable       | Defined by                                                                                                                                         |
| :--------------------------- | :------- | :------- | :------------- | :------------------------------------------------------------------------------------------------------------------------------------------------- |
| [name](#name)                | `string` | Required | cannot be null | [Untitled schema](algorithm_indexer-properties-output-items-properties-name-of-the-struct.md "undefined#/properties/output/items/properties/name") |
| [fields](#fields)            | `object` | Required | cannot be null | [Untitled schema](algorithm_indexer-properties-output-items-properties-fields.md "undefined#/properties/output/items/properties/fields")           |
| [output\_type](#output_type) | `string` | Required | cannot be null | [Untitled schema](algorithm_indexer-properties-output-items-properties-output_type.md "undefined#/properties/output/items/properties/output_type") |

## name



`name`

*   is required

*   Type: `string` ([name of the struct](algorithm_indexer-properties-output-items-properties-name-of-the-struct.md))

*   cannot be null

*   defined in: [Untitled schema](algorithm_indexer-properties-output-items-properties-name-of-the-struct.md "undefined#/properties/output/items/properties/name")

### name Type

`string` ([name of the struct](algorithm_indexer-properties-output-items-properties-name-of-the-struct.md))

### name Constraints

**pattern**: the string must match the following regular expression:&#x20;

```regexp
^[a-zA-Z0-9_-]+$
```

[try pattern](https://regexr.com/?expression=%5E%5Ba-zA-Z0-9_-%5D%2B%24 "try regular expression with regexr.com")

### name Examples

```json
"AccountBalance"
```

## fields

field names and rust types of the struct

`fields`

*   is required

*   Type: `object` ([fields](algorithm_indexer-properties-output-items-properties-fields.md))

*   cannot be null

*   defined in: [Untitled schema](algorithm_indexer-properties-output-items-properties-fields.md "undefined#/properties/output/items/properties/fields")

### fields Type

`object` ([fields](algorithm_indexer-properties-output-items-properties-fields.md))

## output\_type

type of the output. KeyValues: HashMap\<Key, Vec<Values>>, KeyValue: HashMap\<Key, Value>

`output_type`

*   is required

*   Type: `string` ([output\_type](algorithm_indexer-properties-output-items-properties-output_type.md))

*   cannot be null

*   defined in: [Untitled schema](algorithm_indexer-properties-output-items-properties-output_type.md "undefined#/properties/output/items/properties/output_type")

### output\_type Type

`string` ([output\_type](algorithm_indexer-properties-output-items-properties-output_type.md))

### output\_type Constraints

**pattern**: the string must match the following regular expression:&#x20;

```regexp
^(key_values|key_value)$
```

[try pattern](https://regexr.com/?expression=%5E\(key_values%7Ckey_value\)%24 "try regular expression with regexr.com")

### output\_type Examples

```json
"key_values"
```

```json
"key_value"
```
