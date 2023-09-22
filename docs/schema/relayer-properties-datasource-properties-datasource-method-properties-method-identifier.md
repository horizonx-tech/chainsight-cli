# method identifier Schema

```txt
undefined#/properties/datasource/properties/method/properties/identifier
```

method identifier. You can get it from candid definition

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                      |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :-------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [relayer.json\*](../../out/relayer.json "open original schema") |

## identifier Type

`string` ([method identifier](relayer-properties-datasource-properties-datasource-method-properties-method-identifier.md))

## identifier Examples

```json
"get_last_snapshot : () -> (record { value : text; timestamp : nat64 })"
```
