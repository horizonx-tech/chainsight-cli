# Untitled object in Chainsight Event Indexer specification Schema

```txt
undefined#/properties/datasource
```



| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                   |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :--------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Allowed               | none                | [event\_indexer.json\*](../../out/event_indexer.json "open original schema") |

## datasource Type

`object` ([Details](event_indexer-properties-datasource.md))

# datasource Properties

| Property                         | Type          | Required | Nullable       | Defined by                                                                                                                                                            |
| :------------------------------- | :------------ | :------- | :------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [id](#id)                        | Not specified | Required | cannot be null | [Chainsight Event Indexer specification](event_indexer-properties-datasource-properties-id.md "undefined#/properties/datasource/properties/id")                       |
| [event](#event)                  | `object`      | Required | cannot be null | [Chainsight Event Indexer specification](event_indexer-properties-datasource-properties-event.md "undefined#/properties/datasource/properties/event")                 |
| [from](#from)                    | `number`      | Required | cannot be null | [Chainsight Event Indexer specification](event_indexer-properties-datasource-properties-from.md "undefined#/properties/datasource/properties/from")                   |
| [network](#network)              | `object`      | Required | cannot be null | [Chainsight Event Indexer specification](event_indexer-properties-datasource-properties-network.md "undefined#/properties/datasource/properties/network")             |
| [contract\_type](#contract_type) | `string`      | Optional | cannot be null | [Chainsight Event Indexer specification](event_indexer-properties-datasource-properties-contract_type.md "undefined#/properties/datasource/properties/contract_type") |

## id

contract address

`id`

*   is required

*   Type: unknown ([id](event_indexer-properties-datasource-properties-id.md))

*   cannot be null

*   defined in: [Chainsight Event Indexer specification](event_indexer-properties-datasource-properties-id.md "undefined#/properties/datasource/properties/id")

### id Type

unknown ([id](event_indexer-properties-datasource-properties-id.md))

### id Constraints

**pattern**: the string must match the following regular expression:&#x20;

```regexp
^(0x)?[0-9a-fA-F]{40}$
```

[try pattern](https://regexr.com/?expression=%5E\(0x\)%3F%5B0-9a-fA-F%5D%7B40%7D%24 "try regular expression with regexr.com")

### id Examples

```json
"0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512"
```

## event

event identifier to save

`event`

*   is required

*   Type: `object` ([event](event_indexer-properties-datasource-properties-event.md))

*   cannot be null

*   defined in: [Chainsight Event Indexer specification](event_indexer-properties-datasource-properties-event.md "undefined#/properties/datasource/properties/event")

### event Type

`object` ([event](event_indexer-properties-datasource-properties-event.md))

## from

block number to start the query from

`from`

*   is required

*   Type: `number` ([from](event_indexer-properties-datasource-properties-from.md))

*   cannot be null

*   defined in: [Chainsight Event Indexer specification](event_indexer-properties-datasource-properties-from.md "undefined#/properties/datasource/properties/from")

### from Type

`number` ([from](event_indexer-properties-datasource-properties-from.md))

### from Examples

```json
0
```

## network

chain id and rpc url

`network`

*   is required

*   Type: `object` ([network](event_indexer-properties-datasource-properties-network.md))

*   cannot be null

*   defined in: [Chainsight Event Indexer specification](event_indexer-properties-datasource-properties-network.md "undefined#/properties/datasource/properties/network")

### network Type

`object` ([network](event_indexer-properties-datasource-properties-network.md))

## contract\_type

type of the contract. It is not required, but it is useful to filter canisters in the UI

`contract_type`

*   is optional

*   Type: `string` ([contract\_type](event_indexer-properties-datasource-properties-contract_type.md))

*   cannot be null

*   defined in: [Chainsight Event Indexer specification](event_indexer-properties-datasource-properties-contract_type.md "undefined#/properties/datasource/properties/contract_type")

### contract\_type Type

`string` ([contract\_type](event_indexer-properties-datasource-properties-contract_type.md))

### contract\_type Examples

```json
"ERC20"
```

```json
"DEX"
```
