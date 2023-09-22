# name Schema

```txt
undefined#/properties/output/properties/name
```

name of the output struct. Required if 'output.type' is 'struct'

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [algorithm\_lens.json\*](../../out/algorithm_lens.json "open original schema") |

## name Type

`string` ([name](algorithm_lens-properties-output-properties-name.md))

## name Constraints

**pattern**: the string must match the following regular expression:&#x20;

```regexp
^[a-zA-Z0-9_]+$
```

[try pattern](https://regexr.com/?expression=%5E%5Ba-zA-Z0-9_%5D%2B%24 "try regular expression with regexr.com")

## name Examples

```json
"ETHUSDPrice"
```
