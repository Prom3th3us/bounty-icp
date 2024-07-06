use crate::provider::github::api;

use api::get_fixed_by::{get_fixed_by_impl, FixedByErr};
use api::get_is_merged::{get_is_merged_impl, IsMergedErr};
use api::get_issue::{get_issue_impl, IssueErr, IssueResponse};
use api::get_merged_details::{get_merge_details_impl, MergeDetailsErr, PrDetailsResponse};
use api::get_user_exists::{get_user_exists_impl, UserExistsError};

pub struct GithubClient {
    owner: String,
    repo: String,
    github_token: String,
}

impl GithubClient {
    pub fn new(owner: String, repo: String, github_token: String) -> Self {
        GithubClient {
            owner,
            repo,
            github_token,
        }
    }

    pub fn get_owner(&self) -> &str {
        &self.owner
    }
    pub fn get_repo(&self) -> &str {
        &self.repo
    }
    pub fn get_github_token(&self) -> &str {
        &self.github_token
    }
}

#[async_trait::async_trait]
pub trait IGithubClient {
    async fn get_issue(&self, issue_nbr: i32) -> Result<IssueResponse, IssueErr>;
    async fn get_fixed_by(&self, issue_nbr: i32) -> Result<String, FixedByErr>;
    async fn get_is_merged(&self, pr_nbr: i32) -> Result<String, IsMergedErr>;
    async fn get_merged_details(&self, pr_nbr: i32) -> Result<PrDetailsResponse, MergeDetailsErr>;
    async fn get_user_exists(&self, user_id: String) -> Result<String, UserExistsError>;
}

#[async_trait::async_trait]
impl IGithubClient for GithubClient {
    async fn get_issue(&self, issue_nbr: i32) -> Result<IssueResponse, IssueErr> {
        get_issue_impl(
            self.get_owner(),
            self.get_repo(),
            self.get_github_token(),
            issue_nbr,
            get_cycles(),
        )
        .await
    }
    async fn get_fixed_by(&self, issue_nbr: i32) -> Result<String, FixedByErr> {
        get_fixed_by_impl(self.get_owner(), self.get_repo(), issue_nbr, get_cycles()).await
    }
    async fn get_is_merged(&self, pr_nbr: i32) -> Result<String, IsMergedErr> {
        get_is_merged_impl(self.get_owner(), self.get_repo(), pr_nbr, get_cycles()).await
    }
    async fn get_user_exists(&self, user_id: String) -> Result<String, UserExistsError> {
        get_user_exists_impl(self.get_github_token(), user_id, get_cycles()).await
    }
    async fn get_merged_details(&self, pr_nbr: i32) -> Result<PrDetailsResponse, MergeDetailsErr> {
        get_merge_details_impl(
            self.get_owner(),
            self.get_repo(),
            self.get_github_token(),
            pr_nbr,
            get_cycles(),
        )
        .await
    }
}

// FIXME
fn get_cycles() -> u128 {
    2_500_000_000
}
