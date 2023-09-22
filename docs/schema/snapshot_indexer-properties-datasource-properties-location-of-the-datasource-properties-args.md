# args Schema

```txt
undefined#/properties/datasource/properties/location/properties/args
```

args for the datasource.

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                         |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :--------------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Allowed               | none                | [snapshot\_indexer.json\*](../../out/snapshot_indexer.json "open original schema") |

## args Type

`object` ([args](snapshot_indexer-properties-datasource-properties-location-of-the-datasource-properties-args.md))

# args Properties

| Property                   | Type      | Required | Nullable       | Defined by                                                                                                                                                                                                                                                      |
| :------------------------- | :-------- | :------- | :------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [network\_id](#network_id) | `integer` | Optional | cannot be null | [Chainsight Snapshot Indexer specification](snapshot_indexer-properties-datasource-properties-location-of-the-datasource-properties-args-properties-network-id.md "undefined#/properties/datasource/properties/location/properties/args/properties/network_id") |
| [rpc\_url](#rpc_url)       | `string`  | Optional | cannot be null | [Chainsight Snapshot Indexer specification](snapshot_indexer-properties-datasource-properties-location-of-the-datasource-properties-args-properties-rpc-url.md "undefined#/properties/datasource/properties/location/properties/args/properties/rpc_url")       |
| [id\_type](#id_type)       | `string`  | Optional | cannot be null | [Chainsight Snapshot Indexer specification](snapshot_indexer-properties-datasource-properties-location-of-the-datasource-properties-args-properties-id-type.md "undefined#/properties/datasource/properties/location/properties/args/properties/id_type")       |

## network\_id

chain id. It is required if type is contract.

`network_id`

*   is optional

*   Type: `integer` ([network id](snapshot_indexer-properties-datasource-properties-location-of-the-datasource-properties-args-properties-network-id.md))

*   cannot be null

*   defined in: [Chainsight Snapshot Indexer specification](snapshot_indexer-properties-datasource-properties-location-of-the-datasource-properties-args-properties-network-id.md "undefined#/properties/datasource/properties/location/properties/args/properties/network_id")

### network\_id Type

`integer` ([network id](snapshot_indexer-properties-datasource-properties-location-of-the-datasource-properties-args-properties-network-id.md))

### network\_id Constraints

**pattern**: the string must match the following regular expression:&#x20;

```regexp
^[0-9]+$
```

[try pattern](https://regexr.com/?expression=%5E%5B0-9%5D%2B%24 "try regular expression with regexr.com")

### network\_id Examples

```json
1
```

```json
31337
```

## rpc\_url

rpc url of the datasource. It is required if type is contract.

`rpc_url`

*   is optional

*   Type: `string` ([rpc url](snapshot_indexer-properties-datasource-properties-location-of-the-datasource-properties-args-properties-rpc-url.md))

*   cannot be null

*   defined in: [Chainsight Snapshot Indexer specification](snapshot_indexer-properties-datasource-properties-location-of-the-datasource-properties-args-properties-rpc-url.md "undefined#/properties/datasource/properties/location/properties/args/properties/rpc_url")

### rpc\_url Type

`string` ([rpc url](snapshot_indexer-properties-datasource-properties-location-of-the-datasource-properties-args-properties-rpc-url.md))

### rpc\_url Constraints

**pattern**: the string must match the following regular expression:&#x20;

```regexp
^(http|https)://
```

[try pattern](https://regexr.com/?expression=%5E\(http%7Chttps\)%3A%2F%2F "try regular expression with regexr.com")

### rpc\_url Examples

```json
"https://mainnet.infura.io/v3/YOUR_API_KEY"
```

## id\_type

type of the id. It is required if type is canister.

`id_type`

*   is optional

*   Type: `string` ([id type](snapshot_indexer-properties-datasource-properties-location-of-the-datasource-properties-args-properties-id-type.md))

*   cannot be null

*   defined in: [Chainsight Snapshot Indexer specification](snapshot_indexer-properties-datasource-properties-location-of-the-datasource-properties-args-properties-id-type.md "undefined#/properties/datasource/properties/location/properties/args/properties/id_type")

### id\_type Type

`string` ([id type](snapshot_indexer-properties-datasource-properties-location-of-the-datasource-properties-args-properties-id-type.md))

### id\_type Constraints

**enum**: the value of this property must be equal to one of the following values:

| Value             | Explanation |
| :---------------- | :---------- |
| `"canister_id"`   |             |
| `"canister_name"` |             |

**pattern**: the string must match the following regular expression:&#x20;

```regexp
^(canister_id|canister_name)$
```

[try pattern](https://regexr.com/?expression=%5E\(canister_id%7Ccanister_name\)%24 "try regular expression with regexr.com")

### id\_type Examples

```json
"canister_id"
```

```json
"canister_name"
```
