# network id Schema

```txt
undefined#/properties/datasource/properties/location/properties/args/properties/network_id
```

chain id. It is required if type is contract.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                         |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :--------------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [snapshot\_indexer.json\*](../../out/snapshot_indexer.json "open original schema") |

## network\_id Type

`integer` ([network id](snapshot_indexer-properties-datasource-properties-location-of-the-datasource-properties-args-properties-network-id.md))

## network\_id Constraints

**pattern**: the string must match the following regular expression:&#x20;

```regexp
^[0-9]+$
```

[try pattern](https://regexr.com/?expression=%5E%5B0-9%5D%2B%24 "try regular expression with regexr.com")

## network\_id Examples

```json
1
```

```json
31337
```
