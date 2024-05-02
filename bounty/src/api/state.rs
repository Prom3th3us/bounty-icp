
use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, CandidType)]
pub struct BountyState {
    pub authority: Principal,
    pub github_issue_id: i32,
    pub interested_contributors: Vec<Contributor>,
    pub claimed: bool
}

#[derive(Debug, Serialize, Deserialize, CandidType)]
pub struct Contributor {
    pub address: Principal,
    pub crypto_address: String,
}

// Define thread-local storage for the bounty canister state
thread_local! {
    pub static BOUNTY_STATE: std::cell::RefCell<Option<BountyState>> = std::cell::RefCell::new(None);
}
