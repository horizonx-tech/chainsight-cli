# chainsight-cli

## How to use

Build CLI

```bash
cargo build
target/release/chainsight-cli
```

Create ChainSight project

```bash
target/release/chainsight-cli new sample
target/release/chainsight-cli build --path sample
# If you want to delete
target/release/chainsight-cli remove --path sample
```

Temp: Build & Deploy to local

```bash
# prerequisite
# cd sample_pj/artifacts && dfx start --host 127.0.0.1:49430 --clean
target/release/chainsight-cli test --path sample/artifacts --port 49430
# If you want to check candid ui url
# cd sample_pj/artifacts && dfx deploy
```
