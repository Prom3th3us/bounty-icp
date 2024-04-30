#!/bin/bash

set -e

DEPLOY_ID=$(dfx identity get-principal)

dfx canister install icrc1_index --mode reinstall --yes --argument \
    "(record { ledger_id = principal \"${DEPLOY_ID}\" })"
