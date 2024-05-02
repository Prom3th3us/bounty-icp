use ic_cdk::api::{caller, id};
use num_traits::ToPrimitive;

use super::*;

use icrc1::{ICRC1, ICRC1_FEE, MAINNET_ICRC1_LEDGER_CANISTER_ID, AllowanceArgs, Account, TransferFromArgs, TransferArg};

use candid::{CandidType, Nat, Principal};

use ic_ledger_types::{Timestamp, Tokens};
use std::convert::From;

use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, CandidType, PartialEq)]
pub enum DepositErr {
    TransferFailure { reason: String },
}

pub type DepositReceipt = Result<Tokens, DepositErr>;

pub async fn deposit_impl() -> DepositReceipt {
    // FIXME check caller equals the owner who initialized the bounty.
    let caller = caller();
    let icrc1_token_canister_id =
        Principal::from_text(MAINNET_ICRC1_LEDGER_CANISTER_ID)
            .expect("Wrong MAINNET_ICRC1_LEDGER_CANISTER_ID");
    
    return deposit_icrc1(caller, icrc1_token_canister_id).await;
}

async fn deposit_icrc1(
    caller: Principal,
    icrc1_token_canister_id: Principal
) -> Result<Tokens, DepositErr> {
    let icrc1_token = ICRC1::new(icrc1_token_canister_id);
    let icrc1_token_fee = icrc1_token.get_fee().await;

    let bounty_canister_id = id();

    // depends on:
    // > dfx canister call icrc1_ledger_canister icrc2_approve "(record { amount = 100_000; spender = record{owner = principal \"SPENDER_PRINCIPAL\";} })"
    let allowance_args =
        AllowanceArgs {
            account: Account { owner: caller, subaccount : None },
            spender : Account { owner: id(), subaccount : None }
        };
    let allowance = icrc1_token.allowance(allowance_args).await;

    let available = 
        ToPrimitive::to_u64(&allowance.allowance.0)
            .expect("Nat value is too large for u64") 
        - Tokens::e8s(&icrc1_token_fee);
    
    let nanoseconds_since_epoch =
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_nanos() as u64;
        
    let transfer_from_args =
        TransferFromArgs{
            // TODO check or FIXME
            spender_subaccount: Some(From::from(caller)),
            from: Account { 
                owner: caller,
                subaccount : Some(From::from(caller))
            },
            to: Account {
                owner : bounty_canister_id, 
                subaccount : Some(From::from(bounty_canister_id))
            },
            amount: Tokens::from_e8s(available),
            fee: Some(Tokens::from_e8s(ICRC1_FEE)),
            // TODO enhance memo text
            memo:  Some(String::from("deposit_icrc1").as_bytes().to_vec()),
            created_at_time: Some(Timestamp { timestamp_nanos: nanoseconds_since_epoch})
        };
    
    return icrc1_token
        .transfer_from(transfer_from_args)
        .await
        .map(|_| Tokens::from_e8s(available))
        .map_err(|error| 
            DepositErr::TransferFailure{reason: error.to_string()}
        );
}

pub async fn deposit_direct_impl(amount: Nat) -> DepositReceipt {
    // FIXME check caller equals the owner who initialized the bounty.
    let caller = caller();
    let icrc1_ledger_canister_id =
        Principal::from_text(MAINNET_ICRC1_LEDGER_CANISTER_ID)
            .expect("Wrong MAINNET_ICRC1_LEDGER_CANISTER_ID");

    return deposit_direct_icrc1(caller, icrc1_ledger_canister_id, amount).await;
}

async fn deposit_direct_icrc1(
    caller: Principal,
    icrc1_token_canister_id: Principal,
    amount: Nat
) -> Result<Tokens, DepositErr> {
    let icrc1_token = ICRC1::new(icrc1_token_canister_id);
    let icrc1_token_fee = icrc1_token.get_fee().await;

    let bounty_canister_id = id();

    let available = 
        ToPrimitive::to_u64(&amount.0)
            .expect("Nat value is too large for u64") 
        - Tokens::e8s(&icrc1_token_fee);
    
    let nanoseconds_since_epoch =
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_nanos() as u64;

    let transfer_args =
        TransferArg{
            // TODO check or FIXME
            from_subaccount: Some(From::from(caller)),
            to: Account {
                owner : bounty_canister_id, 
                subaccount : Some(From::from(bounty_canister_id))
            },
            amount: Tokens::from_e8s(available),
            fee: Some(Tokens::from_e8s(ICRC1_FEE)),
            // TODO enhance memo text
            memo:  Some(String::from("deposit_icrc1").as_bytes().to_vec()),
            created_at_time: Some(Timestamp { timestamp_nanos: nanoseconds_since_epoch})
        };
    
    return icrc1_token
        .transfer(transfer_args)
        .await
        .map(|_| Tokens::from_e8s(available))
        .map_err(|error| 
            DepositErr::TransferFailure{reason: error.to_string()}
        );
}
