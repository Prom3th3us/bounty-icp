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
}
