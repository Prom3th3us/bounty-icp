use super::*;
use candid::{CandidType, Principal};
use ic_cdk::api::{caller, id};
use ic_ledger_types::{Memo, DEFAULT_SUBACCOUNT};
use icrc1::{
    Account, AllowanceArgs, Tokens, TransferFromArgs, ICRC1, MAINNET_ICRC1_LEDGER_CANISTER_ID,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, CandidType, PartialEq)]
pub enum DepositErr {
    TransferFailure { reason: String },
}

pub type DepositReceipt = Result<Tokens, DepositErr>;

pub async fn deposit_impl() -> DepositReceipt {
    // FIXME: check caller equals the owner who initialized the bounty.
    let caller = caller();
    let icrc1_token_canister_id = Principal::from_text(MAINNET_ICRC1_LEDGER_CANISTER_ID)
        .expect("Wrong MAINNET_ICRC1_LEDGER_CANISTER_ID");

    deposit_icrc1(caller, icrc1_token_canister_id).await
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
        spender_subaccount: Some(DEFAULT_SUBACCOUNT),
        from: Account {
            owner: caller,
            subaccount: None,
        },
        to: Account {
            owner: bounty_canister_id,
            subaccount: None,
        },
        amount: available.clone(),
        fee: Some(icrc1_token_fee),
        // TODO: enhance memo text
        memo: Some(Memo(0)),
        created_at_time: None,
    };

    icrc1_token
        .transfer_from(transfer_from_args)
        .await
        .map(|_| available)
        .map_err(|error| DepositErr::TransferFailure {
            reason: error.as_string(),
        })
}
