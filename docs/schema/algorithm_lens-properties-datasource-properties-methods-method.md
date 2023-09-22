# method Schema

```txt
undefined#/properties/datasource/properties/methods/items
```

method to call on the callee canister

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Allowed               | none                | [algorithm\_lens.json\*](../../out/algorithm_lens.json "open original schema") |

## items Type

`object` ([method](algorithm_lens-properties-datasource-properties-methods-method.md))

# items Properties

| Property                                | Type     | Required | Nullable       | Defined by                                                                                                                                                                                                                       |
| :-------------------------------------- | :------- | :------- | :------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [label](#label)                         | `string` | Required | cannot be null | [Chainsight Algorithm Lens specification](algorithm_lens-properties-datasource-properties-methods-method-properties-label.md "undefined#/properties/datasource/properties/methods/items/properties/label")                       |
| [identifier](#identifier)               | `string` | Required | cannot be null | [Chainsight Algorithm Lens specification](algorithm_lens-properties-datasource-properties-methods-method-properties-identifier.md "undefined#/properties/datasource/properties/methods/items/properties/identifier")             |
| [candid\_file\_path](#candid_file_path) | `string` | Required | cannot be null | [Chainsight Algorithm Lens specification](algorithm_lens-properties-datasource-properties-methods-method-properties-candid_file_path.md "undefined#/properties/datasource/properties/methods/items/properties/candid_file_path") |

## label

label of this method

`label`

*   is required

*   Type: `string` ([label](algorithm_lens-properties-datasource-properties-methods-method-properties-label.md))

*   cannot be null

*   defined in: [Chainsight Algorithm Lens specification](algorithm_lens-properties-datasource-properties-methods-method-properties-label.md "undefined#/properties/datasource/properties/methods/items/properties/label")

### label Type

`string` ([label](algorithm_lens-properties-datasource-properties-methods-method-properties-label.md))

### label Constraints

**pattern**: the string must match the following regular expression:&#x20;

```regexp
^[a-z0-9_]+$
```

[try pattern](https://regexr.com/?expression=%5E%5Ba-z0-9_%5D%2B%24 "try regular expression with regexr.com")

### label Examples

```json
"get_ethusd_price_from_coingecko"
```

```json
"get_ethusd_price_from_chainlink"
```

## identifier

method identifier of the canister to call. You can find it in the candid file

`identifier`

*   is required

*   Type: `string` ([identifier](algorithm_lens-properties-datasource-properties-methods-method-properties-identifier.md))

*   cannot be null

*   defined in: [Chainsight Algorithm Lens specification](algorithm_lens-properties-datasource-properties-methods-method-properties-identifier.md "undefined#/properties/datasource/properties/methods/items/properties/identifier")

### identifier Type

`string` ([identifier](algorithm_lens-properties-datasource-properties-methods-method-properties-identifier.md))

### identifier Examples

```json
"get_last_snapshot_value : () -> (SnapshotValue)"
```

## candid\_file\_path

path to the candid file of the canister to call

`candid_file_path`

*   is required

*   Type: `string` ([candid\_file\_path](algorithm_lens-properties-datasource-properties-methods-method-properties-candid_file_path.md))

*   cannot be null

*   defined in: [Chainsight Algorithm Lens specification](algorithm_lens-properties-datasource-properties-methods-method-properties-candid_file_path.md "undefined#/properties/datasource/properties/methods/items/properties/candid_file_path")

### candid\_file\_path Type

`string` ([candid\_file\_path](algorithm_lens-properties-datasource-properties-methods-method-properties-candid_file_path.md))

### candid\_file\_path Examples

```json
"artifacts/chainlink/src/chainlink.did"
```
