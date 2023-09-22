# network id Schema

```txt
#/properties/destination/properties/network_id#/properties/destination/properties/network_id
```

network id of the destination evm network

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                      |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :-------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [relayer.json\*](../../out/relayer.json "open original schema") |

## network\_id Type

`number` ([network id](relayer-properties-destination-properties-network-id.md))

## network\_id Constraints

**pattern**: the string must match the following regular expression:&#x20;

```regexp
^[0-9]+$
```

[try pattern](https://regexr.com/?expression=%5E%5B0-9%5D%2B%24 "try regular expression with regexr.com")

## network\_id Examples

```json
31337
```

```json
1
```

```json
4
```
