import {describe, test, expect} from 'vitest';
import {ethers} from 'ethers';
import {HttpAgent} from '@dfinity/agent';
import {DFX_URL, HARDHAT_URL} from '../../src';

test('setup test tool', () => {
  expect('ping').toBe('ping');
});

describe('ping', () => {
  test('local evm node', async () => {
    const provider = new ethers.JsonRpcProvider(HARDHAT_URL);
    expect(await provider.send('net_version', [])).toEqual('31337');
    // console.log(await provider.send('eth_chainId', []));
    // console.log(await provider.send('web3_clientVersion', []));
    // console.log(await provider.send('net_version', []));
  });

  test('dfx node', async () => {
    const agent = new HttpAgent({host: DFX_URL});
    const status = await agent.status();
    expect(status['ic_api_version']).toEqual('0.18.0');
    expect(status['impl_version']).toEqual('0.8.0');
    expect(status['replica_health_status']).toEqual('healthy');
  });
});
