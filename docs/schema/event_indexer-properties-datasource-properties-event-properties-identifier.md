# identifier Schema

```txt
undefined#/properties/datasource/properties/event/properties/identifier
```

event name. You can find it in the abi

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                   |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :--------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [event\_indexer.json\*](../../out/event_indexer.json "open original schema") |

## identifier Type

`string` ([identifier](event_indexer-properties-datasource-properties-event-properties-identifier.md))

## identifier Constraints

**pattern**: the string must match the following regular expression:&#x20;

```regexp
^[a-zA-Z0-9_-]+$
```

[try pattern](https://regexr.com/?expression=%5E%5Ba-zA-Z0-9_-%5D%2B%24 "try regular expression with regexr.com")

## identifier Examples

```json
"Transfer"
```
