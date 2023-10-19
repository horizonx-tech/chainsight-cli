# docker for e2e

```bash
docker build -t e2e_node . --progress=plain
# Place dfx.json, wasm module, .did in /inputs
# ex: cp -rp ../../test__e2e_template/artifacts/* .inputs
# NOTE: for docker, need network fields in dfx.json
#       ref: 'exec:add-networks-to-dfx-json' task in host
docker run -t \
  --name e2e_node \
  -p 18545:18545 -p 14943:14943 \
  -v $PWD/.inputs:/workspace/artifacts \
  -v $PWD/.outputs:/workspace/outputs \
  --rm e2e_node
```
