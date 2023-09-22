# output Schema

```txt
undefined#/properties/output
```

output of the algorithm lens

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Allowed               | none                | [algorithm\_lens.json\*](../../out/algorithm_lens.json "open original schema") |

## output Type

`object` ([output](algorithm_lens-properties-output.md))

# output Properties

| Property                 | Type     | Required | Nullable       | Defined by                                                                                                                                              |
| :----------------------- | :------- | :------- | :------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------ |
| [type](#type)            | `string` | Required | cannot be null | [Chainsight Algorithm Lens specification](algorithm_lens-properties-output-properties-type.md "undefined#/properties/output/properties/type")           |
| [type\_name](#type_name) | `string` | Optional | cannot be null | [Chainsight Algorithm Lens specification](algorithm_lens-properties-output-properties-type_name.md "undefined#/properties/output/properties/type_name") |
| [name](#name)            | `string` | Optional | cannot be null | [Chainsight Algorithm Lens specification](algorithm_lens-properties-output-properties-name.md "undefined#/properties/output/properties/name")           |
| [fields](#fields)        | `object` | Optional | cannot be null | [Chainsight Algorithm Lens specification](algorithm_lens-properties-output-properties-fields.md "undefined#/properties/output/properties/fields")       |

## type

type of the output. You can use primitive type of Rust or User defined struct

`type`

*   is required

*   Type: `string` ([type](algorithm_lens-properties-output-properties-type.md))

*   cannot be null

*   defined in: [Chainsight Algorithm Lens specification](algorithm_lens-properties-output-properties-type.md "undefined#/properties/output/properties/type")

### type Type

`string` ([type](algorithm_lens-properties-output-properties-type.md))

### type Constraints

**pattern**: the string must match the following regular expression:&#x20;

```regexp
^(primitive|struct)$
```

[try pattern](https://regexr.com/?expression=%5E\(primitive%7Cstruct\)%24 "try regular expression with regexr.com")

### type Examples

```json
"primitive"
```

```json
"struct"
```

## type\_name

primitive type name. Required if 'output.type' is 'primitive'

`type_name`

*   is optional

*   Type: `string` ([type\_name](algorithm_lens-properties-output-properties-type_name.md))

*   cannot be null

*   defined in: [Chainsight Algorithm Lens specification](algorithm_lens-properties-output-properties-type_name.md "undefined#/properties/output/properties/type_name")

### type\_name Type

`string` ([type\_name](algorithm_lens-properties-output-properties-type_name.md))

### type\_name Examples

```json
"u128"
```

```json
"String"
```

```json
"bool"
```

## name

name of the output struct. Required if 'output.type' is 'struct'

`name`

*   is optional

*   Type: `string` ([name](algorithm_lens-properties-output-properties-name.md))

*   cannot be null

*   defined in: [Chainsight Algorithm Lens specification](algorithm_lens-properties-output-properties-name.md "undefined#/properties/output/properties/name")

### name Type

`string` ([name](algorithm_lens-properties-output-properties-name.md))

### name Constraints

**pattern**: the string must match the following regular expression:&#x20;

```regexp
^[a-zA-Z0-9_]+$
```

[try pattern](https://regexr.com/?expression=%5E%5Ba-zA-Z0-9_%5D%2B%24 "try regular expression with regexr.com")

### name Examples

```json
"ETHUSDPrice"
```

## fields

set of field names and their Rust types. Required if 'output.type' is 'struct'

`fields`

*   is optional

*   Type: `object` ([fields](algorithm_lens-properties-output-properties-fields.md))

*   cannot be null

*   defined in: [Chainsight Algorithm Lens specification](algorithm_lens-properties-output-properties-fields.md "undefined#/properties/output/properties/fields")

### fields Type

`object` ([fields](algorithm_lens-properties-output-properties-fields.md))
