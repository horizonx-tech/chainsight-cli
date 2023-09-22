# datasource method Schema

```txt
undefined#/properties/datasource/properties/method
```

method to call on the canister

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                      |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :-------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Allowed               | none                | [relayer.json\*](../../out/relayer.json "open original schema") |

## method Type

`object` ([datasource method](relayer-properties-datasource-properties-datasource-method.md))

# method Properties

| Property                  | Type     | Required | Nullable       | Defined by                                                                                                                                                                                                |
| :------------------------ | :------- | :------- | :------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [identifier](#identifier) | `string` | Required | cannot be null | [Chainsight Relayer specification](relayer-properties-datasource-properties-datasource-method-properties-method-identifier.md "undefined#/properties/datasource/properties/method/properties/identifier") |
| [args](#args)             | `array`  | Required | cannot be null | [Chainsight Relayer specification](relayer-properties-datasource-properties-datasource-method-properties-method-arguments.md "undefined#/properties/datasource/properties/method/properties/args")        |

## identifier

method identifier. You can get it from candid definition

`identifier`

*   is required

*   Type: `string` ([method identifier](relayer-properties-datasource-properties-datasource-method-properties-method-identifier.md))

*   cannot be null

*   defined in: [Chainsight Relayer specification](relayer-properties-datasource-properties-datasource-method-properties-method-identifier.md "undefined#/properties/datasource/properties/method/properties/identifier")

### identifier Type

`string` ([method identifier](relayer-properties-datasource-properties-datasource-method-properties-method-identifier.md))

### identifier Examples

```json
"get_last_snapshot : () -> (record { value : text; timestamp : nat64 })"
```

## args

array of method arguments

`args`

*   is required

*   Type: `array` ([method arguments](relayer-properties-datasource-properties-datasource-method-properties-method-arguments.md))

*   cannot be null

*   defined in: [Chainsight Relayer specification](relayer-properties-datasource-properties-datasource-method-properties-method-arguments.md "undefined#/properties/datasource/properties/method/properties/args")

### args Type

`array` ([method arguments](relayer-properties-datasource-properties-datasource-method-properties-method-arguments.md))

### args Examples

```json
[
  "bw4dl-smaaa-aaaaa-qaacq-cai",
  1,
  "ETHUSD"
]
```

```json
[]
```
