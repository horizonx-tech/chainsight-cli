# rpc\_url Schema

```txt
undefined#/properties/datasource/properties/network/properties/rpc_url
```

rpc url

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                   |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :--------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [event\_indexer.json\*](../../out/event_indexer.json "open original schema") |

## rpc\_url Type

`string` ([rpc\_url](event_indexer-properties-datasource-properties-network-properties-rpc_url.md))

## rpc\_url Constraints

**pattern**: the string must match the following regular expression:&#x20;

```regexp
^https?://
```

[try pattern](https://regexr.com/?expression=%5Ehttps%3F%3A%2F%2F "try regular expression with regexr.com")

## rpc\_url Examples

```json
"https://mainnet.infura.io/v3/"
```
