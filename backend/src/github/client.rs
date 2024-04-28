use super::api::get_fixed_by::get_fixed_by_impl;
use super::api::get_issue::{get_issue_impl, IssueResponse};
use super::api::get_is_merged::get_is_merged_impl;
use super::api::get_merged_details::{get_merge_details_impl, PrDetailsResponse};


pub struct GithubClient {
    pub owner: String,
    pub repo: String,
    pub github_token: String,
}

impl GithubClient {
    pub async fn get_issue(&self, issue_nbr: i32) -> IssueResponse {
        get_issue_impl(
            self.owner.clone(),
            self.repo.clone(),
            issue_nbr,
            self.github_token.clone(),
        )
        .await
    }
    pub async fn get_fixed_by(&self, issue_nbr: i32) -> String {
        get_fixed_by_impl(
            self.owner.clone(),
            self.repo.clone(),
            issue_nbr,
        )
        .await
    }
    pub async fn get_is_merged(&self, pr_nbr: i32) -> String {
        get_is_merged_impl(
            self.owner.clone(),
            self.repo.clone(),
            pr_nbr,
        )
        .await
    }
    pub async fn get_merged_details(&self, pr_nbr: i32) -> PrDetailsResponse {
        get_merge_details_impl(
            self.owner.clone(),
            self.repo.clone(),
            pr_nbr,
            self.github_token.clone(),
        )
        .await
    }

    
}
