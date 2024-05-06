use super::api::get_fixed_by::{get_fixed_by_impl, FixedByErr};
use super::api::get_is_merged::{get_is_merged_impl, IsMergedErr};
use super::api::get_issue::{get_issue_impl, IssueErr, IssueResponse};
use super::api::get_merged_details::{get_merge_details_impl, MergeDetailsErr, PrDetailsResponse};

pub struct GithubClient {
    pub owner: String,
    pub repo: String,
    pub github_token: String,
}

#[async_trait::async_trait]
pub trait IGithubClient {
    async fn get_issue(&self, issue_nbr: i32) -> Result<IssueResponse, IssueErr>;
    async fn get_fixed_by(&self, issue_nbr: i32) -> Result<String, FixedByErr>;
    async fn get_is_merged(&self, pr_nbr: i32) -> Result<String, IsMergedErr>;
    async fn get_merged_details(&self, pr_nbr: i32) -> Result<PrDetailsResponse, MergeDetailsErr>;
}

#[async_trait::async_trait]
impl IGithubClient for GithubClient {
    async fn get_issue(&self, issue_nbr: i32) -> Result<IssueResponse, IssueErr> {
        get_issue_impl(
            self.owner.clone(),
            self.repo.clone(),
            issue_nbr,
            self.github_token.clone(),
        )
        .await
    }
    async fn get_fixed_by(&self, issue_nbr: i32) -> Result<String, FixedByErr> {
        get_fixed_by_impl(self.owner.clone(), self.repo.clone(), issue_nbr).await
    }
    async fn get_is_merged(&self, pr_nbr: i32) -> Result<String, IsMergedErr> {
        get_is_merged_impl(self.owner.clone(), self.repo.clone(), pr_nbr).await
    }
    async fn get_merged_details(&self, pr_nbr: i32) -> Result<PrDetailsResponse, MergeDetailsErr> {
        get_merge_details_impl(
            self.owner.clone(),
            self.repo.clone(),
            pr_nbr,
            self.github_token.clone(),
        )
        .await
    }
}
