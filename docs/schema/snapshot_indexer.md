# Chainsight Snapshot Indexer specification Schema

```txt
undefined
```

Chainsight Snapshot Indexer specification

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                       |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :------------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Allowed               | none                | [snapshot\_indexer.json](../../out/snapshot_indexer.json "open original schema") |

## Chainsight Snapshot Indexer specification Type

`object` ([Chainsight Snapshot Indexer specification](snapshot_indexer.md))

# Chainsight Snapshot Indexer specification Properties

| Property                  | Type     | Required | Nullable       | Defined by                                                                                                                                        |
| :------------------------ | :------- | :------- | :------------- | :------------------------------------------------------------------------------------------------------------------------------------------------ |
| [version](#version)       | `string` | Required | cannot be null | [Chainsight Snapshot Indexer specification](snapshot_indexer-properties-specification-version-of-the-canister.md "undefined#/properties/version") |
| [metadata](#metadata)     | `object` | Required | cannot be null | [Chainsight Snapshot Indexer specification](snapshot_indexer-properties-metadata.md "undefined#/properties/metadata")                             |
| [datasource](#datasource) | `object` | Required | cannot be null | [Chainsight Snapshot Indexer specification](snapshot_indexer-properties-datasource.md "undefined#/properties/datasource")                         |
| [storage](#storage)       | `object` | Required | cannot be null | [Chainsight Snapshot Indexer specification](snapshot_indexer-properties-storage.md "undefined#/properties/storage")                               |
| [interval](#interval)     | `number` | Required | cannot be null | [Chainsight Snapshot Indexer specification](snapshot_indexer-properties-interval.md "undefined#/properties/interval")                             |

## version



`version`

*   is required

*   Type: `string` ([specification version of the canister](snapshot_indexer-properties-specification-version-of-the-canister.md))

*   cannot be null

*   defined in: [Chainsight Snapshot Indexer specification](snapshot_indexer-properties-specification-version-of-the-canister.md "undefined#/properties/version")

### version Type

`string` ([specification version of the canister](snapshot_indexer-properties-specification-version-of-the-canister.md))

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

*   Type: `object` ([metadata](snapshot_indexer-properties-metadata.md))

*   cannot be null

*   defined in: [Chainsight Snapshot Indexer specification](snapshot_indexer-properties-metadata.md "undefined#/properties/metadata")

### metadata Type

`object` ([metadata](snapshot_indexer-properties-metadata.md))

## datasource



`datasource`

*   is required

*   Type: `object` ([Details](snapshot_indexer-properties-datasource.md))

*   cannot be null

*   defined in: [Chainsight Snapshot Indexer specification](snapshot_indexer-properties-datasource.md "undefined#/properties/datasource")

### datasource Type

`object` ([Details](snapshot_indexer-properties-datasource.md))

## storage

storage properties for the canister

`storage`

*   is required

*   Type: `object` ([storage](snapshot_indexer-properties-storage.md))

*   cannot be null

*   defined in: [Chainsight Snapshot Indexer specification](snapshot_indexer-properties-storage.md "undefined#/properties/storage")

### storage Type

`object` ([storage](snapshot_indexer-properties-storage.md))

## interval

interval of the canister invocation in seconds

`interval`

*   is required

*   Type: `number` ([interval](snapshot_indexer-properties-interval.md))

*   cannot be null

*   defined in: [Chainsight Snapshot Indexer specification](snapshot_indexer-properties-interval.md "undefined#/properties/interval")

### interval Type

`number` ([interval](snapshot_indexer-properties-interval.md))

### interval Examples

```json
60
```
