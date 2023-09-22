# destination Schema

```txt
undefined#/properties/destination
```

destination evm network and contract for the data.

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                      |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :-------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Allowed               | none                | [relayer.json\*](../../out/relayer.json "open original schema") |

## destination Type

`object` ([destination](relayer-properties-destination.md))

# destination Properties

| Property                           | Type     | Required | Nullable       | Defined by                                                                                                                                                          |
| :--------------------------------- | :------- | :------- | :------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| [network\_id](#network_id)         | `number` | Required | cannot be null | [Chainsight Relayer specification](relayer-properties-destination-properties-network-id.md "undefined#/properties/destination/properties/network_id")               |
| [type](#type)                      | `string` | Required | cannot be null | [Chainsight Relayer specification](relayer-properties-destination-properties-oracle-type-of-the-destination.md "undefined#/properties/destination/properties/type") |
| [oracle\_address](#oracle_address) | `number` | Required | cannot be null | [Chainsight Relayer specification](relayer-properties-destination-properties-oracle-address.md "undefined#/properties/destination/properties/oracle_address")       |
| [rpc\_url](#rpc_url)               | `string` | Required | cannot be null | [Chainsight Relayer specification](relayer-properties-destination-properties-rpc-url.md "undefined#/properties/destination/properties/rpc_url")                     |

## network\_id

network id of the destination evm network

`network_id`

*   is required

*   Type: `number` ([network id](relayer-properties-destination-properties-network-id.md))

*   cannot be null

*   defined in: [Chainsight Relayer specification](relayer-properties-destination-properties-network-id.md "undefined#/properties/destination/properties/network_id")

### network\_id Type

`number` ([network id](relayer-properties-destination-properties-network-id.md))

### network\_id Constraints

**pattern**: the string must match the following regular expression:&#x20;

```regexp
^[0-9]+$
```

[try pattern](https://regexr.com/?expression=%5E%5B0-9%5D%2B%24 "try regular expression with regexr.com")

### network\_id Examples

```json
31337
```

```json
1
```

```json
4
```

## type

oracle type of the destination. currently we don't suport user definined oracle types

`type`

*   is required

*   Type: `string` ([oracle type of the destination](relayer-properties-destination-properties-oracle-type-of-the-destination.md))

*   cannot be null

*   defined in: [Chainsight Relayer specification](relayer-properties-destination-properties-oracle-type-of-the-destination.md "undefined#/properties/destination/properties/type")

### type Type

`string` ([oracle type of the destination](relayer-properties-destination-properties-oracle-type-of-the-destination.md))

### type Constraints

**pattern**: the string must match the following regular expression:&#x20;

```regexp
^(uint256|uint128|uint64|string)$
```

[try pattern](https://regexr.com/?expression=%5E\(uint256%7Cuint128%7Cuint64%7Cstring\)%24 "try regular expression with regexr.com")

### type Examples

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

## oracle\_address

address of the destination oracle contract

`oracle_address`

*   is required

*   Type: `number` ([oracle address](relayer-properties-destination-properties-oracle-address.md))

*   cannot be null

*   defined in: [Chainsight Relayer specification](relayer-properties-destination-properties-oracle-address.md "undefined#/properties/destination/properties/oracle_address")

### oracle\_address Type

`number` ([oracle address](relayer-properties-destination-properties-oracle-address.md))

### oracle\_address Constraints

**pattern**: the string must match the following regular expression:&#x20;

```regexp
^(0x)?[0-9a-fA-F]{40}$
```

[try pattern](https://regexr.com/?expression=%5E\(0x\)%3F%5B0-9a-fA-F%5D%7B40%7D%24 "try regular expression with regexr.com")

### oracle\_address Examples

```json
"0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512"
```

## rpc\_url

rpc url of the destination evm network. only supports https

`rpc_url`

*   is required

*   Type: `string` ([rpc url](relayer-properties-destination-properties-rpc-url.md))

*   cannot be null

*   defined in: [Chainsight Relayer specification](relayer-properties-destination-properties-rpc-url.md "undefined#/properties/destination/properties/rpc_url")

### rpc\_url Type

`string` ([rpc url](relayer-properties-destination-properties-rpc-url.md))

### rpc\_url Constraints

**pattern**: the string must match the following regular expression:&#x20;

```regexp
^https://.*$
```

[try pattern](https://regexr.com/?expression=%5Ehttps%3A%2F%2F.*%24 "try regular expression with regexr.com")

### rpc\_url Examples

```json
"https://rinkeby.infura.io/v3/1a2b3c4d5e"
```
