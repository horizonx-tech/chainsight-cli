# url of the datasource Schema

```txt
#/properties/datasource/properties/url#/properties/datasource/properties/url
```



| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                                    |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :-------------------------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [snapshot\_indexer\_http.json\*](../../out/snapshot_indexer_http.json "open original schema") |

## url Type

`string` ([url of the datasource](snapshot_indexer_http-properties-datasource-properties-url-of-the-datasource.md))

## url Constraints

**pattern**: the string must match the following regular expression:&#x20;

```regexp
^(http|https)://[a-zA-Z0-9_./-]+$
```

[try pattern](https://regexr.com/?expression=%5E\(http%7Chttps\)%3A%2F%2F%5Ba-zA-Z0-9_.%2F-%5D%2B%24 "try regular expression with regexr.com")

## url Examples

```json
"https://api.etherscan.io/api"
```
