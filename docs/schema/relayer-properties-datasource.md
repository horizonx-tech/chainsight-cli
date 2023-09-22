# Untitled object in Chainsight Relayer specification Schema

```txt
undefined#/properties/datasource
```



| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                      |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :-------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Allowed               | none                | [relayer.json\*](../../out/relayer.json "open original schema") |

## datasource Type

`object` ([Details](relayer-properties-datasource.md))

# datasource Properties

| Property              | Type     | Required | Nullable       | Defined by                                                                                                                                                 |
| :-------------------- | :------- | :------- | :------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [type](#type)         | `string` | Required | cannot be null | [Chainsight Relayer specification](relayer-properties-datasource-properties-type-of-the-datasource.md "undefined#/properties/datasource/properties/type")  |
| [location](#location) | `object` | Required | cannot be null | [Chainsight Relayer specification](relayer-properties-datasource-properties-datasource-location.md "undefined#/properties/datasource/properties/location") |
| [method](#method)     | `object` | Required | cannot be null | [Chainsight Relayer specification](relayer-properties-datasource-properties-datasource-method.md "undefined#/properties/datasource/properties/method")     |

## type

currently only supports 'canister'

`type`

*   is required

*   Type: `string` ([type of the datasource](relayer-properties-datasource-properties-type-of-the-datasource.md))

*   cannot be null

*   defined in: [Chainsight Relayer specification](relayer-properties-datasource-properties-type-of-the-datasource.md "undefined#/properties/datasource/properties/type")

### type Type

`string` ([type of the datasource](relayer-properties-datasource-properties-type-of-the-datasource.md))

### type Constraints

**pattern**: the string must match the following regular expression:&#x20;

```regexp
^(canister)$
```

[try pattern](https://regexr.com/?expression=%5E\(canister\)%24 "try regular expression with regexr.com")

### type Examples

```json
"canister"
```

## location



`location`

*   is required

*   Type: `object` ([datasource location](relayer-properties-datasource-properties-datasource-location.md))

*   cannot be null

*   defined in: [Chainsight Relayer specification](relayer-properties-datasource-properties-datasource-location.md "undefined#/properties/datasource/properties/location")

### location Type

`object` ([datasource location](relayer-properties-datasource-properties-datasource-location.md))

## method

method to call on the canister

`method`

*   is required

*   Type: `object` ([datasource method](relayer-properties-datasource-properties-datasource-method.md))

*   cannot be null

*   defined in: [Chainsight Relayer specification](relayer-properties-datasource-properties-datasource-method.md "undefined#/properties/datasource/properties/method")

### method Type

`object` ([datasource method](relayer-properties-datasource-properties-datasource-method.md))
