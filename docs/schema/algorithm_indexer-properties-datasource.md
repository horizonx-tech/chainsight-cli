# datasource Schema

```txt
#/properties/datasource#/properties/datasource
```



| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                           |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Allowed               | none                | [algorithm\_indexer.json\*](../../out/algorithm_indexer.json "open original schema") |

## datasource Type

`object` ([datasource](algorithm_indexer-properties-datasource.md))

# datasource Properties

| Property                     | Type     | Required | Nullable       | Defined by                                                                                                                                                                                                 |
| :--------------------------- | :------- | :------- | :------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [principal](#principal)      | `string` | Required | cannot be null | [Chainsight Event Indexer specification](algorithm_indexer-properties-datasource-properties-principal.md "#/properties/datasource/properties/principal#/properties/datasource/properties/principal")       |
| [input](#input)              | `object` | Required | cannot be null | [Chainsight Event Indexer specification](algorithm_indexer-properties-datasource-properties-input.md "#/properties/datasource/properties/input#/properties/datasource/properties/input")                   |
| [from](#from)                | `number` | Required | cannot be null | [Chainsight Event Indexer specification](algorithm_indexer-properties-datasource-properties-from.md "#/properties/datasource/properties/from#/properties/datasource/properties/from")                      |
| [method](#method)            | `string` | Required | cannot be null | [Chainsight Event Indexer specification](algorithm_indexer-properties-datasource-properties-method.md "#/properties/datasource/properties/method#/properties/datasource/properties/method")                |
| [source\_type](#source_type) | `string` | Required | cannot be null | [Chainsight Event Indexer specification](algorithm_indexer-properties-datasource-properties-source_type.md "#/properties/datasource/properties/source_type#/properties/datasource/properties/source_type") |

## principal

principal of the source canister

`principal`

*   is required

*   Type: `string` ([principal](algorithm_indexer-properties-datasource-properties-principal.md))

*   cannot be null

*   defined in: [Chainsight Event Indexer specification](algorithm_indexer-properties-datasource-properties-principal.md "#/properties/datasource/properties/principal#/properties/datasource/properties/principal")

### principal Type

`string` ([principal](algorithm_indexer-properties-datasource-properties-principal.md))

### principal Examples

```json
"rrkah-fqaaa-aaaaa-aaaaq-cai"
```

```json
"bw4dl-smaaa-aaaaa-qaacq-cai"
```

## input

struct retrived from the source canister

`input`

*   is required

*   Type: `object` ([input](algorithm_indexer-properties-datasource-properties-input.md))

*   cannot be null

*   defined in: [Chainsight Event Indexer specification](algorithm_indexer-properties-datasource-properties-input.md "#/properties/datasource/properties/input#/properties/datasource/properties/input")

### input Type

`object` ([input](algorithm_indexer-properties-datasource-properties-input.md))

## from

key to start the query from

`from`

*   is required

*   Type: `number` ([from](algorithm_indexer-properties-datasource-properties-from.md))

*   cannot be null

*   defined in: [Chainsight Event Indexer specification](algorithm_indexer-properties-datasource-properties-from.md "#/properties/datasource/properties/from#/properties/datasource/properties/from")

### from Type

`number` ([from](algorithm_indexer-properties-datasource-properties-from.md))

### from Examples

```json
0
```

## method

method to query the canister

`method`

*   is required

*   Type: `string` ([method](algorithm_indexer-properties-datasource-properties-method.md))

*   cannot be null

*   defined in: [Chainsight Event Indexer specification](algorithm_indexer-properties-datasource-properties-method.md "#/properties/datasource/properties/method#/properties/datasource/properties/method")

### method Type

`string` ([method](algorithm_indexer-properties-datasource-properties-method.md))

### method Examples

```json
"get_balance"
```

```json
"get_result"
```

## source\_type

type of the source canister

`source_type`

*   is required

*   Type: `string` ([source\_type](algorithm_indexer-properties-datasource-properties-source_type.md))

*   cannot be null

*   defined in: [Chainsight Event Indexer specification](algorithm_indexer-properties-datasource-properties-source_type.md "#/properties/datasource/properties/source_type#/properties/datasource/properties/source_type")

### source\_type Type

`string` ([source\_type](algorithm_indexer-properties-datasource-properties-source_type.md))

### source\_type Constraints

**pattern**: the string must match the following regular expression:&#x20;

```regexp
^(event_indexer|key_value|key_values)$
```

[try pattern](https://regexr.com/?expression=%5E\(event_indexer%7Ckey_value%7Ckey_values\)%24 "try regular expression with regexr.com")

### source\_type Examples

```json
"event_indexer"
```

```json
"key_value"
```

```json
"key_values"
```
