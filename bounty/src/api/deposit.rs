use ic_cdk::api::{caller, id};

use super::*;

use icrc1::{ICRC1, MAINNET_ICRC1_LEDGER_CANISTER_ID};

use candid::{CandidType, Nat, Principal};

#[derive(CandidType)]
pub enum DepositErr {
    TransferFailure,
}

pub type DepositReceipt = Result<Nat, DepositErr>;

pub async fn deposit_impl() -> DepositReceipt {
    // FIXME check caller equals the owner who initialized the bounty.
    let caller = caller();
    let icrc1_ledger_canister_id =
        Principal::from_text(MAINNET_ICRC1_LEDGER_CANISTER_ID).unwrap();
    
    let amount = deposit_icrc1(caller, icrc1_ledger_canister_id).await?;
    return DepositReceipt::Ok(amount);
}

async fn deposit_icrc1(
    caller: Principal,
    icrc1_token_canister_id: Principal
) -> Result<Nat, DepositErr> {
    let icrc1_token = ICRC1::new(icrc1_token_canister_id);
    let icrc1_token_fee = icrc1_token.get_metadata().await.fee;

    // depends on:
    // dfx canister call icrc1_ledger_canister icrc2_approve "(record { amount = 100_000; spender = record{owner = principal \"SPENDER_PRINCIPAL\";} })"
    let allowance = icrc1_token.allowance(caller, id()).await;

    let available = allowance - icrc1_token_fee;

    icrc1_token
        .transfer_from(caller, id(), available.to_owned())
        .await
        .map_err(|_| DepositErr::TransferFailure)?;

    Ok(available)
}
