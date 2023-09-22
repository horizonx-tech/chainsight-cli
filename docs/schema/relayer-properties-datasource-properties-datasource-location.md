# datasource location Schema

```txt
undefined#/properties/datasource/properties/location
```



| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                      |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :-------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Allowed               | none                | [relayer.json\*](../../out/relayer.json "open original schema") |

## location Type

`object` ([datasource location](relayer-properties-datasource-properties-datasource-location.md))

# location Properties

| Property      | Type     | Required | Nullable       | Defined by                                                                                                                                                                                               |
| :------------ | :------- | :------- | :------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [id](#id)     | `string` | Required | cannot be null | [Chainsight Relayer specification](relayer-properties-datasource-properties-datasource-location-properties-canister-id.md "undefined#/properties/datasource/properties/location/properties/id")          |
| [args](#args) | `object` | Required | cannot be null | [Chainsight Relayer specification](relayer-properties-datasource-properties-datasource-location-properties-location-arguments.md "undefined#/properties/datasource/properties/location/properties/args") |

## id

canister id or name of the datasource

`id`

*   is required

*   Type: `string` ([canister id](relayer-properties-datasource-properties-datasource-location-properties-canister-id.md))

*   cannot be null

*   defined in: [Chainsight Relayer specification](relayer-properties-datasource-properties-datasource-location-properties-canister-id.md "undefined#/properties/datasource/properties/location/properties/id")

### id Type

`string` ([canister id](relayer-properties-datasource-properties-datasource-location-properties-canister-id.md))

### id Constraints

**pattern**: the string must match the following regular expression:&#x20;

```regexp
^[a-z0-9_-]+$
```

[try pattern](https://regexr.com/?expression=%5E%5Ba-z0-9_-%5D%2B%24 "try regular expression with regexr.com")

### id Examples

```json
"algorithm_lens_ethusd"
```

```json
"bw4dl-smaaa-aaaaa-qaacq-cai"
```

## args



`args`

*   is required

*   Type: `object` ([location arguments](relayer-properties-datasource-properties-datasource-location-properties-location-arguments.md))

*   cannot be null

*   defined in: [Chainsight Relayer specification](relayer-properties-datasource-properties-datasource-location-properties-location-arguments.md "undefined#/properties/datasource/properties/location/properties/args")

### args Type

`object` ([location arguments](relayer-properties-datasource-properties-datasource-location-properties-location-arguments.md))
