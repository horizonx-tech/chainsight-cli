# type of the datasource Schema

```txt
undefined#/properties/datasource/properties/type
```

currently only supports 'canister'

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                      |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :-------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [relayer.json\*](../../out/relayer.json "open original schema") |

## type Type

`string` ([type of the datasource](relayer-properties-datasource-properties-type-of-the-datasource.md))

## type Constraints

**pattern**: the string must match the following regular expression:&#x20;

```regexp
^(canister)$
```

[try pattern](https://regexr.com/?expression=%5E\(canister\)%24 "try regular expression with regexr.com")

## type Examples

```json
"canister"
```
