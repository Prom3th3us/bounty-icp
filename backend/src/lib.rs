#[macro_use]
extern crate derive_builder;

use candid::Principal;

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
use provider::github::api::get_fixed_by::GetFixedByError;
use provider::github::api::get_issue::IssueResponse;
use provider::github::api::get_merged_details::PrDetailsResponse;
use provider::github::client::{GithubClient, IGithubClient};

pub mod bounty {
    pub mod api {
        pub mod accept;
        pub mod claim;
        pub mod deposit;
        pub mod icrc1;
        pub mod init;
        pub mod state;
    }
}

use bounty::api::accept::accept_impl;
use bounty::api::init::init_impl;
use bounty::api::deposit::{deposit_impl, DepositReceipt};
use bounty::api::state::Contributor;

// GITHUB SERVICE
#[ic_cdk::update]
async fn get_issue(github_token: String) -> IssueResponse {
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

#[ic_cdk::update]
async fn get_fixed_by(github_token: String) -> Result<String, GetFixedByError> {
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

#[ic_cdk::update]
async fn get_is_merged(github_token: String) -> String {
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

#[ic_cdk::update]
async fn get_merged_details(github_token: String) -> PrDetailsResponse {
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
fn init(authority: Principal) -> () {
    init_impl(authority);
}

#[ic_cdk::update]
fn accept(contributor: Contributor, github_issue_id: i32, github_pr_id: i32) -> () {
    accept_impl(contributor, github_issue_id, github_pr_id);
}

#[ic_cdk::update]
async fn deposit() -> DepositReceipt {
    return deposit_impl().await;
}

#[ic_cdk::update]
async fn healthcheck() -> String {
    return "OK".to_string();
}
