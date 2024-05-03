#!/bin/bash

set -e

balance_of() {
    dfx canister call --output=json icrc1_index icrc1_balance_of \
        "(record{owner = principal \"$1\"; })"
}

approve_allowance() {
    dfx canister call --output=json icrc1_ledger icrc2_approve \
        "(record { amount = $1; spender = record{owner = principal \"$2\";} })"
}

deposit() {
    dfx canister call --output=json bounty deposit "()"
}

CALLER=$(dfx identity get-principal)
BOUNTY=$(dfx canister id bounty)

# Call the bounty canister to deposit from caller and capture the output
echo "Calling deposit on bounty canister..."

# check initial balances
echo "Caller initial balance: $(balance_of $CALLER)"
echo "Bounty initial balance: $(balance_of $BOUNTY)"

# deposit
echo "Bounty allowance: $(approve_allowance 100_000 $BOUNTY)"
echo "Bounty deposit: $(deposit)"

# check final balances
sleep 1
echo "Caller final balance: $(balance_of $CALLER)"
echo "Bounty final balance: $(balance_of $BOUNTY)"

echo "PASS"
