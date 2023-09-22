# Chainsight Event Indexer specification Schema

```txt
undefined
```

Chainsight Event Indexer specification

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                 |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Allowed               | none                | [event\_indexer.json](../../out/event_indexer.json "open original schema") |

## Chainsight Event Indexer specification Type

`object` ([Chainsight Event Indexer specification](event_indexer.md))

# Chainsight Event Indexer specification Properties

| Property                  | Type     | Required | Nullable       | Defined by                                                                                                                                  |
| :------------------------ | :------- | :------- | :------------- | :------------------------------------------------------------------------------------------------------------------------------------------ |
| [version](#version)       | `string` | Required | cannot be null | [Chainsight Event Indexer specification](event_indexer-properties-specification-version-of-the-canister.md "undefined#/properties/version") |
| [metadata](#metadata)     | `object` | Required | cannot be null | [Chainsight Event Indexer specification](event_indexer-properties-metadata.md "undefined#/properties/metadata")                             |
| [datasource](#datasource) | `object` | Required | cannot be null | [Chainsight Event Indexer specification](event_indexer-properties-datasource.md "undefined#/properties/datasource")                         |
| [interval](#interval)     | `number` | Required | cannot be null | [Chainsight Event Indexer specification](event_indexer-properties-interval.md "undefined#/properties/interval")                             |

## version



`version`

*   is required

*   Type: `string` ([specification version of the canister](event_indexer-properties-specification-version-of-the-canister.md))

*   cannot be null

*   defined in: [Chainsight Event Indexer specification](event_indexer-properties-specification-version-of-the-canister.md "undefined#/properties/version")

### version Type

`string` ([specification version of the canister](event_indexer-properties-specification-version-of-the-canister.md))

### version Constraints

**pattern**: the string must match the following regular expression:&#x20;

```regexp
^(v1)$
```

[try pattern](https://regexr.com/?expression=%5E\(v1\)%24 "try regular expression with regexr.com")

### version Examples

```json
"v1"
```

## metadata



> metadata for the canister

`metadata`

*   is required

*   Type: `object` ([metadata](event_indexer-properties-metadata.md))

*   cannot be null

*   defined in: [Chainsight Event Indexer specification](event_indexer-properties-metadata.md "undefined#/properties/metadata")

### metadata Type

`object` ([metadata](event_indexer-properties-metadata.md))

## datasource



`datasource`

*   is required

*   Type: `object` ([Details](event_indexer-properties-datasource.md))

*   cannot be null

*   defined in: [Chainsight Event Indexer specification](event_indexer-properties-datasource.md "undefined#/properties/datasource")

### datasource Type

`object` ([Details](event_indexer-properties-datasource.md))

## interval

interval of the canister invocation in seconds

`interval`

*   is required

*   Type: `number` ([interval](event_indexer-properties-interval.md))

*   cannot be null

*   defined in: [Chainsight Event Indexer specification](event_indexer-properties-interval.md "undefined#/properties/interval")

### interval Type

`number` ([interval](event_indexer-properties-interval.md))

### interval Examples

```json
60
```
