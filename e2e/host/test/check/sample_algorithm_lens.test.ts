import {describe, test} from 'vitest';
import {loadCandidIds} from './utils';
import {assertMetadata, assertMetric} from './assertions';

const CANISTER_NAME = 'sample_algorithm_lens';

describe('readState', () => {
  test('.metadata', async () => {
    const ids = await loadCandidIds();
    const id = ids[CANISTER_NAME].local;

    await assertMetadata(id, {
      'chainsight:label': 'Sample Algorithm Lens',
      'chainsight:component_type': 'algorithm_lens',
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
