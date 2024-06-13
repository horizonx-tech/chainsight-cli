# Changelog

## 0.2.0

### Changed

- Make it independent of `dfx` binary [pr#238]
  - For the purpose of publication as a library
  - The following commands are executed without dfx binary
    - `csx deploy`, `csx exec`
    - Create your own `component_ids.json` instead of `canister_ids.json` by dfx to store canister ids.
  - If another process requires dfx (e.g. pre-check) and dfx is not present, skip that process.
  - Support for dfx development support functions without dfx for local development
    - Loading Principal, Wallet from local context
