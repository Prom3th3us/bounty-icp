---
keywords: [intermediate, rust, http, get, http get]
---

# HTTP: GET

[View this sample's code on GitHub](https://github.com/dfinity/examples/tree/master/rust/send_http_get)

The purpose of this dapp is to give developers a minimal dapp that uses the IC's HTTPS outcalls feature to make a `GET` request.

This demo goes in hand with the [developer documentation on HTTPS outcalls](https://internetcomputer.org/docs/current/developer-docs/integrations/https-outcalls/https-outcalls-get).

If you want to start working on your project right away, you might want to try the following commands:

```bash
dfx help
dfx canister --help
```

## Running the project locally
If you want to test your project locally, you can use the following commands:

```bash
# Starts the replica, running in the background
dfx start --background

# Deploys your canisters to the replica and generates your candid interface
dfx deploy
```
# Install
1/ dfx start
2/ dfx deploy -> creates .env
3/ define GITHUB_TOKEN in .env
4/ cargo build --target wasm32-unknown-unknown --release
5/ cargo install cargo-audit --locked
6/ make test-1
7/ make test-2

# to restart your local network
dfx start --clean

## other useful dfx commands
dfx identity list
    anonymous
    default *
dfx identity whoami
    default
dfx identity get-principal
    viost-xztac-z5ppy-xg3l4-rcpbr-622pc-pvluy-f4lq4-jwrgo-nusmt-cae
dfx identity get-wallet
    bnz7o-iuaaa-aaaaa-qaaaa-cai
dfx canister info $(dfx identity get-wallet)
dfx canister status $(dfx identity get-wallet)

## ICRC-1 endpoints
> To fetch the balance of an account
dfx canister call icrc1_ledger icrc1_balance_of "(record {owner = principal \"$(dfx identity get-principal)\"; })"
> To transfer
dfx canister call icrc1_ledger icrc1_transfer '(record { to = record { owner = principal "hdq6b-ncywm-yajd5-4inc6-hgpzp-55xnp-py7d5-uqt6o-cv5c6-rrhwa-zqe";};  amount = 100_000:nat;})'
> To fetch the balance of receiving account
dfx canister call icrc1_ledger icrc1_balance_of "(record {owner = principal \"hdq6b-ncywm-yajd5-4inc6-hgpzp-55xnp-py7d5-uqt6o-cv5c6-rrhwa-zqe\"; })"
> others
dfx canister call icrc1_ledger get_blocks '(record{start=0:nat;length=2:nat})'
dfx canister call icrc1_ledger get_transactions '(record{start=0:nat;length=5:nat})'

## ICRC-2 endpoints
> To approve tokens to a certain spender
dfx canister call icrc1_ledger_canister icrc2_approve "(record { amount = 100_000; spender = record{owner = principal \"${EXAMPLE}\";} })"
> To check the allowance of the spender
dfx canister call icrc1_ledger_canister icrc2_allowance "(record { account = record{owner = principal "${DEFAULT}";}; spender = record{owner = principal "${EXAMPLE}";} })"

## index endpoints
> dfx canister call icrc1_index status '()'
> dfx canister call icrc1_index get_blocks '(record{start=0:nat;length=2:nat})'
> dfx canister call icrc1_index get_account_transactions "(record{account=record {owner = principal \"$(dfx identity get-principal)\"; }; max_results=2:nat})"
> dfx canister call icrc1_index icrc1_balance_of "(record{owner = principal \"$(dfx identity get-principal)\"; })"
> dfx canister call icrc1_index icrc1_balance_of "(record{owner = principal \"hdq6b-ncywm-yajd5-4inc6-hgpzp-55xnp-py7d5-uqt6o-cv5c6-rrhwa-zqe\"; })"




Description
marcelomanuelbaraschi
2 minutes ago (edited)
Memory management:

add register issue endpoint (should return error when issue not found or when max limit of 100.000 exceeded)
check max limit of 10 exceed when accepting pr attempts for issue
no hacer checks por caller todavia (eso es otro ticket)
agregar endpoints de unregister issue o attempt (may fail with not found)
tanto los endpoints de accept como de unregister deben ser idempotentes (agregar 2 veces la misma entrada debe retornar success)
expirar items en mapas (issue y pre) cada vez q se agrega (accept) o elimina (unregister) una entrada
