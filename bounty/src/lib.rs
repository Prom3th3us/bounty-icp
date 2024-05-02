use candid::Principal;

mod api {
    pub mod accept;
    pub mod deposit;
    pub mod icrc1;
    pub mod init;
    pub mod state;
}

use api::accept::accept_impl;
use api::deposit::{deposit_direct_impl, deposit_impl, DepositReceipt};
use api::init::init_impl;
use api::state::Contributor;

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
