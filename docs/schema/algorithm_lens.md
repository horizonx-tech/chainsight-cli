# Chainsight Algorithm Lens specification Schema

```txt
undefined
```

Chainsight Algorithm Lens specification

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                   |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :--------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Allowed               | none                | [algorithm\_lens.json](../../out/algorithm_lens.json "open original schema") |

## Chainsight Algorithm Lens specification Type

`object` ([Chainsight Algorithm Lens specification](algorithm_lens.md))

# Chainsight Algorithm Lens specification Properties

| Property                  | Type     | Required | Nullable       | Defined by                                                                                                                                    |
| :------------------------ | :------- | :------- | :------------- | :-------------------------------------------------------------------------------------------------------------------------------------------- |
| [version](#version)       | `string` | Required | cannot be null | [Chainsight Algorithm Lens specification](algorithm_lens-properties-specification-version-of-the-canister.md "undefined#/properties/version") |
| [metadata](#metadata)     | `object` | Required | cannot be null | [Chainsight Algorithm Lens specification](algorithm_lens-properties-metadata.md "undefined#/properties/metadata")                             |
| [datasource](#datasource) | `object` | Required | cannot be null | [Chainsight Algorithm Lens specification](algorithm_lens-properties-datasource.md "undefined#/properties/datasource")                         |
| [output](#output)         | `object` | Required | cannot be null | [Chainsight Algorithm Lens specification](algorithm_lens-properties-output.md "undefined#/properties/output")                                 |

## version



`version`

*   is required

*   Type: `string` ([specification version of the canister](algorithm_lens-properties-specification-version-of-the-canister.md))

*   cannot be null

*   defined in: [Chainsight Algorithm Lens specification](algorithm_lens-properties-specification-version-of-the-canister.md "undefined#/properties/version")

### version Type

`string` ([specification version of the canister](algorithm_lens-properties-specification-version-of-the-canister.md))

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

*   Type: `object` ([metadata](algorithm_lens-properties-metadata.md))

*   cannot be null

*   defined in: [Chainsight Algorithm Lens specification](algorithm_lens-properties-metadata.md "undefined#/properties/metadata")

### metadata Type

`object` ([metadata](algorithm_lens-properties-metadata.md))

## datasource



`datasource`

*   is required

*   Type: `object` ([Details](algorithm_lens-properties-datasource.md))

*   cannot be null

*   defined in: [Chainsight Algorithm Lens specification](algorithm_lens-properties-datasource.md "undefined#/properties/datasource")

### datasource Type

`object` ([Details](algorithm_lens-properties-datasource.md))

## output

output of the algorithm lens

`output`

*   is required

*   Type: `object` ([output](algorithm_lens-properties-output.md))

*   cannot be null

*   defined in: [Chainsight Algorithm Lens specification](algorithm_lens-properties-output.md "undefined#/properties/output")

### output Type

`object` ([output](algorithm_lens-properties-output.md))
