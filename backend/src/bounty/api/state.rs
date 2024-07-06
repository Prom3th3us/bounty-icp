use std::cell::RefCell;
use std::collections::{HashMap, HashSet};

use candid::{CandidType, Nat, Principal};
use serde::Deserialize;

pub type IssueId = String;

pub type UserId = String;

pub type PullRequestId = String;

pub type Time = u64;

#[derive(Debug, CandidType, Deserialize, Clone)]
pub struct PullRequestMetadata {
    accepted_at: Time,
    updated_at: Time,
}

impl PullRequestMetadata {
    pub fn new(accepted_at: Time, updated_at: Time) -> Self {
        PullRequestMetadata {
            accepted_at,
            updated_at,
        }
    }
}

#[derive(Debug, CandidType, Deserialize, Clone)]
pub struct PullRequest {
    id: PullRequestId,
    contributor: UserId,
    metadata: PullRequestMetadata,
}

impl PullRequest {
    pub fn new(id: &PullRequestId, contributor: &UserId, metadata: PullRequestMetadata) -> Self {
        PullRequest {
            id: id.to_string(),
            contributor: contributor.to_string(),
            metadata,
        }
    }
}

#[derive(Debug, CandidType, Deserialize, Clone)]
pub struct Bounty {
    amount: Nat,
    winner: Option<PullRequestId>,
    accepted_prs: HashMap<PullRequestId, PullRequest>,
}

impl Bounty {
    pub fn new(
        amount: Nat,
        winner: Option<PullRequestId>,
        accepted_prs: HashMap<PullRequestId, PullRequest>,
    ) -> Self {
        Bounty {
            amount,
            winner,
            accepted_prs,
        }
    }

    pub fn winner(&self) -> &Option<PullRequestId> {
        &self.winner
    }

    pub fn accepted_prs(&self) -> &HashMap<PullRequestId, PullRequest> {
        &self.accepted_prs
    }

    pub fn insert_pull_request(&mut self, github_pr_id: PullRequestId, pr: PullRequest) {
        self.accepted_prs.insert(github_pr_id, pr);
    }

    pub fn remove_pull_request(&mut self, github_pr_id: &PullRequestId) {
        self.accepted_prs.remove(github_pr_id);
    }
}

#[derive(Debug, CandidType, Deserialize, Clone)]
pub struct IssueMetadata {
    created_at: Time,
    updated_at: Time,
}

impl IssueMetadata {
    pub fn new(created_at: Time, updated_at: Time) -> Self {
        IssueMetadata {
            created_at,
            updated_at,
        }
    }
}

#[derive(Debug, CandidType, Deserialize, Clone)]
pub struct Issue {
    id: IssueId,
    maintainer: UserId,
    bounty: Bounty,
    metadata: IssueMetadata,
}

impl Issue {
    pub fn new(id: &IssueId, maintainer: &UserId, bounty: Bounty, metadata: IssueMetadata) -> Self {
        Issue {
            id: id.to_string(),
            maintainer: maintainer.to_string(),
            bounty,
            metadata,
        }
    }

    pub fn bounty(&self) -> &Bounty {
        &self.bounty
    }

    pub fn bounty_mut(&mut self) -> &mut Bounty {
        &mut self.bounty
    }
}

#[derive(Debug, CandidType, Deserialize, Default)]
pub struct Metadata {
    custodians: HashSet<Principal>,
    created_at: Time,
    upgraded_at: Time,
}

impl Metadata {
    pub fn insert_custodian(&mut self, custodian: Principal) {
        self.custodians.insert(custodian);
    }
}

#[derive(Debug, CandidType, Deserialize, Default)]
pub struct GitHubUser {
    user_id: UserId,
    wallet: Option<Principal>,
    created_at: Time,
    updated_at: Time,
}

impl GitHubUser {
    pub fn new(
        user_id: &str,
        wallet: Option<Principal>,
        created_at: Time,
        updated_at: Time,
    ) -> Self {
        GitHubUser {
            user_id: user_id.to_string(),
            wallet,
            created_at,
            updated_at,
        }
    }

    pub fn wallet(&self) -> Option<Principal> {
        self.wallet
    }

    pub fn set_wallet(&mut self, wallet: Option<Principal>) {
        self.wallet = wallet
    }
}

#[derive(Debug, CandidType, Deserialize, Default)]
pub struct BountyState {
    metadata: Metadata,
    github_issues: HashMap<IssueId, Issue>,
    github_known_users: HashMap<UserId, GitHubUser>,
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
    custodians: Option<HashSet<Principal>>,
}

impl InitArgs {
    pub fn get_custodians(&self) -> &Option<HashSet<Principal>> {
        &self.custodians
    }
}

impl BountyState {
    pub fn init_metadata(
        &mut self,
        time: Time,
        default_custodian: Principal,
        args: Option<InitArgs>,
    ) {
        let metadata = self.metadata_mut();
        metadata.insert_custodian(default_custodian);
        if let Some(args) = args {
            if let Some(custodians) = args.get_custodians() {
                for custodian in custodians {
                    metadata.insert_custodian(*custodian);
                }
            }
        }
        metadata.created_at = time;
        metadata.upgraded_at = time;
    }

    pub fn bounty(&self) -> &Metadata {
        &self.metadata
    }

    pub fn metadata(&self) -> &Metadata {
        &self.metadata
    }

    // REVIEW
    pub fn metadata_mut(&mut self) -> &mut Metadata {
        &mut self.metadata
    }

    pub fn github_known_users(&self) -> &HashMap<UserId, GitHubUser> {
        &self.github_known_users
    }

    pub fn github_known_users_mut(&mut self) -> &mut HashMap<UserId, GitHubUser> {
        &mut self.github_known_users
    }

    pub fn insert_github_user(&mut self, github_user_id: String, github_user: GitHubUser) {
        self.github_known_users.insert(github_user_id, github_user);
    }

    pub fn is_user_existed(&self, github_user_id: &str) -> bool {
        self.github_known_users.contains_key(github_user_id)
    }

    pub fn github_issues(&self) -> &HashMap<IssueId, Issue> {
        &self.github_issues
    }

    pub fn github_issues_mut(&mut self) -> &mut HashMap<IssueId, Issue> {
        &mut self.github_issues
    }

    pub fn insert_github_issue(&mut self, github_issue_id: String, github_issue: Issue) {
        self.github_issues.insert(github_issue_id, github_issue);
    }

    pub fn is_issue_existed(&self, github_issue_id: &str) -> bool {
        self.github_issues.contains_key(github_issue_id)
    }

    pub fn remove_github_issue(&mut self, github_issue_id: &str) {
        self.github_issues.remove(github_issue_id);
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

pub fn is_canister_custodian_guard() -> Result<(), String> {
    self::with(|state| {
        state
            .metadata()
            .custodians
            .contains(&ic_cdk::caller())
            .then_some(())
            .ok_or_else(|| "Caller is not an custodian of canister".into())
    })
}
