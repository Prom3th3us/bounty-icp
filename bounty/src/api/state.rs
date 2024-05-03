use std::collections::HashMap;

use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

type IssueId = i32;
type PullRequestId = i32;

#[derive(Debug, Serialize, Deserialize, CandidType)]
pub struct BountyState {
    pub authority: Principal,
    pub github_issue_id: IssueId,
    pub interested_contributors: HashMap<PullRequestId, Contributor>,
    pub claimed: bool,
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
