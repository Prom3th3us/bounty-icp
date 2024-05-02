#!/bin/bash

set -e

ICRC1_LEDGER=$(dfx canister id icrc1_ledger)

dfx canister install icrc1_index --mode reinstall --yes --argument \
    "(opt variant{Init = record {ledger_id = principal \"${ICRC1_LEDGER}\"}})"
