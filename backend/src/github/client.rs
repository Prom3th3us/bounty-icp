use super::api::get_fixed_by::get_fixed_by_impl;
use super::api::get_issue::get_issue_impl;

use super::utils::IssueResponse;

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
}
