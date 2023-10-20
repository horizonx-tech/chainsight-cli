import {describe, test} from 'vitest';
import {loadCandidIds} from './utils';
import {assertMetadata} from './assertions';

const CANISTER_NAME = 'sample_snapshot_indexer_https';

// describe('common', () => {
//   test(
//     '.metric',
//     async () => {
//       const ids = await loadCandidIds();
//       const id = ids[CANISTER_NAME].local;
//       await assertMetric(id, NODE_URL);
//     },
//     {timeout: 10000}
//   );
// });

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
