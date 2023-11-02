import {describe, test} from 'vitest';
import {loadCandidIds} from './utils';
import {assertMetadata} from './assertions';

const CANISTER_NAME = 'sample_algorithm_indexer';

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
      'chainsight:label': 'Sample Algorithm Indexer',
      'chainsight:component_type': 'algorithm_indexer',
    });
  });
});
