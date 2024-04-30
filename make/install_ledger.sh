#!/bin/bash

set -e

TOKEN_NAME=PROTK
TOKEN_SYMBOL=PRO
PRE_MINTED_TOKENS=10_000_000_000
TRANSFER_FEE=10_000
TRIGGER_THRESHOLD=2000
NUM_OF_BLOCK_TO_ARCHIVE=1000
CYCLE_FOR_ARCHIVE_CREATION=10000000000000
FEATURE_FLAGS=true

# FIXME: this should be used as minting_account owner
MINTER_ACCOUNT_ID=$(dfx ledger account-id)
DEPLOY_ID=$(dfx identity get-principal)
ARCHIVE_CONTROLLER=$(dfx identity get-principal)

dfx canister install icrc1_ledger --mode reinstall --yes --argument \
    " \
    (variant { \
        Init = record { \
            decimals = null; \
            token_symbol = \"${TOKEN_SYMBOL}\"; \
            transfer_fee = ${TRANSFER_FEE}; \
            metadata = vec {}; \
            minting_account = record { \
                owner = principal \"${DEPLOY_ID}\"; \
                subaccount = null; \
            }; \
            initial_balances = vec { record { record { owner = principal \"${DEPLOY_ID}\"; }; ${PRE_MINTED_TOKENS}; }; }; \
            fee_collector_account = null; \
            archive_options = record { \
                num_blocks_to_archive = ${NUM_OF_BLOCK_TO_ARCHIVE}; \
                trigger_threshold = ${TRIGGER_THRESHOLD}; \
                max_message_size_bytes = null; \
                cycles_for_archive_creation = opt ${CYCLE_FOR_ARCHIVE_CREATION}; \
                node_max_memory_size_bytes = null; \
                controller_id = principal \"${ARCHIVE_CONTROLLER}\"; \
            }; \
            max_memo_length = null; \
            token_name = \"${TOKEN_NAME}\"; \
            feature_flags = opt record{icrc2 = ${FEATURE_FLAGS}}; \
        } \
    }) \
    "
