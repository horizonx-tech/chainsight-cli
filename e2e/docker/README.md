# docker for e2e

```bash
docker build -t e2e_node . --progress=plain
# Place dfx.json, wasm module, .did in /inputs
# ex: cp -rp ../../test__e2e_template/artifacts/* .inputs
# todo: update dfx.json to add `networks` field (host is 0.0.0.0:14943)
docker run -t \
  --name e2e_node \
  -p 18545:18545 -p 14943:14943 \
  -v $PWD/.inputs:/workspace/artifacts \
  -v $PWD/.outputs:/workspace/outputs \
  --rm e2e_node
```
