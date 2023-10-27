#!/usr/bin/env bash
dfx stop 

dfx start --background --clean
SPENDER="ztbm7-6u6jt-hgprr-7qx56-p36ca-a2qvx-y5767-4dxuc-xllss-ebtpt-sae"
YOU=$(dfx identity get-principal)

echo "* register profile"
dfx canister call ic_portal_canisters "jack"

echo "* get profile" 
dfx canister call ic_portal_canisters get_profile

echo ""