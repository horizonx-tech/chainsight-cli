import * as fs from 'fs';
import * as path from 'path';
import {CanisterStatus, HttpAgent} from '@dfinity/agent';
import {Principal} from '@dfinity/principal';
import {MetaData} from '@dfinity/agent/lib/cjs/canisterStatus';
import {DFX_URL} from '../../src';

export const getAgent = async () => {
  const agent = new HttpAgent({host: DFX_URL});
  await agent.fetchRootKey();
  return agent;
};

export type CandidIdsType = {[key: string]: {local: string}};
export const loadCandidIds = async (): Promise<CandidIdsType> => {
  const filepath = path.join(
    __dirname,
    '../../../docker/.outputs/.dfx/local/canister_ids.json'
  ); // TODO: calculate/consider path
  const raw = fs.readFileSync(filepath, 'utf8');
  return JSON.parse(raw);
};

export const getMetadata = async (canisterId: string, keys: string[]) => {
  const agent = await getAgent();
  const ident = Principal.fromText(canisterId);

  return await CanisterStatus.request({
    agent,
    canisterId: ident,
    paths: keys.map(toMetadata),
  });
};

const toMetadata = (key: string): MetaData => ({
  kind: 'metadata',
  key,
  path: key,
  decodeStrategy: 'utf-8',
});
