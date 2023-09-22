# rpc url Schema

```txt
undefined#/properties/datasource/properties/location/properties/args/properties/rpc_url
```

rpc url of the datasource. It is required if type is contract.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                         |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :--------------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [snapshot\_indexer.json\*](../../out/snapshot_indexer.json "open original schema") |

## rpc\_url Type

`string` ([rpc url](snapshot_indexer-properties-datasource-properties-location-of-the-datasource-properties-args-properties-rpc-url.md))

## rpc\_url Constraints

**pattern**: the string must match the following regular expression:&#x20;

```regexp
^(http|https)://
```

[try pattern](https://regexr.com/?expression=%5E\(http%7Chttps\)%3A%2F%2F "try regular expression with regexr.com")

## rpc\_url Examples

```json
"https://mainnet.infura.io/v3/YOUR_API_KEY"
```
