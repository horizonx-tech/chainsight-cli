import * as fs from 'fs';
import * as path from 'path';
import {expect} from 'vitest';
import {Actor, HttpAgent} from '@dfinity/agent';
import {IDL} from '@dfinity/candid';
import {Principal} from '@dfinity/principal';

export const NODE_URL = 'http://localhost:14943';

export type CandidIdsType = {[key: string]: {local: string}};

export const loadCandidIds = async (): Promise<CandidIdsType> => {
  const filepath = path.join(
    __dirname,
    '../../../docker/.outputs/.dfx/local/canister_ids.json'
  );
  const raw = fs.readFileSync(filepath, 'utf8');
  return JSON.parse(raw);
};

export const assertMetric = async (canister_id: string, host: string) => {
  const agent = new HttpAgent({host});
  const ident = Principal.fromText(canister_id);

  const idl: IDL.InterfaceFactory = ({IDL}) => {
    return IDL.Service({
      metric: IDL.Func(
        [],
        [IDL.Record({cycles: IDL.Nat, timestamp: IDL.Nat64})],
        ['query']
      ),
    });
  };

  const actor = Actor.createActor(idl, {canisterId: ident, agent});
  const res = (await actor.metric()) as {
    cycles: IDL.NatClass;
    timestamp: IDL.FixedIntClass;
  };
  expect(res.cycles).toBeGreaterThan(0);
  expect(res.timestamp).toBeGreaterThan(0);
};
