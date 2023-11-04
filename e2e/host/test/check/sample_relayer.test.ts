import {describe, test} from 'vitest';
import {loadCandidIds} from './utils';
import {assertMetadata, assertMetric} from './assertions';

const CANISTER_NAME = 'sample_relayer';

describe('readState', () => {
  test('.metadata', async () => {
    const ids = await loadCandidIds();
    const id = ids[CANISTER_NAME].local;

    await assertMetadata(id, {
      'chainsight:label': 'Sample Relayer',
      'chainsight:component_type': 'relayer',
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
