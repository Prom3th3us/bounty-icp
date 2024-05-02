use candid::Principal;

mod api {
    pub mod state;
    pub mod init;
    pub mod accept;
    pub mod deposit;
    pub mod icrc1;
}

use api::state::Contributor;
use api::init::init_impl;
use api::accept::accept_impl;
use api::deposit::{deposit_impl, deposit_direct_impl, DepositReceipt};

#[ic_cdk::init]
fn init(authority: Principal, github_issue_id: i32) -> () {
    init_impl(authority, github_issue_id);
}

#[ic_cdk::update]
fn accept(contributor: Contributor) -> () {
    accept_impl(contributor);
}

#[ic_cdk::update]
async fn deposit() -> DepositReceipt {
    return deposit_impl().await;
}

#[ic_cdk::update]
async fn deposit_direct(amount: u64) -> DepositReceipt {
    return deposit_direct_impl(amount).await;
}

#[ic_cdk::update]
async fn healthcheck() -> String {
    return "OK".to_string();
}
