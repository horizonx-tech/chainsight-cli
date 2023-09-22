# HTTP request headers for the datasource Schema

```txt
undefined#/properties/datasource/properties/headers
```

HTTP request headers for the datasource

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                                    |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :-------------------------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [snapshot\_indexer\_http.json\*](../../out/snapshot_indexer_http.json "open original schema") |

## headers Type

`object` ([HTTP request headers for the datasource](snapshot_indexer_http-properties-datasource-properties-http-request-headers-for-the-datasource.md))

## headers Examples

```json
{
  "Content-Type": "application/json"
}
```

# headers Properties

| Property            | Type     | Required | Nullable       | Defined by                                                                                                                                                                                                                                                                 |
| :------------------ | :------- | :------- | :------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `^[a-zA-Z0-9_\-]+$` | `string` | Optional | cannot be null | [Chainsight HTTP Event Indexer specification](snapshot_indexer_http-properties-datasource-properties-http-request-headers-for-the-datasource-patternproperties-a-za-z0-9_-.md "undefined#/properties/datasource/properties/headers/patternProperties/^\[a-zA-Z0-9_\\-]+$") |

## Pattern: `^[a-zA-Z0-9_\-]+$`



`^[a-zA-Z0-9_\-]+$`

*   is optional

*   Type: `string`

*   cannot be null

*   defined in: [Chainsight HTTP Event Indexer specification](snapshot_indexer_http-properties-datasource-properties-http-request-headers-for-the-datasource-patternproperties-a-za-z0-9_-.md "undefined#/properties/datasource/properties/headers/patternProperties/^\[a-zA-Z0-9_\\-]+$")

### ^\[a-zA-Z0-9\_\\-]+$ Type

`string`
