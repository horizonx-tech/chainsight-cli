# README

## Top-level Schemas

*   [Chainsight Algorithm Lens specification](./algorithm_lens.md "Chainsight Algorithm Lens specification") – `https://raw.githubusercontent.com/horizonx-tech/chainsight-cli/main/resources/schema/algorithm_lens.json`

*   [Chainsight Event Indexer specification](./algorithm_indexer.md "Chainsight Algorithm Indexer specification") – `https://raw.githubusercontent.com/horizonx-tech/chainsight-cli/main/resources/schema/algorithm_indexer.json`

*   [Chainsight Event Indexer specification](./event_indexer.md "Chainsight Event Indexer specification") – `https://raw.githubusercontent.com/horizonx-tech/chainsight-cli/main/resources/schema/event_indexer.json`

*   [Chainsight HTTP Event Indexer specification](./snapshot_indexer_http.md "Chainsight HTTP Snapshot Indexer specification") – `https://raw.githubusercontent.com/horizonx-tech/chainsight-cli/main/resources/schema/snapshot_indexer_http.json`

*   [Chainsight Relayer specification](./relayer.md "Chainsight Relayer specification") – `https://raw.githubusercontent.com/horizonx-tech/chainsight-cli/main/resources/schema/relayer.json`

*   [Chainsight Snapshot Indexer specification](./snapshot_indexer.md "Chainsight Snapshot Indexer specification") – `https://raw.githubusercontent.com/horizonx-tech/chainsight-cli/main/resources/schema/snapshot_indexer.json`

## Other Schemas

### Objects

*   [HTTP request headers for the datasource](./snapshot_indexer_http-properties-datasource-properties-http-request-headers-for-the-datasource.md "HTTP request headers for the datasource") – `#/properties/datasource/properties/headers#/properties/datasource/properties/headers`

*   [Untitled object in Chainsight HTTP Event Indexer specification](./snapshot_indexer_http-properties-datasource.md) – `https://raw.githubusercontent.com/horizonx-tech/chainsight-cli/main/resources/schema/snapshot_indexer_http.json#/properties/datasource`

*   [datasource](./algorithm_indexer-properties-datasource.md) – `#/properties/datasource#/properties/datasource`

*   [destination](./relayer-properties-destination.md "destination evm network and contract for the data") – `#/properties/destination#/properties/destination`

*   [fields](./algorithm_indexer-properties-datasource-properties-input-properties-fields.md "field names and rust types of the struct") – `#/properties/datasource/properties/input/properties/fields#/properties/datasource/properties/input/properties/fields`

*   [fields](./algorithm_indexer-properties-output-output-struct-properties-fields.md "field names and rust types of the struct") – `#/properties/output/items/properties/fields#/properties/output/items/properties/fields`

*   [input](./algorithm_indexer-properties-datasource-properties-input.md "struct retrived from the source canister") – `#/properties/datasource/properties/input#/properties/datasource/properties/input`

*   [lens targets](./relayer-properties-lens-targets.md "targets for the lens") – `#/properties/lens_targets#/properties/lens_targets`

*   [metadata for the canister](./algorithm_indexer-properties-metadata-for-the-canister.md) – `#/properties/metadata#/properties/metadata`

*   [output struct](./algorithm_indexer-properties-output-output-struct.md) – `#/properties/output/items#/properties/output/items`

*   [query parameters for the datasource](./snapshot_indexer_http-properties-datasource-properties-query-parameters-for-the-datasource.md "query parameter names and values for the datasource") – `#/properties/datasource/properties/queries#/properties/datasource/properties/queries`

*   [storage](./snapshot_indexer-properties-storage.md "storage properties for the canister") – `#/properties/storage#/properties/storage`

### Arrays

*   [identifiers](./relayer-properties-lens-targets-properties-identifiers.md "canister ids of the lens targets") – `#/properties/lens_targets/properties/identifiers#/properties/lens_targets/properties/identifiers`

*   [output](./algorithm_indexer-properties-output.md "array of output struct name and fields") – `#/properties/output#/properties/output`

*   [tags for the canister](./algorithm_indexer-properties-metadata-for-the-canister-properties-tags-for-the-canister.md "Can be used to filter canisters in the UI") – `#/properties/metadata/properties/tags#/properties/metadata/properties/tags`

## Version Note

The schemas linked above follow the JSON Schema Spec version: `http://json-schema.org/draft-07/schema#`
