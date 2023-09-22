# canister id Schema

```txt
undefined#/properties/datasource/properties/location/properties/id
```

canister id or name of the datasource

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                      |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :-------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [relayer.json\*](../../out/relayer.json "open original schema") |

## id Type

`string` ([canister id](relayer-properties-datasource-properties-datasource-location-properties-canister-id.md))

## id Constraints

**pattern**: the string must match the following regular expression:&#x20;

```regexp
^[a-z0-9_-]+$
```

[try pattern](https://regexr.com/?expression=%5E%5Ba-z0-9_-%5D%2B%24 "try regular expression with regexr.com")

## id Examples

```json
"algorithm_lens_ethusd"
```

```json
"bw4dl-smaaa-aaaaa-qaacq-cai"
```
