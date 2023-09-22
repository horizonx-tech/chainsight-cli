# Untitled object in Chainsight Snapshot Indexer specification Schema

```txt
undefined#/properties/datasource
```



| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                         |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :--------------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Allowed               | none                | [snapshot\_indexer.json\*](../../out/snapshot_indexer.json "open original schema") |

## datasource Type

`object` ([Details](snapshot_indexer-properties-datasource.md))

# datasource Properties

| Property              | Type     | Required | Nullable       | Defined by                                                                                                                                                                          |
| :-------------------- | :------- | :------- | :------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [type](#type)         | `string` | Required | cannot be null | [Chainsight Snapshot Indexer specification](snapshot_indexer-properties-datasource-properties-type-of-the-datasource.md "undefined#/properties/datasource/properties/type")         |
| [location](#location) | `object` | Required | cannot be null | [Chainsight Snapshot Indexer specification](snapshot_indexer-properties-datasource-properties-location-of-the-datasource.md "undefined#/properties/datasource/properties/location") |
| [method](#method)     | `object` | Required | cannot be null | [Chainsight Snapshot Indexer specification](snapshot_indexer-properties-datasource-properties-method.md "undefined#/properties/datasource/properties/method")                       |

## type

type of the datasource.If you want to get data from HTTP outcall, you can use HTTP Snapshot Indexer

`type`

*   is required

*   Type: `string` ([type of the datasource](snapshot_indexer-properties-datasource-properties-type-of-the-datasource.md))

*   cannot be null

*   defined in: [Chainsight Snapshot Indexer specification](snapshot_indexer-properties-datasource-properties-type-of-the-datasource.md "undefined#/properties/datasource/properties/type")

### type Type

`string` ([type of the datasource](snapshot_indexer-properties-datasource-properties-type-of-the-datasource.md))

### type Constraints

**enum**: the value of this property must be equal to one of the following values:

| Value        | Explanation |
| :----------- | :---------- |
| `"contract"` |             |
| `"canister"` |             |

**pattern**: the string must match the following regular expression:&#x20;

```regexp
^(contract|canister)$
```

[try pattern](https://regexr.com/?expression=%5E\(contract%7Ccanister\)%24 "try regular expression with regexr.com")

### type Examples

```json
"contract"
```

```json
"canister"
```

## location

location of the datasource. For contract, it is the contract address. For canister, it is the canister id or canister name defined in dfx.json.

`location`

*   is required

*   Type: `object` ([location of the datasource](snapshot_indexer-properties-datasource-properties-location-of-the-datasource.md))

*   cannot be null

*   defined in: [Chainsight Snapshot Indexer specification](snapshot_indexer-properties-datasource-properties-location-of-the-datasource.md "undefined#/properties/datasource/properties/location")

### location Type

`object` ([location of the datasource](snapshot_indexer-properties-datasource-properties-location-of-the-datasource.md))

## method

method of the datasource. The canister will call this method to get data.

`method`

*   is required

*   Type: `object` ([method](snapshot_indexer-properties-datasource-properties-method.md))

*   cannot be null

*   defined in: [Chainsight Snapshot Indexer specification](snapshot_indexer-properties-datasource-properties-method.md "undefined#/properties/datasource/properties/method")

### method Type

`object` ([method](snapshot_indexer-properties-datasource-properties-method.md))
