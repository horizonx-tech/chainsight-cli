# type Schema

```txt
undefined#/properties/output/properties/type
```

type of the output. You can use primitive type of Rust or User defined struct

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [algorithm\_lens.json\*](../../out/algorithm_lens.json "open original schema") |

## type Type

`string` ([type](algorithm_lens-properties-output-properties-type.md))

## type Constraints

**pattern**: the string must match the following regular expression:&#x20;

```regexp
^(primitive|struct)$
```

[try pattern](https://regexr.com/?expression=%5E\(primitive%7Cstruct\)%24 "try regular expression with regexr.com")

## type Examples

```json
"primitive"
```

```json
"struct"
```
