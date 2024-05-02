use super::*;
use std::convert::From;
use serde::{Deserialize, Serialize};
use candid::{CandidType, Principal};
use ic_cdk::api::{caller, id};
use ic_ledger_types::Memo;
use icrc1::{
    Account, AllowanceArgs, Tokens, TransferArg, TransferFromArgs, ICRC1,
    MAINNET_ICRC1_LEDGER_CANISTER_ID,
};

#[derive(Debug, Serialize, Deserialize, CandidType, PartialEq)]
pub enum DepositErr {
    TransferFailure { reason: String },
}

pub type DepositReceipt = Result<Tokens, DepositErr>;

pub async fn deposit_impl() -> DepositReceipt {
    // FIXME check caller equals the owner who initialized the bounty.
    let caller = caller();
    let icrc1_token_canister_id = Principal::from_text(MAINNET_ICRC1_LEDGER_CANISTER_ID)
        .expect("Wrong MAINNET_ICRC1_LEDGER_CANISTER_ID");

    return deposit_icrc1(caller, icrc1_token_canister_id).await;
}

async fn deposit_icrc1(
    caller: Principal,
    icrc1_token_canister_id: Principal,
) -> Result<Tokens, DepositErr> {
    let icrc1_token = ICRC1::new(icrc1_token_canister_id);
    let icrc1_token_fee = icrc1_token.get_fee().await;

    let bounty_canister_id = id();

    let allowance_args = AllowanceArgs {
        account: Account {
            owner: caller,
            subaccount: None,
        },
        spender: Account {
            owner: bounty_canister_id,
            subaccount: None,
        },
    };
    let allowance = icrc1_token.allowance(allowance_args).await;

    let available = allowance.allowance.clone() - icrc1_token_fee.clone();

    let transfer_from_args = TransferFromArgs {
        // TODO check or FIXME
        spender_subaccount: Some(From::from(caller)),
        from: Account {
            owner: caller,
            subaccount: Some(From::from(caller)),
        },
        to: Account {
            owner: bounty_canister_id,
            subaccount: Some(From::from(bounty_canister_id)),
        },
        amount: available.clone(),
        fee: Some(icrc1_token_fee),
        // TODO enhance memo text
        memo: Some(Memo(0)),
        created_at_time: None,
    };

    return icrc1_token
        .transfer_from(transfer_from_args)
        .await
        .map(|_| available)
        .map_err(|error| DepositErr::TransferFailure {
            reason: error.to_string(),
        });
}

pub async fn deposit_direct_impl(amount: u64) -> DepositReceipt {
    // FIXME check caller equals the owner who initialized the bounty.
    let caller = caller();
    let icrc1_ledger_canister_id = Principal::from_text(MAINNET_ICRC1_LEDGER_CANISTER_ID)
        .expect("Wrong MAINNET_ICRC1_LEDGER_CANISTER_ID");

    return deposit_direct_icrc1(caller, icrc1_ledger_canister_id, amount).await;
}

async fn deposit_direct_icrc1(
    caller: Principal,
    icrc1_token_canister_id: Principal,
    amount: u64,
) -> Result<Tokens, DepositErr> {
    let icrc1_token = ICRC1::new(icrc1_token_canister_id);
    let icrc1_token_fee = icrc1_token.get_fee().await;

    let bounty_canister_id = id();

    let available = amount.clone() - icrc1_token_fee.clone();

    let transfer_args = TransferArg {
        // TODO check or FIXME
        from_subaccount: Some(From::from(caller)),
        to: Account {
            owner: bounty_canister_id,
            subaccount: Some(From::from(bounty_canister_id)),
        },
        amount: available.clone(),
        fee: Some(icrc1_token_fee),
        // TODO enhance memo text
        memo: Some(Memo(1)),
        created_at_time: None,
    };

    return icrc1_token
        .transfer(transfer_args)
        .await
        .map(|_| available)
        .map_err(|error| DepositErr::TransferFailure {
            reason: error.to_string(),
        });
}
