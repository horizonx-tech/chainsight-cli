# oracle address Schema

```txt
#/properties/destination/properties/oracle_address#/properties/destination/properties/oracle_address
```

address of the destination oracle contract

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                      |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :-------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [relayer.json\*](../../out/relayer.json "open original schema") |

## oracle\_address Type

`string` ([oracle address](relayer-properties-destination-properties-oracle-address.md))

## oracle\_address Constraints

**pattern**: the string must match the following regular expression:&#x20;

```regexp
^(0x)?[0-9a-fA-F]{40}$
```

[try pattern](https://regexr.com/?expression=%5E\(0x\)%3F%5B0-9a-fA-F%5D%7B40%7D%24 "try regular expression with regexr.com")

## oracle\_address Examples

```json
"0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512"
```
