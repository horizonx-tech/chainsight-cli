# fields Schema

```txt
undefined#/properties/output/properties/fields
```

set of field names and their Rust types. Required if 'output.type' is 'struct'

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [algorithm\_lens.json\*](../../out/algorithm_lens.json "open original schema") |

## fields Type

`object` ([fields](algorithm_lens-properties-output-properties-fields.md))

# fields Properties

| Property              | Type     | Required | Nullable       | Defined by                                                                                                                                                                                  |
| :-------------------- | :------- | :------- | :------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Additional Properties | `string` | Optional | cannot be null | [Chainsight Algorithm Lens specification](algorithm_lens-properties-output-properties-fields-additionalproperties.md "undefined#/properties/output/properties/fields/additionalProperties") |

## Additional Properties

Additional properties are allowed, as long as they follow this schema:



*   is optional

*   Type: `string`

*   cannot be null

*   defined in: [Chainsight Algorithm Lens specification](algorithm_lens-properties-output-properties-fields-additionalproperties.md "undefined#/properties/output/properties/fields/additionalProperties")

### additionalProperties Type

`string`

### additionalProperties Examples

```json
"u128"
```

```json
"String"
```

```json
"bool"
```
