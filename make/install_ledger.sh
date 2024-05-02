#!/bin/bash

set -e

TOKEN_NAME="Local ckBTC"
TOKEN_SYMBOL=LckBTC
PRE_MINTED_TOKENS=10_000_000_000
TRANSFER_FEE=10_000
TRIGGER_THRESHOLD=2000
NUM_OF_BLOCK_TO_ARCHIVE=1000
CYCLE_FOR_ARCHIVE_CREATION=10000000000000
FEATURE_FLAGS=true

# FIXME: create propper identity
MINTER="t2y5w-qp34w-qixaj-s67wp-syrei-5yqse-xbed6-z5nsd-fszmf-izgt2-lqe"
dfx identity use default
DEFAULT=$(dfx identity get-principal)
# FIXME: create propper identity
ARCHIVE_CONTROLLER=$(dfx identity get-principal)

dfx canister install icrc1_ledger --mode reinstall --yes --argument "(variant { Init =
record {
    token_symbol = \"${TOKEN_SYMBOL}\";
    token_name = \"${TOKEN_NAME}\";
    minting_account = record { owner = principal \"${MINTER}\" };
    transfer_fee = ${TRANSFER_FEE};
    metadata = vec {};
    feature_flags = opt record{icrc2 = ${FEATURE_FLAGS}};
    initial_balances = vec { record { record { owner = principal \"${DEFAULT}\"; }; ${PRE_MINTED_TOKENS}; }; };
    archive_options = record {
        num_blocks_to_archive = ${NUM_OF_BLOCK_TO_ARCHIVE};
        trigger_threshold = ${TRIGGER_THRESHOLD};
        controller_id = principal \"${ARCHIVE_CONTROLLER}\";
        cycles_for_archive_creation = opt ${CYCLE_FOR_ARCHIVE_CREATION};
    };
 }
})"
