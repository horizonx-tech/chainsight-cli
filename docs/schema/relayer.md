# Chainsight Relayer specification Schema

```txt
undefined
```

Chainsight Relayer specification

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                    |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :------------------------------------------------------------ |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Allowed               | none                | [relayer.json](../../out/relayer.json "open original schema") |

## Chainsight Relayer specification Type

`object` ([Chainsight Relayer specification](relayer.md))

# Chainsight Relayer specification Properties

| Property                       | Type     | Required | Nullable       | Defined by                                                                                                                      |
| :----------------------------- | :------- | :------- | :------------- | :------------------------------------------------------------------------------------------------------------------------------ |
| [version](#version)            | `string` | Required | cannot be null | [Chainsight Relayer specification](relayer-properties-specification-version-of-the-canister.md "undefined#/properties/version") |
| [metadata](#metadata)          | `object` | Required | cannot be null | [Chainsight Relayer specification](relayer-properties-metadata.md "undefined#/properties/metadata")                             |
| [datasource](#datasource)      | `object` | Required | cannot be null | [Chainsight Relayer specification](relayer-properties-datasource.md "undefined#/properties/datasource")                         |
| [lens\_targets](#lens_targets) | `object` | Optional | cannot be null | [Chainsight Relayer specification](relayer-properties-lens_targets.md "undefined#/properties/lens_targets")                     |
| [destination](#destination)    | `object` | Required | cannot be null | [Chainsight Relayer specification](relayer-properties-destination.md "undefined#/properties/destination")                       |
| [interval](#interval)          | `number` | Required | cannot be null | [Chainsight Relayer specification](relayer-properties-interval.md "undefined#/properties/interval")                             |

## version



`version`

*   is required

*   Type: `string` ([specification version of the canister](relayer-properties-specification-version-of-the-canister.md))

*   cannot be null

*   defined in: [Chainsight Relayer specification](relayer-properties-specification-version-of-the-canister.md "undefined#/properties/version")

### version Type

`string` ([specification version of the canister](relayer-properties-specification-version-of-the-canister.md))

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

*   Type: `object` ([metadata](relayer-properties-metadata.md))

*   cannot be null

*   defined in: [Chainsight Relayer specification](relayer-properties-metadata.md "undefined#/properties/metadata")

### metadata Type

`object` ([metadata](relayer-properties-metadata.md))

## datasource



`datasource`

*   is required

*   Type: `object` ([Details](relayer-properties-datasource.md))

*   cannot be null

*   defined in: [Chainsight Relayer specification](relayer-properties-datasource.md "undefined#/properties/datasource")

### datasource Type

`object` ([Details](relayer-properties-datasource.md))

## lens\_targets

targets for the lens. Only used when the datasource canister is a algorithm\_lens

`lens_targets`

*   is optional

*   Type: `object` ([Details](relayer-properties-lens_targets.md))

*   cannot be null

*   defined in: [Chainsight Relayer specification](relayer-properties-lens_targets.md "undefined#/properties/lens_targets")

### lens\_targets Type

`object` ([Details](relayer-properties-lens_targets.md))

## destination

destination evm network and contract for the data.

`destination`

*   is required

*   Type: `object` ([destination](relayer-properties-destination.md))

*   cannot be null

*   defined in: [Chainsight Relayer specification](relayer-properties-destination.md "undefined#/properties/destination")

### destination Type

`object` ([destination](relayer-properties-destination.md))

## interval

interval of the canister invocation in seconds

`interval`

*   is required

*   Type: `number` ([interval](relayer-properties-interval.md))

*   cannot be null

*   defined in: [Chainsight Relayer specification](relayer-properties-interval.md "undefined#/properties/interval")

### interval Type

`number` ([interval](relayer-properties-interval.md))

### interval Examples

```json
60
```
