# interface Schema

```txt
undefined#/properties/datasource/properties/event/properties/interface
```

abi json file. It must be in ./interfaces folder

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                   |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :--------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [event\_indexer.json\*](../../out/event_indexer.json "open original schema") |

## interface Type

`string` ([interface](event_indexer-properties-datasource-properties-event-properties-interface.md))

## interface Constraints

**pattern**: the string must match the following regular expression:&#x20;

```regexp
^[a-zA-Z0-9_-]+\.json$
```

[try pattern](https://regexr.com/?expression=%5E%5Ba-zA-Z0-9_-%5D%2B%5C.json%24 "try regular expression with regexr.com")

## interface Examples

```json
"IERC20.json"
```
