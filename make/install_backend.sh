#!/bin/bash

set -e

DEPLOY_ID=$(dfx identity get-principal)
BOUNTY_ISSUE=1218

dfx canister install backend --mode reinstall --yes --argument \
    "(opt record{custodians=opt vec{principal \"${DEPLOY_ID}\"}})"