use candid::{CandidType, Deserialize, Nat, Principal};
use ic_cdk::api::call::call;

pub struct ICRC1 {
    principal: Principal,
}

#[derive(CandidType, Debug, PartialEq, Deserialize)]
pub enum TxError {
    InsufficientBalance,
    InsufficientAllowance,
    Unauthorized,
    LedgerTrap,
    AmountTooSmall,
    BlockUsed,
    ErrorOperationStyle,
    ErrorTo,
    Other,
}
pub type TxReceipt = Result<Nat, TxError>;

#[allow(non_snake_case)]
#[derive(CandidType, Clone, Debug, Deserialize)]
pub struct Metadata {
    pub logo: String,
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub totalSupply: Nat,
    pub owner: Principal,
    pub fee: Nat,
}

// pub const ICRC1_FEE: u64 = 10_000;
pub const MAINNET_ICRC1_LEDGER_CANISTER_ID: &str = "mxzaz-hqaaa-aaaar-qaada-cai";
    
impl ICRC1 {
    pub fn new(principal: Principal) -> Self {
        ICRC1 { principal }
    }

    // pub async fn transfer(&self, target: Principal, amount: Nat) -> TxReceipt {
    //     let call_result: Result<(TxReceipt,), _> =
    //         call(self.principal, "icrc1_transfer", (target, amount)).await;

    //     call_result.unwrap().0
    // }

    pub async fn transfer_from(
        &self,
        source: Principal,
        target: Principal,
        amount: Nat,
    ) -> TxReceipt {
        let call_result: Result<(TxReceipt,), _> =
            call(self.principal, "icrc2_transfer_from", (source, target, amount)).await;

        call_result.unwrap().0
    }

    pub async fn allowance(&self, owner: Principal, spender: Principal) -> Nat {
        let call_result: Result<(Nat,), _> =
            call(self.principal, "icrc2_allowance", (owner, spender)).await;

        call_result.unwrap().0
    }

    pub async fn get_metadata(&self) -> Metadata {
        let call_result: Result<(Metadata,), _> =
            call(self.principal, "icrc1_metadata", ()).await;

        call_result.unwrap().0
    }
}