import {describe, test} from 'vitest';
import {NODE_URL, assertMetric, loadCandidIds} from './common';

const CANISTER_NAME = 'sample_snapshot_indexer_https';

describe('common', () => {
  test(
    '.metric',
    async () => {
      const ids = await loadCandidIds();
      const id = ids[CANISTER_NAME].local;
      await assertMetric(id, NODE_URL);
    },
    {timeout: 10000}
  );
});
