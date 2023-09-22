# event Schema

```txt
undefined#/properties/datasource/properties/event
```

event identifier to save

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                   |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :--------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Allowed               | none                | [event\_indexer.json\*](../../out/event_indexer.json "open original schema") |

## event Type

`object` ([event](event_indexer-properties-datasource-properties-event.md))

# event Properties

| Property                  | Type     | Required | Nullable       | Defined by                                                                                                                                                                                        |
| :------------------------ | :------- | :------- | :------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| [identifier](#identifier) | `string` | Required | cannot be null | [Chainsight Event Indexer specification](event_indexer-properties-datasource-properties-event-properties-identifier.md "undefined#/properties/datasource/properties/event/properties/identifier") |
| [interface](#interface)   | `string` | Required | cannot be null | [Chainsight Event Indexer specification](event_indexer-properties-datasource-properties-event-properties-interface.md "undefined#/properties/datasource/properties/event/properties/interface")   |

## identifier

event name. You can find it in the abi

`identifier`

*   is required

*   Type: `string` ([identifier](event_indexer-properties-datasource-properties-event-properties-identifier.md))

*   cannot be null

*   defined in: [Chainsight Event Indexer specification](event_indexer-properties-datasource-properties-event-properties-identifier.md "undefined#/properties/datasource/properties/event/properties/identifier")

### identifier Type

`string` ([identifier](event_indexer-properties-datasource-properties-event-properties-identifier.md))

### identifier Constraints

**pattern**: the string must match the following regular expression:&#x20;

```regexp
^[a-zA-Z0-9_-]+$
```

[try pattern](https://regexr.com/?expression=%5E%5Ba-zA-Z0-9_-%5D%2B%24 "try regular expression with regexr.com")

### identifier Examples

```json
"Transfer"
```

## interface

abi json file. It must be in ./interfaces folder

`interface`

*   is required

*   Type: `string` ([interface](event_indexer-properties-datasource-properties-event-properties-interface.md))

*   cannot be null

*   defined in: [Chainsight Event Indexer specification](event_indexer-properties-datasource-properties-event-properties-interface.md "undefined#/properties/datasource/properties/event/properties/interface")

### interface Type

`string` ([interface](event_indexer-properties-datasource-properties-event-properties-interface.md))

### interface Constraints

**pattern**: the string must match the following regular expression:&#x20;

```regexp
^[a-zA-Z0-9_-]+\.json$
```

[try pattern](https://regexr.com/?expression=%5E%5Ba-zA-Z0-9_-%5D%2B%5C.json%24 "try regular expression with regexr.com")

### interface Examples

```json
"IERC20.json"
```
