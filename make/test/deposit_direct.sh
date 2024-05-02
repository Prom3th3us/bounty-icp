#!/bin/bash

set -e

CALLER=$(dfx identity get-principal)
BOUNTY="bd3sg-teaaa-aaaaa-qaaba-cai"

# Call the bounty canister to deposit direct from caller and capture the output
echo "Calling deposit_direct on bounty canister..."

# check initial balances
echo "Caller initial balance:"
dfx canister call icrc1_index icrc1_balance_of "(record{owner = principal \"${CALLER}\"; })"
echo "Bounty initial balance:"
dfx canister call icrc1_index icrc1_balance_of "(record{owner = principal \"${BOUNTY}\"; })"

# deposit direct
dfx canister call bounty deposit_direct '(100_000,)'

# check final balances
echo "Caller final balance:"
dfx canister call icrc1_index icrc1_balance_of "(record{owner = principal \"${CALLER}\"; })"
echo "Bounty final balance:"
dfx canister call icrc1_index icrc1_balance_of "(record{owner = principal \"${BOUNTY}\"; })"
