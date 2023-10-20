import * as fs from 'fs';
import * as path from 'path';

interface Data {
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  [key: string]: any;
}

const NETWORKS_DATA = {
  local: {
    bind: '0.0.0.0:14943',
    type: 'ephemeral',
  },
};

const main = async () => {
  const filepath = path.join(__dirname, '../../docker/.inputs/dfx.json'); // TODO: calculate/consider path
  const data: Data = JSON.parse(fs.readFileSync(filepath, 'utf8'));
  data.networks = NETWORKS_DATA;
  const updatedData = JSON.stringify(data, null, 2);
  fs.writeFileSync(filepath, updatedData);
};

main()
  .then(() => console.log('Done: add-networks-for-docker-to-dfx-json'))
  .catch(() => console.error('Failed: add-networks-for-docker-to-dfx-json'));
