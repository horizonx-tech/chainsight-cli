import {describe, test} from 'vitest';
import {loadCandidIds} from './utils';
import {assertMetadata, assertMetric} from './assertions';

const CANISTER_NAME = 'sample_snapshot_indexer_evm';

describe('readState', () => {
  test('.metadata', async () => {
    const ids = await loadCandidIds();
    const id = ids[CANISTER_NAME].local;

    await assertMetadata(id, {
      'chainsight:label': 'Sample Snapshot Indexer Evm',
      'chainsight:component_type': 'snapshot_indexer_evm',
    });
  });
});

describe('query', () => {
  test(
    '.metric',
    async () => {
      const ids = await loadCandidIds();
      const id = ids[CANISTER_NAME].local;
      await assertMetric(id);
    },
    {timeout: 10000}
  );
});
