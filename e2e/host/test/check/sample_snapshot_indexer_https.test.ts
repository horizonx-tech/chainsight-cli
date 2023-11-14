import {describe, test} from 'vitest';
import {loadCandidIds} from './utils';
import {assertMetadata, assertRespondable} from './assertions';

const CANISTER_NAME = 'sample_snapshot_indexer_https';

describe('readState', () => {
  test('.metadata', async () => {
    const ids = await loadCandidIds();
    const id = ids[CANISTER_NAME].local;

    await assertMetadata(id, {
      'chainsight:label': 'Sample Snapshot Indexer Https',
      'chainsight:component_type': 'snapshot_indexer_https',
    });
  });
});

describe.skip('query', () => {
  test(
    'respondable',
    async () => {
      const ids = await loadCandidIds();
      const id = ids[CANISTER_NAME].local;
      await assertRespondable(id);
    },
    {timeout: 10000}
  );
});
