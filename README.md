# Chainsight command-line execution envirionment

## How to use

Build CLI

```bash
cargo build
target/release/csx
```

Create Chainsight project

```bash
target/release/csx new sample
target/release/csx build --path sample
# If you want to delete
target/release/csx remove --path sample
```

Temp: Build & Deploy to local

```bash
# prerequisite
# cd sample_pj/artifacts && dfx start --host 127.0.0.1:49430 --clean
target/release/csx test --path sample/artifacts --port 49430
# If you want to check candid ui url
# cd sample_pj/artifacts && dfx deploy
```
