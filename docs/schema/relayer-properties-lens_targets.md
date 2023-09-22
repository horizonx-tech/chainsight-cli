# Untitled object in Chainsight Relayer specification Schema

```txt
undefined#/properties/lens_targets
```

targets for the lens. Only used when the datasource canister is a algorithm\_lens

| Abstract            | Extensible | Status         | Identifiable | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                      |
| :------------------ | :--------- | :------------- | :----------- | :---------------- | :-------------------- | :------------------ | :-------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | No           | Forbidden         | Allowed               | none                | [relayer.json\*](../../out/relayer.json "open original schema") |

## lens\_targets Type

`object` ([Details](relayer-properties-lens_targets.md))

# lens\_targets Properties

| Property                    | Type    | Required | Nullable       | Defined by                                                                                                                                                |
| :-------------------------- | :------ | :------- | :------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [identifiers](#identifiers) | `array` | Required | cannot be null | [Chainsight Relayer specification](relayer-properties-lens_targets-properties-identifiers.md "undefined#/properties/lens_targets/properties/identifiers") |

## identifiers

canister ids of the lens targets. If the canister calls an algorithm\_lens and the lens calls 3 canisters, you must set 3 canister ids here

`identifiers`

*   is required

*   Type: `string[]`

*   cannot be null

*   defined in: [Chainsight Relayer specification](relayer-properties-lens_targets-properties-identifiers.md "undefined#/properties/lens_targets/properties/identifiers")

### identifiers Type

`string[]`

### identifiers Examples

```json
[
  "bw4dl-smaaa-aaaaa-qaacq-cai",
  "bw4dl-smaaa-aaaaa-qaacq-cai",
  "bw4dl-smaaa-aaaaa-qaacq-cai"
]
```

```json
[
  "bw4dl-smaaa-aaaaa-qaacq-cai"
]
```
