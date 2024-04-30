#!/bin/bash

set -e

dfx canister install icrc1_index --mode reinstall --yes --argument \
    "(opt variant{Init = record {ledger_id = principal \"mxzaz-hqaaa-aaaar-qaada-cai\"}})"
