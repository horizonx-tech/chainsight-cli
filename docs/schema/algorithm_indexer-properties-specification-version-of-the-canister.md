# specification version of the canister Schema

```txt
#/properties/version#/properties/version
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                           |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [algorithm\_indexer.json\*](../../out/algorithm_indexer.json "open original schema") |

## version Type

`string` ([specification version of the canister](algorithm_indexer-properties-specification-version-of-the-canister.md))

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
