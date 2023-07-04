# Chainsight command-line execution envirionment

## How to use

Build CLI & Check

```bash
cargo build --release
alias csx="<repository_path>/target/release/csx"
csx --version
# -> csx x.y.z
```

Example of Operation Flow

```bash
### Create Chainsight project
csx new sample

# (manual) modify project/component manifests

### Build project
csx build --path sample

### Deploy project
## If you deploy in local, dfx network must be started (ex: 'dfx start')
csx deploy --path sample/artifacts

### Initialize Components / Start processing
csx exec --path sample

### Remove project
target/release/csx remove --path sample
```
