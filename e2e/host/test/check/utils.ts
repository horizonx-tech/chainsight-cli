import * as fs from 'fs';
import * as path from 'path';
import {CanisterStatus, HttpAgent} from '@dfinity/agent';
import {Principal} from '@dfinity/principal';
import {MetaData} from '@dfinity/agent/lib/cjs/canisterStatus';

export type CandidIdsType = {[key: string]: {local: string}};

export const loadCandidIds = async (): Promise<CandidIdsType> => {
  const filepath = path.join(
    __dirname,
    '../../../docker/.outputs/.dfx/local/canister_ids.json'
  ); // TODO: calculate/consider path
  const raw = fs.readFileSync(filepath, 'utf8');
  return JSON.parse(raw);
};

export const getMetadata = async (canister_id: string, host: string) => {
  const agent = new HttpAgent({host});
  await agent.fetchRootKey();
  const ident = Principal.fromText(canister_id);

  return await CanisterStatus.request({
    agent,
    canisterId: ident,
    paths: [
      toMetadata('chainsight:label'),
      toMetadata('chainsight:component_type'),
    ],
  });
};

const toMetadata = (key: string): MetaData => ({
  kind: 'metadata',
  key,
  path: key,
  decodeStrategy: 'utf-8',
});
