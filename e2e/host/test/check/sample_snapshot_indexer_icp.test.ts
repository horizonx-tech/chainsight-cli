import {describe, test, expect} from 'vitest';
import {getMetadata, loadCandidIds} from './utils';
import {DFX_URL} from '../../src';

const CANISTER_NAME = 'sample_snapshot_indexer_icp';

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
    const res = await getMetadata(id, DFX_URL);

    // eslint-disable-next-line node/no-unsupported-features/es-builtins
    expect(Object.fromEntries(res)).toStrictEqual({
      'chainsight:label': 'Sample Snapshot Indexer Icp',
      'chainsight:component_type': 'snapshot_indexer_icp',
    });
  });
});
