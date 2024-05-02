#!/bin/bash

set -e

CALLER=$(dfx identity get-principal)
BOUNTY="bd3sg-teaaa-aaaaa-qaaba-cai"

# Call the bounty canister to deposit direct from caller and capture the output
echo "Calling deposit on bounty canister..."

# check initial balances
echo "Caller initial balance:"
dfx canister call icrc1_index icrc1_balance_of "(record{owner = principal \"${CALLER}\"; })"
echo "Bounty initial balance:"
dfx canister call icrc1_index icrc1_balance_of "(record{owner = principal \"${BOUNTY}\"; })"

# deposit
dfx canister call icrc1_ledger icrc2_approve "(record { amount = 100_000; spender = record{owner = principal \"${BOUNTY}\";} })"
dfx canister call bounty deposit '()'

# check final balances
echo "Caller final balance:"
dfx canister call icrc1_index icrc1_balance_of "(record{owner = principal \"${CALLER}\"; })"
echo "Bounty final balance:"
dfx canister call icrc1_index icrc1_balance_of "(record{owner = principal \"${BOUNTY}\"; })"
