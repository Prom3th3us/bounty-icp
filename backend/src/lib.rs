use candid::{Nat, Principal};

pub mod provider {
    pub mod github {
        pub mod api {
            pub mod get_fixed_by;
            pub mod get_is_merged;
            pub mod get_issue;
            pub mod get_merged_details;
        }
        pub mod client;
        pub mod utils;
    }
}
use ic_cdk::api::time;
use ic_cdk::caller;
use provider::github::api::get_fixed_by::FixedByErr;
use provider::github::api::get_is_merged::IsMergedErr;
use provider::github::api::get_issue::{IssueErr, IssueResponse};
use provider::github::api::get_merged_details::{MergeDetailsErr, PrDetailsResponse};
use provider::github::client::{GithubClient, IGithubClient};

pub mod bounty {
    pub mod api {
        pub mod accept;
        pub mod claim;
        pub mod deposit;
        pub mod icrc1;
        pub mod init;
        pub mod register_issue;
        pub mod state;
        pub mod unregister_issue;
    }
}

use bounty::api::accept::{accept_impl, AcceptReceipt};
use bounty::api::deposit::{deposit_impl, DepositReceipt};
use bounty::api::init::init_impl;
use bounty::api::register_issue::{register_issue_impl, RegisterIssueReceipt};
use bounty::api::state::{InitArgs, IssueId, PullRequestId, UserId};
use bounty::api::unregister_issue::{unregister_issue_impl, UnRegisterIssueReceipt};

use crate::bounty::api::state::is_canister_custodian_guard;

pub mod users {
    pub mod api {
        pub mod register_user;
        pub mod upsert_user_wallet;
    }
}

use users::api::register_user::{register_user_impl, RegisterUserReceipt};
use users::api::upsert_user_wallet::{upsert_user_wallet_impl, UpsertUserWalletReceipt};

// GITHUB SERVICE
#[ic_cdk::update(guard=is_canister_custodian_guard)]
async fn get_issue(github_token: String) -> Result<IssueResponse, IssueErr> {
    let owner = "input-output-hk".to_string();
    let repo = "hydra".to_string();
    let issue_nbr = 1218;
    let client = GithubClient {
        owner,
        repo,
        github_token,
    };
    return client.get_issue(issue_nbr).await;
}

#[ic_cdk::update(guard=is_canister_custodian_guard)]
async fn get_fixed_by(github_token: String) -> Result<String, FixedByErr> {
    let owner = "input-output-hk".to_string();
    let repo = "hydra".to_string();
    let issue_nbr = 1370;
    let client = GithubClient {
        owner,
        repo,
        github_token,
    };
    return client.get_fixed_by(issue_nbr).await;
}

#[ic_cdk::update(guard=is_canister_custodian_guard)]
async fn get_is_merged(github_token: String) -> Result<String, IsMergedErr> {
    let owner = "input-output-hk".to_string();
    let repo = "hydra".to_string();
    let pr_nbr = 1266;
    let client = GithubClient {
        owner,
        repo,
        github_token,
    };
    return client.get_is_merged(pr_nbr).await;
}

#[ic_cdk::update(guard=is_canister_custodian_guard)]
async fn get_merged_details(github_token: String) -> Result<PrDetailsResponse, MergeDetailsErr> {
    let owner = "input-output-hk".to_string();
    let repo = "hydra".to_string();
    let pr_nbr = 1266;
    let client = GithubClient {
        owner,
        repo,
        github_token,
    };
    return client.get_merged_details(pr_nbr).await;
}

// BOUNTY SERVICE
#[ic_cdk::init]
fn init(args: Option<InitArgs>) -> () {
    init_impl(time(), caller(), args);
}

#[ic_cdk::update(guard=is_canister_custodian_guard)]
fn accept(
    github_user_id: UserId,
    github_issue_id: IssueId,
    github_pr_id: PullRequestId,
) -> AcceptReceipt {
    return accept_impl(github_user_id, github_issue_id, github_pr_id, time());
}

#[ic_cdk::update(guard=is_canister_custodian_guard)]
async fn deposit() -> DepositReceipt {
    return deposit_impl().await;
}

// #[ic_cdk::update(guard=is_canister_custodian_guard)]
#[ic_cdk::update]
async fn healthcheck() -> String {
    return "OK".to_string();
}

#[ic_cdk::update(guard=is_canister_custodian_guard)]
fn register_issue(
    github_user_id: UserId,
    github_issue_id: IssueId,
    amount: Nat,
) -> RegisterIssueReceipt {
    return register_issue_impl(github_user_id, github_issue_id, amount, time());
}

#[ic_cdk::update(guard=is_canister_custodian_guard)]
fn unregister_issue(github_user_id: UserId, github_issue_id: IssueId) -> UnRegisterIssueReceipt {
    return unregister_issue_impl(github_user_id, github_issue_id);
}

// USER SERVICE
#[ic_cdk::update(guard=is_canister_custodian_guard)]
fn register_user(github_user_id: UserId) -> RegisterUserReceipt {
    return register_user_impl(github_user_id, time());
}

#[ic_cdk::update(guard=is_canister_custodian_guard)]
fn upsert_user_wallet(
    github_user_id: UserId,
    wallet: Option<Principal>,
) -> UpsertUserWalletReceipt {
    return upsert_user_wallet_impl(github_user_id, wallet);
}
