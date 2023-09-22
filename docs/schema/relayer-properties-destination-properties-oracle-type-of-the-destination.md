# oracle type of the destination Schema

```txt
undefined#/properties/destination/properties/type
```

oracle type of the destination. currently we don't suport user definined oracle types

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                      |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :-------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [relayer.json\*](../../out/relayer.json "open original schema") |

## type Type

`string` ([oracle type of the destination](relayer-properties-destination-properties-oracle-type-of-the-destination.md))

## type Constraints

**pattern**: the string must match the following regular expression:&#x20;

```regexp
^(uint256|uint128|uint64|string)$
```

[try pattern](https://regexr.com/?expression=%5E\(uint256%7Cuint128%7Cuint64%7Cstring\)%24 "try regular expression with regexr.com")

## type Examples

```json
"uint256"
```

```json
"uint128"
```

```json
"uint64"
```

```json
"string"
```
