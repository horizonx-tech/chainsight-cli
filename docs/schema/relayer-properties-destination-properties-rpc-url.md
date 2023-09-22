# rpc url Schema

```txt
undefined#/properties/destination/properties/rpc_url
```

rpc url of the destination evm network. only supports https

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                      |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :-------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [relayer.json\*](../../out/relayer.json "open original schema") |

## rpc\_url Type

`string` ([rpc url](relayer-properties-destination-properties-rpc-url.md))

## rpc\_url Constraints

**pattern**: the string must match the following regular expression:&#x20;

```regexp
^https://.*$
```

[try pattern](https://regexr.com/?expression=%5Ehttps%3A%2F%2F.*%24 "try regular expression with regexr.com")

## rpc\_url Examples

```json
"https://rinkeby.infura.io/v3/1a2b3c4d5e"
```
