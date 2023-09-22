# label Schema

```txt
undefined#/properties/datasource/properties/methods/items/properties/label
```

label of this method

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [algorithm\_lens.json\*](../../out/algorithm_lens.json "open original schema") |

## label Type

`string` ([label](algorithm_lens-properties-datasource-properties-methods-method-properties-label.md))

## label Constraints

**pattern**: the string must match the following regular expression:&#x20;

```regexp
^[a-z0-9_]+$
```

[try pattern](https://regexr.com/?expression=%5E%5Ba-z0-9_%5D%2B%24 "try regular expression with regexr.com")

## label Examples

```json
"get_ethusd_price_from_coingecko"
```

```json
"get_ethusd_price_from_chainlink"
```
