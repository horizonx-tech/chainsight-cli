#!/usr/bin/env bash
if [ -d "./artifacts" ]; then
    cp -rp artifacts ws_dfx
else
    mkdir ws_dfx
fi
cd ws_dfx

# dfx
dfx start --background --clean
if [ -f "./dfx.json" ]; then
    dfx canister create --all
    dfx build
    dfx canister install --all
    ## check dashboard url (dfx deploy)
    dfx deploy
fi
## collect deliverables from deployments
mkdir -p ../outputs/.dfx
cp -rp .env ../outputs
cp -rp .dfx/local ../outputs/.dfx

# hardhat
cd ..
yarn hardhat node --port 18545
