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
    dfx canister call --output=json backend deposit "()"
}

CALLER=$(dfx identity get-principal)
BACKEND=$(dfx canister id backend)

# Call the backend canister to deposit from caller and capture the output
echo "Calling deposit on backend canister..."

# check initial balances
echo "Caller initial balance: $(balance_of $CALLER)"
echo "Backend initial balance: $(balance_of $BACKEND)"

# deposit
echo "Backend allowance: $(approve_allowance 100_000 $BACKEND)"
echo "Backend deposit: $(deposit)"

# check final balances
sleep 1
echo "Caller final balance: $(balance_of $CALLER)"
echo "Backend final balance: $(balance_of $BACKEND)"

echo "PASS"
