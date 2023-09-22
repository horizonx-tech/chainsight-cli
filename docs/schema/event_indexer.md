# Chainsight Event Indexer specification Schema

```txt
https://raw.githubusercontent.com/horizonx-tech/chainsight-cli/main/resources/schema/event_indexer.json
```

Chainsight Event Indexer specification

> Chainsight Event Indexer specification

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                 |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Allowed               | none                | [event\_indexer.json](../../out/event_indexer.json "open original schema") |

## Chainsight Event Indexer specification Type

`object` ([Chainsight Event Indexer specification](event_indexer.md))

# Chainsight Event Indexer specification Properties

| Property                  | Type     | Required | Nullable       | Defined by                                                                                                                                             |
| :------------------------ | :------- | :------- | :------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------- |
| [version](#version)       | `string` | Required | cannot be null | [Chainsight Event Indexer specification](event_indexer-properties-specification-version-of-the-canister.md "#/properties/version#/properties/version") |
| [metadata](#metadata)     | `object` | Required | cannot be null | [Chainsight Event Indexer specification](event_indexer-properties-metadata.md "#/properties/metadata#/properties/metadata")                            |
| [datasource](#datasource) | `object` | Required | cannot be null | [Chainsight Event Indexer specification](event_indexer-properties-datasource.md "#/properties/datasource#/properties/datasource")                      |
| [interval](#interval)     | `number` | Required | cannot be null | [Chainsight Event Indexer specification](event_indexer-properties-interval.md "#/properties/interval#/properties/interval")                            |

## version



`version`

*   is required

*   Type: `string` ([specification version of the canister](event_indexer-properties-specification-version-of-the-canister.md))

*   cannot be null

*   defined in: [Chainsight Event Indexer specification](event_indexer-properties-specification-version-of-the-canister.md "#/properties/version#/properties/version")

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

*   defined in: [Chainsight Event Indexer specification](event_indexer-properties-metadata.md "#/properties/metadata#/properties/metadata")

### metadata Type

`object` ([metadata](event_indexer-properties-metadata.md))

## datasource

datasource for the canister

`datasource`

*   is required

*   Type: `object` ([datasource](event_indexer-properties-datasource.md))

*   cannot be null

*   defined in: [Chainsight Event Indexer specification](event_indexer-properties-datasource.md "#/properties/datasource#/properties/datasource")

### datasource Type

`object` ([datasource](event_indexer-properties-datasource.md))

## interval

interval of the canister invocation in seconds

`interval`

*   is required

*   Type: `number` ([interval](event_indexer-properties-interval.md))

*   cannot be null

*   defined in: [Chainsight Event Indexer specification](event_indexer-properties-interval.md "#/properties/interval#/properties/interval")

### interval Type

`number` ([interval](event_indexer-properties-interval.md))

### interval Examples

```json
60
```
