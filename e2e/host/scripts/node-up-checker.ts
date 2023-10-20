import fetch, {FetchError} from 'node-fetch';
import {exit} from 'node:process';

const DFX_NODE_URL = 'http://localhost:14943';
const HARDHAT_NODE_URL = 'http://localhost:18545';

const MAX_RETIRES = 20;
const RETRY_INTERVAL = 15000;

const ping = async (url: string, expected_status: number) => {
  try {
    const res = await fetch(url);
    if (res.status !== expected_status) {
      throw new Error(`Expected status ${expected_status}, got ${res.status}`);
    }
    return true;
  } catch (e) {
    if (e instanceof FetchError && e.code === 'ECONNRESET') {
      return false;
    }
    throw e;
  }
};

const pingWithWait = async (url: string, expected_status: number) => {
  let retries = 0;
  while (retries < MAX_RETIRES) {
    const isConn = await ping(url, expected_status);
    if (isConn) return null;
    retries++;
    console.log(`Retrying ${retries}: ${url}`);
    await new Promise(resolve => setTimeout(resolve, RETRY_INTERVAL));
  }

  throw new Error(`Could not connect to ${url}`);
};

const main = async () => {
  await pingWithWait(DFX_NODE_URL, 400);
  await pingWithWait(HARDHAT_NODE_URL, 200);

  const [isConnDfx, isConnHardhat] = await Promise.all([
    ping(DFX_NODE_URL, 400),
    ping(HARDHAT_NODE_URL, 200),
  ]);

  if (!isConnDfx || !isConnHardhat) {
    throw new Error(
      `Could not connect to one of the nodes: dfx=${isConnDfx}, hardhat=${isConnHardhat}`
    );
  }
};

main().catch(e => {
  console.error(e);
  exit(1);
});
