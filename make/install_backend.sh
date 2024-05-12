#!/bin/bash

set -e

# "rwbxt-jvr66-qvpbz-2kbh3-u226q-w6djk-b45cp-66ewo-tpvng-thbkh-wae"
# This is a principal, derived from a seed phrase of the word "test" 12 times.
# > check identity.js
DEPLOY_ID=$(dfx identity get-principal)
BOUNTY_ISSUE=1218

dfx canister install backend --mode reinstall --yes --argument \
    "(opt record{custodians=opt vec{principal \"${DEPLOY_ID}\"}})"