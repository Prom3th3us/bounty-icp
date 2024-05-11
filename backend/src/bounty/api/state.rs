use std::cell::RefCell;
use std::collections::{HashMap, HashSet};

use candid::{CandidType, Nat, Principal};
use serde::Deserialize;

pub type IssueId = String;

pub type PullRequestId = String;

pub type Time = u64;

#[derive(Debug, CandidType, Deserialize, Clone)]
pub struct Contributor {
    pub address: Principal,
}

#[derive(Debug, CandidType, Deserialize, Clone)]
pub struct PullRequestMetadata {
    pub accepted_at: Time,
    pub updated_at: Time,
}

#[derive(Debug, CandidType, Deserialize, Clone)]
pub struct PullRequest {
    pub id: PullRequestId,
    pub contributor: Contributor,
    pub metadata: PullRequestMetadata,
}

#[derive(Debug, CandidType, Deserialize, Clone)]
pub struct Bounty {
    pub amount: Nat,
    pub winner: Option<PullRequestId>,
    pub accepted_prs: HashMap<PullRequestId, PullRequest>,
}

#[derive(Debug, CandidType, Deserialize, Clone)]
pub struct IssueMetadata {
    pub created_at: Time,
    pub updated_at: Time,
}

#[derive(Debug, CandidType, Deserialize, Clone)]
pub struct Issue {
    pub id: IssueId,
    pub maintainer: Contributor,
    pub bounty: Bounty,
    pub metadata: IssueMetadata,
}

#[derive(Debug, CandidType, Deserialize, Default)]
pub struct Metadata {
    pub custodians: HashSet<Principal>,
    pub created_at: Time,
    pub upgraded_at: Time,
}

#[derive(Debug, CandidType, Deserialize, Default)]
pub struct BountyState {
    pub metadata: Metadata,
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
    static BOUNTY_STATE: RefCell<BountyState> = RefCell::new(BountyState::default());
}

pub fn with<T, F: FnOnce(&BountyState) -> T>(f: F) -> T {
    BOUNTY_STATE.with(|state| f(&state.borrow()))
}

pub fn with_mut<T, F: FnOnce(&mut BountyState) -> T>(f: F) -> T {
    BOUNTY_STATE.with(|state| f(&mut state.borrow_mut()))
}

#[derive(Debug, CandidType, Deserialize)]
pub struct InitArgs {
    pub custodians: Option<HashSet<Principal>>,
}

impl BountyState {
    pub fn init_metadata(&mut self, time: Time, default_custodian: Principal, args: Option<InitArgs>) {
        let metadata = self.metadata_mut();
        metadata.custodians.insert(default_custodian);
        if let Some(args) = args {
            if let Some(custodians) = args.custodians {
                for custodians in custodians {
                    metadata.custodians.insert(custodians);
                }
            }
        }
        metadata.created_at = time;
        metadata.upgraded_at = time;
    }

    pub fn metadata(&self) -> &Metadata {
        &self.metadata
    }

    pub fn metadata_mut(&mut self) -> &mut Metadata {
        &mut self.metadata
    }

    pub fn is_issue_existed(&self, github_issue_id: &IssueId) -> bool {
        self.github_issues.contains_key(github_issue_id)
    }

    pub fn init(&mut self, time: Time, caller: Principal, args: Option<InitArgs>) {
        self.init_metadata(time, caller, args);
    }

    pub fn is_canister_custodian(&self, caller: Principal) -> Result<(), String> {
        self.metadata()
            .custodians
            .contains(&caller)
            .then_some(())
            .ok_or_else(|| "Caller is not an custodian of canister".into())
    }
}
