use candid::{CandidType, Nat, Principal};
use serde::{Deserialize, Serialize};
use ic_ledger_types::{Timestamp, Subaccount, Tokens};
use ic_cdk::api::call::call;

type BlockIndex = Nat;

use std::fmt::{Display, Formatter};

#[derive(Debug, Serialize, Deserialize, CandidType, PartialEq)]
pub enum TransferError {
    BadFee { expected_fee : Tokens },
    BadBurn { min_burn_amount : Tokens },
    InsufficientFunds { balance : Tokens },
    TooOld,
    CreatedInFuture { ledger_time : Timestamp },
    TemporarilyUnavailable,
    Duplicate { duplicate_of : BlockIndex },
    GenericError { error_code : Nat,  message : String },
}

impl TransferError {
    pub fn to_string(&self) -> String {
        match self {
            TransferError::BadFee { expected_fee } =>
                format!("Bad fee: {}", expected_fee),
            TransferError::BadBurn { min_burn_amount } =>
                format!("Bad burn: {}", min_burn_amount),
            TransferError::InsufficientFunds { balance } =>
                format!("Insufficient funds: {}", balance),
            TransferError::TooOld =>
                String::from("Transaction too old"),
            TransferError::CreatedInFuture { ledger_time } =>
                format!("Created in the future: {}", ledger_time.timestamp_nanos.to_string()),
            TransferError::TemporarilyUnavailable =>
                String::from("Ledger temporarily unavailable"),
            TransferError::Duplicate { duplicate_of } => 
                format!("Duplicate of: {}", duplicate_of),
            TransferError::GenericError { error_code, message } =>
                format!("Generic error code {}: {}", error_code, message)
        }
    }
}

pub type TransferResult = Result<BlockIndex, TransferError>;

#[derive(Debug, Serialize, Deserialize, CandidType, PartialEq)]
pub enum TransferFromError {
    BadFee { expected_fee : Tokens },
    BadBurn { min_burn_amount : Tokens },
    InsufficientFunds { balance : Tokens },
    InsufficientAllowance { allowance : Tokens },
    TooOld,
    CreatedInFuture { ledger_time : Timestamp },
    TemporarilyUnavailable,
    Duplicate { duplicate_of : BlockIndex },
    GenericError { error_code : Nat, message : String },
}

impl TransferFromError {
    pub fn to_string(&self) -> String {
        match self {
            TransferFromError::BadFee { expected_fee } =>
                format!("Bad fee: {}", expected_fee),
            TransferFromError::BadBurn { min_burn_amount } =>
                format!("Bad burn: {}", min_burn_amount),
            TransferFromError::InsufficientFunds { balance } =>
                format!("Insufficient funds: {}", balance),
            TransferFromError::InsufficientAllowance { allowance } =>
                format!("Insufficient allowance: {}", allowance),
            TransferFromError::TooOld =>
                String::from("Transaction too old"),
            TransferFromError::CreatedInFuture { ledger_time } =>
                format!("Created in the future: {}", ledger_time.timestamp_nanos.to_string()),
            TransferFromError::TemporarilyUnavailable =>
                String::from("Ledger temporarily unavailable"),
            TransferFromError::Duplicate { duplicate_of } =>
                format!("Duplicate of: {}", duplicate_of),
            TransferFromError::GenericError { error_code, message } =>
                format!("Generic error code {}: {}", error_code, message)
        }
    }
}

impl Display for TransferFromError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TransferFromError::BadFee { expected_fee } => {
                write!(f, "Bad fee: {}", expected_fee)
            }
            TransferFromError::BadBurn { min_burn_amount } => {
                write!(f, "Bad burn: {}", min_burn_amount)
            }
            TransferFromError::InsufficientFunds { balance } => {
                write!(f, "Insufficient funds: {}", balance)
            }
            TransferFromError::InsufficientAllowance { allowance } => {
                write!(f, "Insufficient allowance: {}", allowance)
            }
            TransferFromError::TooOld => write!(f, "Transaction too old"),
            TransferFromError::CreatedInFuture { ledger_time } => {
                write!(f, "Created in the future: {}", ledger_time.timestamp_nanos.to_string())
            }
            TransferFromError::TemporarilyUnavailable => {
                write!(f, "Ledger temporarily unavailable")
            }
            TransferFromError::Duplicate { duplicate_of } => {
                write!(f, "Duplicate of: {}", duplicate_of)
            }
            TransferFromError::GenericError { error_code, message } => {
                write!(f, "Generic error code {}: {}", error_code, message)
            }
        }
    }
}

pub type TransferFromResult = Result<BlockIndex, TransferFromError>;

pub type Blob = Vec<u8>;

#[derive(Debug, Serialize, Deserialize, CandidType)]
pub struct Account  {
    pub owner : Principal,
    pub subaccount : Option<Subaccount>
}

#[derive(Debug, Serialize, Deserialize, CandidType)]
pub struct TransferArg {
    pub from_subaccount : Option<Subaccount>,
    pub to : Account,
    pub amount : Tokens,
    pub fee : Option<Tokens>,
    pub memo : Option<Blob>,
    pub created_at_time: Option<Timestamp>
}

#[derive(Debug, Serialize, Deserialize, CandidType)]
pub struct TransferFromArgs {
    pub spender_subaccount : Option<Subaccount>,
    pub from : Account,
    pub to : Account,
    pub amount : Tokens,
    pub fee : Option<Tokens>,
    pub memo : Option<Blob>,
    pub created_at_time: Option<Timestamp>
}

#[derive(Debug, Serialize, Deserialize, CandidType)]
pub struct AllowanceArgs { 
    pub account : Account,
    pub spender : Account 
}

#[derive(Debug, Serialize, Deserialize, CandidType)]
pub struct Allowance { 
    pub allowance : Nat,
    pub expires_at : Option<Timestamp> 
}

pub const ICRC1_FEE: u64 = 10_000;
pub const MAINNET_ICRC1_LEDGER_CANISTER_ID: &str = "mxzaz-hqaaa-aaaar-qaada-cai";

pub struct ICRC1 {
    principal: Principal,
}

impl ICRC1 {
    pub fn new(principal: Principal) -> Self {
        ICRC1 { principal }
    }

    pub async fn transfer(&self, args: TransferArg) -> TransferResult {
        let argument = (
            args.from_subaccount,
            args.to,
            args.amount,
            args.fee,
            args.memo,
            args.created_at_time
        );
        let call_result: Result<(TransferResult,), _> =
            call(self.principal, "icrc1_transfer", argument).await;

        call_result.unwrap().0
    }

    pub async fn transfer_from(&self, args: TransferFromArgs) -> TransferFromResult {
        let argument = (
            args.spender_subaccount,
            args.from,
            args.to,
            args.amount,
            args.fee,
            args.memo,
            args.created_at_time
        );
        let call_result: Result<(TransferFromResult,), _> =
            call(self.principal, "icrc2_transfer_from", argument).await;

        call_result.unwrap().0
    }

    pub async fn allowance(&self, args: AllowanceArgs) -> Allowance {
        let argument = (args.account, args.spender);
        let call_result: Result<(Allowance, ), _> =
            call(self.principal, "icrc2_allowance", argument).await;

        return call_result.unwrap().0;
    }

    pub async fn get_fee(&self) -> Tokens {
        let call_result: Result<(Tokens,), _> =
            call(self.principal, "icrc1_fee", ()).await;

        call_result.unwrap().0
    }
}