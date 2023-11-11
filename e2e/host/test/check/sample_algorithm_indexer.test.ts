import {describe, test} from 'vitest';
import {loadCandidIds} from './utils';
import {assertMetadata, assertRespondable} from './assertions';

const CANISTER_NAME = 'sample_algorithm_indexer';

describe('readState', () => {
  test('.metadata', async () => {
    const ids = await loadCandidIds();
    const id = ids[CANISTER_NAME].local;

    await assertMetadata(id, {
      'chainsight:label': 'Sample Algorithm Indexer',
      'chainsight:component_type': 'algorithm_indexer',
    });
  });
});

describe('query', () => {
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
