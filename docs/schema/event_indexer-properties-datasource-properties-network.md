# network Schema

```txt
undefined#/properties/datasource/properties/network
```

chain id and rpc url

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                   |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :--------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Allowed               | none                | [event\_indexer.json\*](../../out/event_indexer.json "open original schema") |

## network Type

`object` ([network](event_indexer-properties-datasource-properties-network.md))

# network Properties

| Property               | Type     | Required | Nullable       | Defined by                                                                                                                                                                                        |
| :--------------------- | :------- | :------- | :------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| [chain\_id](#chain_id) | `number` | Required | cannot be null | [Chainsight Event Indexer specification](event_indexer-properties-datasource-properties-network-properties-chain_id.md "undefined#/properties/datasource/properties/network/properties/chain_id") |
| [rpc\_url](#rpc_url)   | `string` | Required | cannot be null | [Chainsight Event Indexer specification](event_indexer-properties-datasource-properties-network-properties-rpc_url.md "undefined#/properties/datasource/properties/network/properties/rpc_url")   |

## chain\_id

chain id

`chain_id`

*   is required

*   Type: `number` ([chain\_id](event_indexer-properties-datasource-properties-network-properties-chain_id.md))

*   cannot be null

*   defined in: [Chainsight Event Indexer specification](event_indexer-properties-datasource-properties-network-properties-chain_id.md "undefined#/properties/datasource/properties/network/properties/chain_id")

### chain\_id Type

`number` ([chain\_id](event_indexer-properties-datasource-properties-network-properties-chain_id.md))

### chain\_id Examples

```json
1
```

## rpc\_url

rpc url

`rpc_url`

*   is required

*   Type: `string` ([rpc\_url](event_indexer-properties-datasource-properties-network-properties-rpc_url.md))

*   cannot be null

*   defined in: [Chainsight Event Indexer specification](event_indexer-properties-datasource-properties-network-properties-rpc_url.md "undefined#/properties/datasource/properties/network/properties/rpc_url")

### rpc\_url Type

`string` ([rpc\_url](event_indexer-properties-datasource-properties-network-properties-rpc_url.md))

### rpc\_url Constraints

**pattern**: the string must match the following regular expression:&#x20;

```regexp
^https?://
```

[try pattern](https://regexr.com/?expression=%5Ehttps%3F%3A%2F%2F "try regular expression with regexr.com")

### rpc\_url Examples

```json
"https://mainnet.infura.io/v3/"
```
