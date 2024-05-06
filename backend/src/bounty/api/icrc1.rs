use serde::{Deserialize, Serialize};
use candid::{CandidType, Nat, Principal};
use ic_cdk::api::call::call;
use ic_ledger_types::{Memo, Subaccount, Timestamp};

type BlockIndex = Nat;

pub type Tokens = Nat;

#[derive(Debug, Serialize, Deserialize, CandidType, PartialEq)]
pub enum TransferFromError {
    BadFee { expected_fee: Tokens },
    BadBurn { min_burn_amount: Tokens },
    InsufficientFunds { balance: Tokens },
    InsufficientAllowance { allowance: Tokens },
    TooOld,
    CreatedInFuture { ledger_time: Timestamp },
    TemporarilyUnavailable,
    Duplicate { duplicate_of: BlockIndex },
    GenericError { error_code: Nat, message: String },
}

impl TransferFromError {
    pub fn to_string(&self) -> String {
        match self {
            TransferFromError::BadFee { expected_fee } => format!("Bad fee: {}", expected_fee),
            TransferFromError::BadBurn { min_burn_amount } => {
                format!("Bad burn: {}", min_burn_amount)
            }
            TransferFromError::InsufficientFunds { balance } => {
                format!("Insufficient funds: {}", balance)
            }
            TransferFromError::InsufficientAllowance { allowance } => {
                format!("Insufficient allowance: {}", allowance)
            }
            TransferFromError::TooOld => String::from("Transaction too old"),
            TransferFromError::CreatedInFuture { ledger_time } => format!(
                "Created in the future: {}",
                ledger_time.timestamp_nanos.to_string()
            ),
            TransferFromError::TemporarilyUnavailable => {
                String::from("Ledger temporarily unavailable")
            }
            TransferFromError::Duplicate { duplicate_of } => {
                format!("Duplicate of: {}", duplicate_of)
            }
            TransferFromError::GenericError {
                error_code,
                message,
            } => format!("Generic error code {}: {}", error_code, message),
        }
    }
}

pub type TransferFromResult = Result<BlockIndex, TransferFromError>;

#[derive(Debug, Serialize, Deserialize, CandidType)]
pub struct Account {
    pub owner: Principal,
    pub subaccount: Option<Subaccount>,
}

#[derive(Debug, Serialize, Deserialize, CandidType)]
pub struct TransferFromArgs {
    pub spender_subaccount: Option<Subaccount>,
    pub from: Account,
    pub to: Account,
    pub amount: Tokens,
    pub fee: Option<Tokens>,
    pub memo: Option<Memo>,
    pub created_at_time: Option<Timestamp>,
}

#[derive(Debug, Serialize, Deserialize, CandidType)]
pub struct AllowanceArgs {
    pub account: Account,
    pub spender: Account,
}

#[derive(Debug, Serialize, Deserialize, CandidType)]
pub struct Allowance {
    pub allowance: Nat,
    pub expires_at: Option<Timestamp>,
}

pub const MAINNET_ICRC1_LEDGER_CANISTER_ID: &str = "mxzaz-hqaaa-aaaar-qaada-cai";

pub struct ICRC1 {
    principal: Principal,
}

impl ICRC1 {
    pub fn new(principal: Principal) -> Self {
        ICRC1 { principal }
    }

    pub async fn transfer_from(&self, args: TransferFromArgs) -> TransferFromResult {
        let call_result: Result<(TransferFromResult,), _> =
            call(self.principal, "icrc2_transfer_from", (args,)).await;

        return call_result.unwrap().0;
    }

    pub async fn allowance(&self, args: AllowanceArgs) -> Allowance {
        let call_result: Result<(Allowance,), _> =
            call(self.principal, "icrc2_allowance", (args,)).await;

        return call_result.unwrap().0;
    }

    pub async fn get_fee(&self) -> Nat {
        let call_result: Result<(Nat,), _> = call(self.principal, "icrc1_fee", ()).await;

        return call_result.unwrap().0;
    }
}
