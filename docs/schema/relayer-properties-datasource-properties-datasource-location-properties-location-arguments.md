# location arguments Schema

```txt
undefined#/properties/datasource/properties/location/properties/args
```



| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                      |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :-------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Allowed               | none                | [relayer.json\*](../../out/relayer.json "open original schema") |

## args Type

`object` ([location arguments](relayer-properties-datasource-properties-datasource-location-properties-location-arguments.md))

# args Properties

| Property             | Type     | Required | Nullable       | Defined by                                                                                                                                                                                                                                            |
| :------------------- | :------- | :------- | :------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [id\_type](#id_type) | `string` | Required | cannot be null | [Chainsight Relayer specification](relayer-properties-datasource-properties-datasource-location-properties-location-arguments-properties-type-of-the-id.md "undefined#/properties/datasource/properties/location/properties/args/properties/id_type") |

## id\_type

canister\_name: id is interpreted as canister name, principal\_id: id is interpreted as principal id

`id_type`

*   is required

*   Type: `string` ([type of the id](relayer-properties-datasource-properties-datasource-location-properties-location-arguments-properties-type-of-the-id.md))

*   cannot be null

*   defined in: [Chainsight Relayer specification](relayer-properties-datasource-properties-datasource-location-properties-location-arguments-properties-type-of-the-id.md "undefined#/properties/datasource/properties/location/properties/args/properties/id_type")

### id\_type Type

`string` ([type of the id](relayer-properties-datasource-properties-datasource-location-properties-location-arguments-properties-type-of-the-id.md))

### id\_type Constraints

**pattern**: the string must match the following regular expression:&#x20;

```regexp
^(canister_name|principal_id)$
```

[try pattern](https://regexr.com/?expression=%5E\(canister_name%7Cprincipal_id\)%24 "try regular expression with regexr.com")

### id\_type Examples

```json
"canister_name"
```

```json
"principal_id"
```
