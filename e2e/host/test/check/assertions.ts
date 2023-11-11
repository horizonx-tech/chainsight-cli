import {expect} from 'vitest';
import {Actor} from '@dfinity/agent';
import {IDL} from '@dfinity/candid';
import {Principal} from '@dfinity/principal';
import {getAgent, getMetadata} from './utils';

export const assertRespondable = async (canisterId: string) => {
  const agent = await getAgent();
  const ident = Principal.fromText(canisterId);

  const idl: IDL.InterfaceFactory = ({IDL}) => {
    return IDL.Service({
      get_proxy: IDL.Func([], [IDL.Principal], ['query']),
    });
  };

  const actor = Actor.createActor(idl, {canisterId: ident, agent});
  const res = await actor.get_proxy();
  expect(res).toBeTruthy();
};

export const assertMetadata = async (
  canisterId: string,
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  expected: {[key in string]: any}
) => {
  const result = await getMetadata(canisterId, Object.keys(expected));
  // eslint-disable-next-line node/no-unsupported-features/es-builtins
  expect(Object.fromEntries(result)).toStrictEqual(expected);
};
