#!/bin/bash

set -e

DEPLOY_ID=$(dfx identity get-principal)
BOUNTY_ISSUE=1218

dfx canister install bounty --mode reinstall --yes --argument \
    "(principal \"${DEPLOY_ID}\", ${BOUNTY_ISSUE} : int32)"
