# metadata Schema

```txt
undefined#/properties/metadata
```



> metadata for the canister

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Allowed               | none                | [algorithm\_lens.json\*](../../out/algorithm_lens.json "open original schema") |

## metadata Type

`object` ([metadata](algorithm_lens-properties-metadata.md))

# metadata Properties

| Property                    | Type     | Required | Nullable       | Defined by                                                                                                                                                                      |
| :-------------------------- | :------- | :------- | :------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| [label](#label)             | `string` | Required | cannot be null | [Chainsight Algorithm Lens specification](algorithm_lens-properties-metadata-properties-label-for-the-canister.md "undefined#/properties/metadata/properties/label")            |
| [type](#type)               | `string` | Required | cannot be null | [Chainsight Algorithm Lens specification](algorithm_lens-properties-metadata-properties-type-of-the-canister.md "undefined#/properties/metadata/properties/type")               |
| [description](#description) | `string` | Optional | cannot be null | [Chainsight Algorithm Lens specification](algorithm_lens-properties-metadata-properties-description-of-the-canister.md "undefined#/properties/metadata/properties/description") |
| [tags](#tags)               | `array`  | Optional | cannot be null | [Chainsight Algorithm Lens specification](algorithm_lens-properties-metadata-properties-tags-for-the-canister.md "undefined#/properties/metadata/properties/tags")              |

## label



`label`

*   is required

*   Type: `string` ([label for the canister](algorithm_lens-properties-metadata-properties-label-for-the-canister.md))

*   cannot be null

*   defined in: [Chainsight Algorithm Lens specification](algorithm_lens-properties-metadata-properties-label-for-the-canister.md "undefined#/properties/metadata/properties/label")

### label Type

`string` ([label for the canister](algorithm_lens-properties-metadata-properties-label-for-the-canister.md))

### label Constraints

**pattern**: the string must match the following regular expression:&#x20;

```regexp
^[a-z0-9_]+$
```

[try pattern](https://regexr.com/?expression=%5E%5Ba-z0-9_%5D%2B%24 "try regular expression with regexr.com")

### label Examples

```json
"relayer_ethusd"
```

## type



`type`

*   is required

*   Type: `string` ([type of the canister](algorithm_lens-properties-metadata-properties-type-of-the-canister.md))

*   cannot be null

*   defined in: [Chainsight Algorithm Lens specification](algorithm_lens-properties-metadata-properties-type-of-the-canister.md "undefined#/properties/metadata/properties/type")

### type Type

`string` ([type of the canister](algorithm_lens-properties-metadata-properties-type-of-the-canister.md))

### type Constraints

**pattern**: the string must match the following regular expression:&#x20;

```regexp
^(event_indexer|algorithm_indexer|snapshot_indexer|snapshot_json_rpc|relayer|algorithm_lens)$
```

[try pattern](https://regexr.com/?expression=%5E\(event_indexer%7Calgorithm_indexer%7Csnapshot_indexer%7Csnapshot_json_rpc%7Crelayer%7Calgorithm_lens\)%24 "try regular expression with regexr.com")

### type Examples

```json
"event_indexer"
```

```json
"algorithm_indexer"
```

```json
"snapshot_indexer"
```

```json
"snapshot_json_rpc"
```

```json
"relayer"
```

```json
"algorithm_lens"
```

## description

Can be used to filter canisters in the UI

`description`

*   is optional

*   Type: `string` ([description of the canister](algorithm_lens-properties-metadata-properties-description-of-the-canister.md))

*   cannot be null

*   defined in: [Chainsight Algorithm Lens specification](algorithm_lens-properties-metadata-properties-description-of-the-canister.md "undefined#/properties/metadata/properties/description")

### description Type

`string` ([description of the canister](algorithm_lens-properties-metadata-properties-description-of-the-canister.md))

### description Examples

```json
"Relayer for ETHUSD"
```

## tags

Can be used to filter canisters in the UI

`tags`

*   is optional

*   Type: `string[]`

*   cannot be null

*   defined in: [Chainsight Algorithm Lens specification](algorithm_lens-properties-metadata-properties-tags-for-the-canister.md "undefined#/properties/metadata/properties/tags")

### tags Type

`string[]`
