# Untitled object in Chainsight HTTP Event Indexer specification Schema

```txt
https://raw.githubusercontent.com/horizonx-tech/chainsight-cli/main/resources/schema/snapshot_indexer_http.json#/properties/datasource
```



| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                                    |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :-------------------------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Allowed               | none                | [snapshot\_indexer\_http.json\*](../../out/snapshot_indexer_http.json "open original schema") |

## datasource Type

`object` ([Details](snapshot_indexer_http-properties-datasource.md))

# datasource Properties

| Property            | Type     | Required | Nullable       | Defined by                                                                                                                                                                                                                              |
| :------------------ | :------- | :------- | :------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [url](#url)         | `string` | Required | cannot be null | [Chainsight HTTP Event Indexer specification](snapshot_indexer_http-properties-datasource-properties-url-of-the-datasource.md "#/properties/datasource/properties/url#/properties/datasource/properties/url")                           |
| [headers](#headers) | `object` | Optional | cannot be null | [Chainsight HTTP Event Indexer specification](snapshot_indexer_http-properties-datasource-properties-http-request-headers-for-the-datasource.md "#/properties/datasource/properties/headers#/properties/datasource/properties/headers") |
| [queries](#queries) | `object` | Optional | cannot be null | [Chainsight HTTP Event Indexer specification](snapshot_indexer_http-properties-datasource-properties-query-parameters-for-the-datasource.md "#/properties/datasource/properties/queries#/properties/datasource/properties/queries")     |

## url



`url`

*   is required

*   Type: `string` ([url of the datasource](snapshot_indexer_http-properties-datasource-properties-url-of-the-datasource.md))

*   cannot be null

*   defined in: [Chainsight HTTP Event Indexer specification](snapshot_indexer_http-properties-datasource-properties-url-of-the-datasource.md "#/properties/datasource/properties/url#/properties/datasource/properties/url")

### url Type

`string` ([url of the datasource](snapshot_indexer_http-properties-datasource-properties-url-of-the-datasource.md))

### url Constraints

**pattern**: the string must match the following regular expression:&#x20;

```regexp
^(http|https)://[a-zA-Z0-9_./-]+$
```

[try pattern](https://regexr.com/?expression=%5E\(http%7Chttps\)%3A%2F%2F%5Ba-zA-Z0-9_.%2F-%5D%2B%24 "try regular expression with regexr.com")

### url Examples

```json
"https://api.etherscan.io/api"
```

## headers

HTTP request headers for the datasource

`headers`

*   is optional

*   Type: `object` ([HTTP request headers for the datasource](snapshot_indexer_http-properties-datasource-properties-http-request-headers-for-the-datasource.md))

*   cannot be null

*   defined in: [Chainsight HTTP Event Indexer specification](snapshot_indexer_http-properties-datasource-properties-http-request-headers-for-the-datasource.md "#/properties/datasource/properties/headers#/properties/datasource/properties/headers")

### headers Type

`object` ([HTTP request headers for the datasource](snapshot_indexer_http-properties-datasource-properties-http-request-headers-for-the-datasource.md))

### headers Examples

```json
{
  "Content-Type": "application/json"
}
```

## queries

query parameter names and values for the datasource

`queries`

*   is optional

*   Type: `object` ([query parameters for the datasource](snapshot_indexer_http-properties-datasource-properties-query-parameters-for-the-datasource.md))

*   cannot be null

*   defined in: [Chainsight HTTP Event Indexer specification](snapshot_indexer_http-properties-datasource-properties-query-parameters-for-the-datasource.md "#/properties/datasource/properties/queries#/properties/datasource/properties/queries")

### queries Type

`object` ([query parameters for the datasource](snapshot_indexer_http-properties-datasource-properties-query-parameters-for-the-datasource.md))

### queries Examples

```json
{
  "id": 2112
}
```

```json
{
  "vs_currencies": "usd"
}
```

```json
{
  "include_24hr_vol": true
}
```

```json
{
  "precision": 18
}
```
