# id Schema

```txt
undefined#/properties/datasource/properties/id
```

contract address

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                   |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :--------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [event\_indexer.json\*](../../out/event_indexer.json "open original schema") |

## id Type

unknown ([id](event_indexer-properties-datasource-properties-id.md))

## id Constraints

**pattern**: the string must match the following regular expression:&#x20;

```regexp
^(0x)?[0-9a-fA-F]{40}$
```

[try pattern](https://regexr.com/?expression=%5E\(0x\)%3F%5B0-9a-fA-F%5D%7B40%7D%24 "try regular expression with regexr.com")

## id Examples

```json
"0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512"
```
