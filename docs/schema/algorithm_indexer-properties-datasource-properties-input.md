# input Schema

```txt
#/properties/datasource/properties/input#/properties/datasource/properties/input
```

struct retrived from the source canister

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                           |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Allowed               | none                | [algorithm\_indexer.json\*](../../out/algorithm_indexer.json "open original schema") |

## input Type

`object` ([input](algorithm_indexer-properties-datasource-properties-input.md))

# input Properties

| Property          | Type     | Required | Nullable       | Defined by                                                                                                                                                                                                                                             |
| :---------------- | :------- | :------- | :------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [name](#name)     | `string` | Required | cannot be null | [Chainsight Event Indexer specification](algorithm_indexer-properties-datasource-properties-input-properties-name-of-the-struct.md "#/properties/datasource/properties/input/properties/name#/properties/datasource/properties/input/properties/name") |
| [fields](#fields) | `object` | Required | cannot be null | [Chainsight Event Indexer specification](algorithm_indexer-properties-datasource-properties-input-properties-fields.md "#/properties/datasource/properties/input/properties/fields#/properties/datasource/properties/input/properties/fields")         |

## name



`name`

*   is required

*   Type: `string` ([name of the struct](algorithm_indexer-properties-datasource-properties-input-properties-name-of-the-struct.md))

*   cannot be null

*   defined in: [Chainsight Event Indexer specification](algorithm_indexer-properties-datasource-properties-input-properties-name-of-the-struct.md "#/properties/datasource/properties/input/properties/name#/properties/datasource/properties/input/properties/name")

### name Type

`string` ([name of the struct](algorithm_indexer-properties-datasource-properties-input-properties-name-of-the-struct.md))

### name Constraints

**pattern**: the string must match the following regular expression:&#x20;

```regexp
^[a-zA-Z0-9_-]+$
```

[try pattern](https://regexr.com/?expression=%5E%5Ba-zA-Z0-9_-%5D%2B%24 "try regular expression with regexr.com")

### name Examples

```json
"Relayer"
```

## fields

field names and rust types of the struct

`fields`

*   is required

*   Type: `object` ([fields](algorithm_indexer-properties-datasource-properties-input-properties-fields.md))

*   cannot be null

*   defined in: [Chainsight Event Indexer specification](algorithm_indexer-properties-datasource-properties-input-properties-fields.md "#/properties/datasource/properties/input/properties/fields#/properties/datasource/properties/input/properties/fields")

### fields Type

`object` ([fields](algorithm_indexer-properties-datasource-properties-input-properties-fields.md))
