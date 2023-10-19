#!/usr/bin/env bash
cp -rp artifacts ws_dfx
cd ws_dfx

# dfx
dfx start --background --clean
dfx canister create --all
dfx build
dfx canister install --all
## check dashboard url (dfx deploy)
dfx deploy
## collect deliverables from deployments
mkdir -p ../outputs/.dfx
cp -rp .env ../outputs
cp -rp .dfx/local ../outputs/.dfx

# hardhat
cd ..
yarn hardhat node --port 18545
