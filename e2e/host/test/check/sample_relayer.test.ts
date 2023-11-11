import {describe, test} from 'vitest';
import {loadCandidIds} from './utils';
import {assertMetadata, assertRespondable} from './assertions';

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
    'respondable',
    async () => {
      const ids = await loadCandidIds();
      const id = ids[CANISTER_NAME].local;
      await assertRespondable(id);
    },
    {timeout: 10000}
  );
});
