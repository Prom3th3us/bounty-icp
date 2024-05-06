use std::collections::HashMap;

use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

pub type IssueId = String;
pub type PullRequestId = String;

#[derive(Debug, Serialize, Deserialize, CandidType, Clone, Builder)]
pub struct Contributor {
    pub address: Principal,
    pub crypto_address: String,
}

#[derive(Debug, Serialize, Deserialize, CandidType, Clone, Builder)]
pub struct PullRequest {
    pub id: PullRequestId,
    pub contributor: Contributor,
}

#[derive(Debug, Serialize, Deserialize, CandidType, Clone, Builder)]
pub struct Bounty {
    pub amount: i32,
    pub winner: Option<PullRequestId>,
    pub accepted_prs: HashMap<PullRequestId, PullRequest>,
}

#[derive(Debug, Serialize, Deserialize, CandidType, Clone, Builder)]
pub struct Issue {
    pub id: IssueId,
    pub maintainer: Contributor,
    pub bounty: Bounty,
}

#[derive(Debug, Serialize, Deserialize, CandidType)]
pub struct BountyState {
    pub authority: Principal,
    pub github_issues: HashMap<IssueId, Issue>,   
}
// Define thread-local storage for the bounty canister state
// WASM is single-threaded by nature. [RefCell] and [thread_local!] are used despite being not totally safe primitives.
// This is to ensure that the canister state can be used throughout.
// Your other option here is to avoid [thread_local!] and use a [RefCell<RwLock/Mutex/Atomic>].
// Here we use [thread_local!] because it is simpler.
thread_local! {
    // Currently, a single canister smart contract is limited to 4 GB of storage due to WebAssembly limitations.
    // To ensure that our canister does not exceed this limit, we restrict memory usage to at most 2 GB because 
    // up to 2x memory may be needed for data serialization during canister upgrades.
    pub static BOUNTY_STATE: std::cell::RefCell<Option<BountyState>> = std::cell::RefCell::new(None);
}
