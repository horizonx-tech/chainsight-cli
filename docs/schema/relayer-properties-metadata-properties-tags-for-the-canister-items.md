# Untitled string in Chainsight Relayer specification Schema

```txt
undefined#/properties/metadata/properties/tags/items
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                      |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :-------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [relayer.json\*](../../out/relayer.json "open original schema") |

## items Type

`string`

## items Constraints

**pattern**: the string must match the following regular expression:&#x20;

```regexp
^[a-zA-Z0-9_-]+$
```

[try pattern](https://regexr.com/?expression=%5E%5Ba-zA-Z0-9_-%5D%2B%24 "try regular expression with regexr.com")

## items Examples

```json
"Ethereum"
```

```json
"Relayer"
```

```json
"Account"
```
