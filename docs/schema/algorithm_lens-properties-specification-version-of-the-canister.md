# specification version of the canister Schema

```txt
undefined#/properties/version
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [algorithm\_lens.json\*](../../out/algorithm_lens.json "open original schema") |

## version Type

`string` ([specification version of the canister](algorithm_lens-properties-specification-version-of-the-canister.md))

## version Constraints

**pattern**: the string must match the following regular expression:&#x20;

```regexp
^(v1)$
```

[try pattern](https://regexr.com/?expression=%5E\(v1\)%24 "try regular expression with regexr.com")

## version Examples

```json
"v1"
```