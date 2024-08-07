# Chainsight Event Indexer specification

- [1. Property `Chainsight Event Indexer specification > version`](#version)
- [2. Property `Chainsight Event Indexer specification > metadata`](#metadata)
  - [2.1. Property `Chainsight Event Indexer specification > metadata > label`](#metadata_label)
  - [2.2. Property `Chainsight Event Indexer specification > metadata > type`](#metadata_type)
  - [2.3. Property `Chainsight Event Indexer specification > metadata > description`](#metadata_description)
  - [2.4. Property `Chainsight Event Indexer specification > metadata > tags`](#metadata_tags)
    - [2.4.1. Chainsight Event Indexer specification > metadata > tags > tags items](#autogenerated_heading_2)
- [3. Property `Chainsight Event Indexer specification > datasource`](#datasource)
  - [3.1. Property `Chainsight Event Indexer specification > datasource > id`](#datasource_id)
  - [3.2. Property `Chainsight Event Indexer specification > datasource > event`](#datasource_event)
    - [3.2.1. Property `Chainsight Event Indexer specification > datasource > event > identifier`](#datasource_event_identifier)
    - [3.2.2. Property `Chainsight Event Indexer specification > datasource > event > interface`](#datasource_event_interface)
  - [3.3. Property `Chainsight Event Indexer specification > datasource > from`](#datasource_from)
  - [3.4. Property `Chainsight Event Indexer specification > datasource > network`](#datasource_network)
    - [3.4.1. Property `Chainsight Event Indexer specification > datasource > network > chain_id`](#datasource_network_chain_id)
    - [3.4.2. Property `Chainsight Event Indexer specification > datasource > network > rpc_url`](#datasource_network_rpc_url)
  - [3.5. Property `Chainsight Event Indexer specification > datasource > contract_type`](#datasource_contract_type)
  - [3.6. Property `Chainsight Event Indexer specification > datasource > batch_size`](#datasource_batch_size)
- [4. Property `Chainsight Event Indexer specification > timer_settings`](#timer_settings)
  - [4.1. Property `Chainsight Event Indexer specification > timer_settings > interval_sec`](#timer_settings_interval_sec)
  - [4.2. Property `Chainsight Event Indexer specification > timer_settings > delay_sec`](#timer_settings_delay_sec)
  - [4.3. Property `Chainsight Event Indexer specification > timer_settings > is_round_start_timing`](#timer_settings_is_round_start_timing)
- [5. Property `Chainsight Event Indexer specification > cycles`](#cycles)
  - [5.1. Property `Chainsight Event Indexer specification > cycles > refueling_interval`](#cycles_refueling_interval)
  - [5.2. Property `Chainsight Event Indexer specification > cycles > vault_intial_supply`](#cycles_vault_intial_supply)
  - [5.3. Property `Chainsight Event Indexer specification > cycles > indexer`](#cycles_indexer)
    - [5.3.1. Property `Chainsight Event Indexer specification > cycles > indexer > initial_supply`](#cycles_indexer_initial_supply)
    - [5.3.2. Property `Chainsight Event Indexer specification > cycles > indexer > refueling_threshold`](#cycles_indexer_refueling_threshold)
    - [5.3.3. Property `Chainsight Event Indexer specification > cycles > indexer > refueling_amount`](#cycles_indexer_refueling_amount)
  - [5.4. Property `Chainsight Event Indexer specification > cycles > db`](#cycles_db)
    - [5.4.1. Property `Chainsight Event Indexer specification > cycles > db > initial_supply`](#cycles_db_initial_supply)
    - [5.4.2. Property `Chainsight Event Indexer specification > cycles > db > refueling_threshold`](#cycles_db_refueling_threshold)
    - [5.4.3. Property `Chainsight Event Indexer specification > cycles > db > refueling_amount`](#cycles_db_refueling_amount)
  - [5.5. Property `Chainsight Event Indexer specification > cycles > proxy`](#cycles_proxy)
    - [5.5.1. Property `Chainsight Event Indexer specification > cycles > proxy > initial_supply`](#cycles_proxy_initial_supply)
    - [5.5.2. Property `Chainsight Event Indexer specification > cycles > proxy > refueling_threshold`](#cycles_proxy_refueling_threshold)
    - [5.5.3. Property `Chainsight Event Indexer specification > cycles > proxy > refueling_amount`](#cycles_proxy_refueling_amount)

**Title:** Chainsight Event Indexer specification

|                           |                                                         |
| ------------------------- | ------------------------------------------------------- |
| **Type**                  | `object`                                                |
| **Required**              | No                                                      |
| **Additional properties** | [[Not allowed]](# "Additional Properties not allowed.") |

**Description:** Chainsight Event Indexer specification

| Property                             | Pattern | Type           | Deprecated | Definition | Title/Description                     |
| ------------------------------------ | ------- | -------------- | ---------- | ---------- | ------------------------------------- |
| + [version](#version )               | No      | string         | No         | -          | specification version of the canister |
| + [metadata](#metadata )             | No      | object         | No         | -          | metadata                              |
| + [datasource](#datasource )         | No      | object         | No         | -          | datasource                            |
| + [timer_settings](#timer_settings ) | No      | object         | No         | -          | timer_settings                        |
| - [cycles](#cycles )                 | No      | object or null | No         | -          | cycles                                |

## <a name="version"></a>1. Property `Chainsight Event Indexer specification > version`

**Title:** specification version of the canister

|              |          |
| ------------ | -------- |
| **Type**     | `string` |
| **Required** | Yes      |

**Example:** 

```json
"v1"
```

| Restrictions                      |                                                                                     |
| --------------------------------- | ----------------------------------------------------------------------------------- |
| **Must match regular expression** | ```^(v1)$``` [Test](https://regex101.com/?regex=%5E%28v1%29%24&testString=%22v1%22) |

## <a name="metadata"></a>2. Property `Chainsight Event Indexer specification > metadata`

**Title:** metadata

|                           |                                                         |
| ------------------------- | ------------------------------------------------------- |
| **Type**                  | `object`                                                |
| **Required**              | Yes                                                     |
| **Additional properties** | [[Not allowed]](# "Additional Properties not allowed.") |

| Property                                | Pattern | Type            | Deprecated | Definition | Title/Description           |
| --------------------------------------- | ------- | --------------- | ---------- | ---------- | --------------------------- |
| + [label](#metadata_label )             | No      | string          | No         | -          | label for the canister      |
| + [type](#metadata_type )               | No      | string          | No         | -          | type of the canister        |
| - [description](#metadata_description ) | No      | string          | No         | -          | description of the canister |
| - [tags](#metadata_tags )               | No      | array of string | No         | -          | tags for the canister       |

### <a name="metadata_label"></a>2.1. Property `Chainsight Event Indexer specification > metadata > label`

**Title:** label for the canister

|              |          |
| ------------ | -------- |
| **Type**     | `string` |
| **Required** | Yes      |

**Example:** 

```json
"relayer_ethusd"
```

### <a name="metadata_type"></a>2.2. Property `Chainsight Event Indexer specification > metadata > type`

**Title:** type of the canister

|              |          |
| ------------ | -------- |
| **Type**     | `string` |
| **Required** | Yes      |

**Examples:** 

```json
"event_indexer"
```

```json
"algorithm_indexer"
```

```json
"snapshot_indexer_icp"
```

```json
"snapshot_indexer_evm"
```

```json
"snapshot_indexer_https"
```

```json
"relayer"
```

```json
"algorithm_lens"
```

| Restrictions                      |                                                                                                                                                                                                                                                                                                                                                            |
| --------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Must match regular expression** | ```^(event_indexer\|algorithm_indexer\|snapshot_indexer_icp\|snapshot_indexer_evm\|snapshot_indexer_https\|relayer\|algorithm_lens)$``` [Test](https://regex101.com/?regex=%5E%28event_indexer%7Calgorithm_indexer%7Csnapshot_indexer_icp%7Csnapshot_indexer_evm%7Csnapshot_indexer_https%7Crelayer%7Calgorithm_lens%29%24&testString=%22event_indexer%22) |

### <a name="metadata_description"></a>2.3. Property `Chainsight Event Indexer specification > metadata > description`

**Title:** description of the canister

|              |          |
| ------------ | -------- |
| **Type**     | `string` |
| **Required** | No       |

**Description:** Can be used to filter canisters in the UI

**Example:** 

```json
"Relayer for ETHUSD"
```

### <a name="metadata_tags"></a>2.4. Property `Chainsight Event Indexer specification > metadata > tags`

**Title:** tags for the canister

|              |                   |
| ------------ | ----------------- |
| **Type**     | `array of string` |
| **Required** | No                |

**Description:** Can be used to filter canisters in the UI

|                      | Array restrictions |
| -------------------- | ------------------ |
| **Min items**        | N/A                |
| **Max items**        | N/A                |
| **Items unicity**    | False              |
| **Additional items** | False              |
| **Tuple validation** | See below          |

| Each item of this array must be    | Description |
| ---------------------------------- | ----------- |
| [tags items](#metadata_tags_items) | -           |

#### <a name="autogenerated_heading_2"></a>2.4.1. Chainsight Event Indexer specification > metadata > tags > tags items

|              |          |
| ------------ | -------- |
| **Type**     | `string` |
| **Required** | No       |

**Examples:** 

```json
"Ethereum"
```

```json
"Relayer"
```

```json
"Account"
```

| Restrictions                      |                                                                                                                 |
| --------------------------------- | --------------------------------------------------------------------------------------------------------------- |
| **Must match regular expression** | ```^[a-zA-Z0-9_-]+$``` [Test](https://regex101.com/?regex=%5E%5Ba-zA-Z0-9_-%5D%2B%24&testString=%22Ethereum%22) |

## <a name="datasource"></a>3. Property `Chainsight Event Indexer specification > datasource`

**Title:** datasource

|                           |                                                         |
| ------------------------- | ------------------------------------------------------- |
| **Type**                  | `object`                                                |
| **Required**              | Yes                                                     |
| **Additional properties** | [[Not allowed]](# "Additional Properties not allowed.") |

**Description:** datasource for the canister

| Property                                      | Pattern | Type           | Deprecated | Definition | Title/Description |
| --------------------------------------------- | ------- | -------------- | ---------- | ---------- | ----------------- |
| + [id](#datasource_id )                       | No      | object         | No         | -          | id                |
| + [event](#datasource_event )                 | No      | object         | No         | -          | event             |
| + [from](#datasource_from )                   | No      | number         | No         | -          | from              |
| + [network](#datasource_network )             | No      | object         | No         | -          | network           |
| - [contract_type](#datasource_contract_type ) | No      | string         | No         | -          | contract_type     |
| - [batch_size](#datasource_batch_size )       | No      | number or null | No         | -          | batch_size        |

### <a name="datasource_id"></a>3.1. Property `Chainsight Event Indexer specification > datasource > id`

**Title:** id

|                           |                                                                           |
| ------------------------- | ------------------------------------------------------------------------- |
| **Type**                  | `object`                                                                  |
| **Required**              | Yes                                                                       |
| **Additional properties** | [[Any type: allowed]](# "Additional Properties of any type are allowed.") |

**Description:** contract address

**Example:** 

```json
"0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512"
```

| Restrictions                      |                                                                                                                                                                       |
| --------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Must match regular expression** | ```^(0x)?[0-9a-fA-F]{40}$``` [Test](https://regex101.com/?regex=%5E%280x%29%3F%5B0-9a-fA-F%5D%7B40%7D%24&testString=%220xe7f1725E7734CE288F8367e1Bb143E90bb3F0512%22) |

### <a name="datasource_event"></a>3.2. Property `Chainsight Event Indexer specification > datasource > event`

**Title:** event

|                           |                                                         |
| ------------------------- | ------------------------------------------------------- |
| **Type**                  | `object`                                                |
| **Required**              | Yes                                                     |
| **Additional properties** | [[Not allowed]](# "Additional Properties not allowed.") |

**Description:** event identifier to save

| Property                                      | Pattern | Type           | Deprecated | Definition | Title/Description |
| --------------------------------------------- | ------- | -------------- | ---------- | ---------- | ----------------- |
| + [identifier](#datasource_event_identifier ) | No      | string         | No         | -          | identifier        |
| + [interface](#datasource_event_interface )   | No      | string or null | No         | -          | interface         |

#### <a name="datasource_event_identifier"></a>3.2.1. Property `Chainsight Event Indexer specification > datasource > event > identifier`

**Title:** identifier

|              |          |
| ------------ | -------- |
| **Type**     | `string` |
| **Required** | Yes      |

**Description:** event name. You can find it in the abi

**Example:** 

```json
"Transfer"
```

| Restrictions                      |                                                                                                                 |
| --------------------------------- | --------------------------------------------------------------------------------------------------------------- |
| **Must match regular expression** | ```^[a-zA-Z0-9_-]+$``` [Test](https://regex101.com/?regex=%5E%5Ba-zA-Z0-9_-%5D%2B%24&testString=%22Transfer%22) |

#### <a name="datasource_event_interface"></a>3.2.2. Property `Chainsight Event Indexer specification > datasource > event > interface`

**Title:** interface

|              |                  |
| ------------ | ---------------- |
| **Type**     | `string or null` |
| **Required** | Yes              |

**Description:** abi json file. It must be in ./interfaces folder

**Example:** 

```json
"IERC20.json"
```

| Restrictions                      |                                                                                                                                  |
| --------------------------------- | -------------------------------------------------------------------------------------------------------------------------------- |
| **Must match regular expression** | ```^[a-zA-Z0-9_-]+\.json$``` [Test](https://regex101.com/?regex=%5E%5Ba-zA-Z0-9_-%5D%2B%5C.json%24&testString=%22IERC20.json%22) |

### <a name="datasource_from"></a>3.3. Property `Chainsight Event Indexer specification > datasource > from`

**Title:** from

|              |          |
| ------------ | -------- |
| **Type**     | `number` |
| **Required** | Yes      |

**Description:** block number to start the query from

**Example:** 

```json
0
```

### <a name="datasource_network"></a>3.4. Property `Chainsight Event Indexer specification > datasource > network`

**Title:** network

|                           |                                                         |
| ------------------------- | ------------------------------------------------------- |
| **Type**                  | `object`                                                |
| **Required**              | Yes                                                     |
| **Additional properties** | [[Not allowed]](# "Additional Properties not allowed.") |

**Description:** chain id and rpc url

| Property                                    | Pattern | Type   | Deprecated | Definition | Title/Description |
| ------------------------------------------- | ------- | ------ | ---------- | ---------- | ----------------- |
| + [chain_id](#datasource_network_chain_id ) | No      | number | No         | -          | chain_id          |
| + [rpc_url](#datasource_network_rpc_url )   | No      | string | No         | -          | rpc_url           |

#### <a name="datasource_network_chain_id"></a>3.4.1. Property `Chainsight Event Indexer specification > datasource > network > chain_id`

**Title:** chain_id

|              |          |
| ------------ | -------- |
| **Type**     | `number` |
| **Required** | Yes      |

**Description:** chain id

**Example:** 

```json
1
```

#### <a name="datasource_network_rpc_url"></a>3.4.2. Property `Chainsight Event Indexer specification > datasource > network > rpc_url`

**Title:** rpc_url

|              |          |
| ------------ | -------- |
| **Type**     | `string` |
| **Required** | Yes      |

**Description:** rpc url

**Example:** 

```json
"https://eth.llamarpc.com"
```

| Restrictions                      |                                                                                                                           |
| --------------------------------- | ------------------------------------------------------------------------------------------------------------------------- |
| **Must match regular expression** | ```^https?://``` [Test](https://regex101.com/?regex=%5Ehttps%3F%3A%2F%2F&testString=%22https%3A%2F%2Feth.llamarpc.com%22) |

### <a name="datasource_contract_type"></a>3.5. Property `Chainsight Event Indexer specification > datasource > contract_type`

**Title:** contract_type

|              |          |
| ------------ | -------- |
| **Type**     | `string` |
| **Required** | No       |

**Description:** type of the contract. It is not required, but it is useful to filter canisters in the UI

**Examples:** 

```json
"ERC20"
```

```json
"DEX"
```

### <a name="datasource_batch_size"></a>3.6. Property `Chainsight Event Indexer specification > datasource > batch_size`

**Title:** batch_size

|              |                  |
| ------------ | ---------------- |
| **Type**     | `number or null` |
| **Required** | No               |
| **Default**  | `500`            |

**Description:** number of blocks to save in a single transaction. This is useful to avoid exceeding Http body size limit

**Example:** 

```json
100
```

## <a name="timer_settings"></a>4. Property `Chainsight Event Indexer specification > timer_settings`

**Title:** timer_settings

|                           |                                                         |
| ------------------------- | ------------------------------------------------------- |
| **Type**                  | `object`                                                |
| **Required**              | Yes                                                     |
| **Additional properties** | [[Not allowed]](# "Additional Properties not allowed.") |

**Description:** timer execution settings

| Property                                                          | Pattern | Type            | Deprecated | Definition | Title/Description |
| ----------------------------------------------------------------- | ------- | --------------- | ---------- | ---------- | ----------------- |
| + [interval_sec](#timer_settings_interval_sec )                   | No      | number          | No         | -          | interval_sec      |
| - [delay_sec](#timer_settings_delay_sec )                         | No      | number or null  | No         | -          | delay_sec         |
| - [is_round_start_timing](#timer_settings_is_round_start_timing ) | No      | boolean or null | No         | -          | delay_sec         |

### <a name="timer_settings_interval_sec"></a>4.1. Property `Chainsight Event Indexer specification > timer_settings > interval_sec`

**Title:** interval_sec

|              |          |
| ------------ | -------- |
| **Type**     | `number` |
| **Required** | Yes      |

**Description:** interval of the canister invocation in seconds

**Example:** 

```json
3600
```

### <a name="timer_settings_delay_sec"></a>4.2. Property `Chainsight Event Indexer specification > timer_settings > delay_sec`

**Title:** delay_sec

|              |                  |
| ------------ | ---------------- |
| **Type**     | `number or null` |
| **Required** | No               |

**Description:** delay of the canister invocation in seconds

**Example:** 

```json
60
```

### <a name="timer_settings_is_round_start_timing"></a>4.3. Property `Chainsight Event Indexer specification > timer_settings > is_round_start_timing`

**Title:** delay_sec

|              |                   |
| ------------ | ----------------- |
| **Type**     | `boolean or null` |
| **Required** | No                |

**Description:** whether to round execution timing by interval or not

**Example:** 

```json
true
```

## <a name="cycles"></a>5. Property `Chainsight Event Indexer specification > cycles`

**Title:** cycles

|              |                  |
| ------------ | ---------------- |
| **Type**     | `object or null` |
| **Required** | No               |

**Description:** manage component cycles

| Property                                              | Pattern | Type           | Deprecated | Definition | Title/Description   |
| ----------------------------------------------------- | ------- | -------------- | ---------- | ---------- | ------------------- |
| - [refueling_interval](#cycles_refueling_interval )   | No      | number or null | No         | -          | refueling_interval  |
| - [vault_intial_supply](#cycles_vault_intial_supply ) | No      | number or null | No         | -          | vault_intial_supply |
| - [indexer](#cycles_indexer )                         | No      | object or null | No         | -          | indexer             |
| - [db](#cycles_db )                                   | No      | object or null | No         | -          | db                  |
| - [proxy](#cycles_proxy )                             | No      | object or null | No         | -          | proxy               |

### <a name="cycles_refueling_interval"></a>5.1. Property `Chainsight Event Indexer specification > cycles > refueling_interval`

**Title:** refueling_interval

|              |                  |
| ------------ | ---------------- |
| **Type**     | `number or null` |
| **Required** | No               |

**Description:** interval of the refueling to canisters in seconds

**Example:** 

```json
86400
```

### <a name="cycles_vault_intial_supply"></a>5.2. Property `Chainsight Event Indexer specification > cycles > vault_intial_supply`

**Title:** vault_intial_supply

|              |                  |
| ------------ | ---------------- |
| **Type**     | `number or null` |
| **Required** | No               |

**Description:** initial supply cycles to the vault canister

**Example:** 

```json
3000000000000
```

### <a name="cycles_indexer"></a>5.3. Property `Chainsight Event Indexer specification > cycles > indexer`

**Title:** indexer

|              |                  |
| ------------ | ---------------- |
| **Type**     | `object or null` |
| **Required** | No               |

**Description:** cycles setting of indexer canister

| Property                                                      | Pattern | Type           | Deprecated | Definition | Title/Description   |
| ------------------------------------------------------------- | ------- | -------------- | ---------- | ---------- | ------------------- |
| - [initial_supply](#cycles_indexer_initial_supply )           | No      | number or null | No         | -          | initial_supply      |
| - [refueling_threshold](#cycles_indexer_refueling_threshold ) | No      | number or null | No         | -          | refueling_threshold |
| - [refueling_amount](#cycles_indexer_refueling_amount )       | No      | number or null | No         | -          | refueling_amount    |

#### <a name="cycles_indexer_initial_supply"></a>5.3.1. Property `Chainsight Event Indexer specification > cycles > indexer > initial_supply`

**Title:** initial_supply

|              |                  |
| ------------ | ---------------- |
| **Type**     | `number or null` |
| **Required** | No               |

**Description:** initial supply cycles to the indexer canister

**Example:** 

```json
1000000000000
```

#### <a name="cycles_indexer_refueling_threshold"></a>5.3.2. Property `Chainsight Event Indexer specification > cycles > indexer > refueling_threshold`

**Title:** refueling_threshold

|              |                  |
| ------------ | ---------------- |
| **Type**     | `number or null` |
| **Required** | No               |

**Description:** vault refuel cycles to the canister when the cycles balance is below this value

**Example:** 

```json
500000000000
```

#### <a name="cycles_indexer_refueling_amount"></a>5.3.3. Property `Chainsight Event Indexer specification > cycles > indexer > refueling_amount`

**Title:** refueling_amount

|              |                  |
| ------------ | ---------------- |
| **Type**     | `number or null` |
| **Required** | No               |

**Description:** vault refuel cycles with this value to the canister when the cycles balance is below the refueling_threshold

**Example:** 

```json
1000000000000
```

### <a name="cycles_db"></a>5.4. Property `Chainsight Event Indexer specification > cycles > db`

**Title:** db

|              |                  |
| ------------ | ---------------- |
| **Type**     | `object or null` |
| **Required** | No               |

**Description:** cycles setting of db canister

| Property                                                 | Pattern | Type           | Deprecated | Definition | Title/Description   |
| -------------------------------------------------------- | ------- | -------------- | ---------- | ---------- | ------------------- |
| - [initial_supply](#cycles_db_initial_supply )           | No      | number or null | No         | -          | initial_supply      |
| - [refueling_threshold](#cycles_db_refueling_threshold ) | No      | number or null | No         | -          | refueling_threshold |
| - [refueling_amount](#cycles_db_refueling_amount )       | No      | number or null | No         | -          | refueling_amount    |

#### <a name="cycles_db_initial_supply"></a>5.4.1. Property `Chainsight Event Indexer specification > cycles > db > initial_supply`

**Title:** initial_supply

|              |                  |
| ------------ | ---------------- |
| **Type**     | `number or null` |
| **Required** | No               |

**Description:** initial supply cycles to the db canister

**Example:** 

```json
150000000000
```

#### <a name="cycles_db_refueling_threshold"></a>5.4.2. Property `Chainsight Event Indexer specification > cycles > db > refueling_threshold`

**Title:** refueling_threshold

|              |                  |
| ------------ | ---------------- |
| **Type**     | `number or null` |
| **Required** | No               |

**Description:** vault refuel cycles to the canister when the cycles balance is below this value

**Example:** 

```json
1000000000000
```

#### <a name="cycles_db_refueling_amount"></a>5.4.3. Property `Chainsight Event Indexer specification > cycles > db > refueling_amount`

**Title:** refueling_amount

|              |                  |
| ------------ | ---------------- |
| **Type**     | `number or null` |
| **Required** | No               |

**Description:** vault refuel cycles with this value to the canister when the cycles balance is below the refueling_threshold

**Example:** 

```json
1000000000000
```

### <a name="cycles_proxy"></a>5.5. Property `Chainsight Event Indexer specification > cycles > proxy`

**Title:** proxy

|              |                  |
| ------------ | ---------------- |
| **Type**     | `object or null` |
| **Required** | No               |

**Description:** cycles setting of proxy canister

| Property                                                    | Pattern | Type           | Deprecated | Definition | Title/Description   |
| ----------------------------------------------------------- | ------- | -------------- | ---------- | ---------- | ------------------- |
| - [initial_supply](#cycles_proxy_initial_supply )           | No      | number or null | No         | -          | initial_supply      |
| - [refueling_threshold](#cycles_proxy_refueling_threshold ) | No      | number or null | No         | -          | refueling_threshold |
| - [refueling_amount](#cycles_proxy_refueling_amount )       | No      | number or null | No         | -          | refueling_amount    |

#### <a name="cycles_proxy_initial_supply"></a>5.5.1. Property `Chainsight Event Indexer specification > cycles > proxy > initial_supply`

**Title:** initial_supply

|              |                  |
| ------------ | ---------------- |
| **Type**     | `number or null` |
| **Required** | No               |

**Description:** initial supply cycles to the proxy canister

**Example:** 

```json
300000000000
```

#### <a name="cycles_proxy_refueling_threshold"></a>5.5.2. Property `Chainsight Event Indexer specification > cycles > proxy > refueling_threshold`

**Title:** refueling_threshold

|              |                  |
| ------------ | ---------------- |
| **Type**     | `number or null` |
| **Required** | No               |

**Description:** vault refuel cycles to the canister when the cycles balance is below this value

**Example:** 

```json
100000000000
```

#### <a name="cycles_proxy_refueling_amount"></a>5.5.3. Property `Chainsight Event Indexer specification > cycles > proxy > refueling_amount`

**Title:** refueling_amount

|              |                  |
| ------------ | ---------------- |
| **Type**     | `number or null` |
| **Required** | No               |

**Description:** vault refuel cycles with this value to the canister when the cycles balance is below the refueling_threshold

**Example:** 

```json
1000000000000
```

----------------------------------------------------------------------------------------------------------------------------
Generated using [json-schema-for-humans](https://github.com/coveooss/json-schema-for-humans) on 2024-07-18 at 02:25:10 +0000
