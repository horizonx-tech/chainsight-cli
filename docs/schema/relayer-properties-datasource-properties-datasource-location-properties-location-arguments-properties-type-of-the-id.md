# type of the id Schema

```txt
undefined#/properties/datasource/properties/location/properties/args/properties/id_type
```

canister\_name: id is interpreted as canister name, principal\_id: id is interpreted as principal id

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                      |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :-------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [relayer.json\*](../../out/relayer.json "open original schema") |

## id\_type Type

`string` ([type of the id](relayer-properties-datasource-properties-datasource-location-properties-location-arguments-properties-type-of-the-id.md))

## id\_type Constraints

**pattern**: the string must match the following regular expression:&#x20;

```regexp
^(canister_name|principal_id)$
```

[try pattern](https://regexr.com/?expression=%5E\(canister_name%7Cprincipal_id\)%24 "try regular expression with regexr.com")

## id\_type Examples

```json
"canister_name"
```

```json
"principal_id"
```
