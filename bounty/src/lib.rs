use candid::Principal;

pub mod api {
    pub mod accept;
    pub mod claim;
    pub mod deposit;
    pub mod icrc1;
    pub mod init;
    pub mod state;
}

use api::accept::accept_impl;
use api::deposit::{deposit_impl, DepositReceipt};
use api::init::init_impl;
use api::state::Contributor;

#[ic_cdk::init]
fn init(authority: Principal, github_issue_id: i32) -> () {
    init_impl(authority, github_issue_id);
}

#[ic_cdk::update]
fn accept(contributor: Contributor, github_pr_id: i32) -> () {
    accept_impl(contributor, github_pr_id);
}

#[ic_cdk::update]
async fn deposit() -> DepositReceipt {
    return deposit_impl().await;
}

#[ic_cdk::update]
async fn healthcheck() -> String {
    return "OK".to_string();
}
