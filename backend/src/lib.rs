mod github {
    pub mod get_issue;
}

use github::get_issue::{IssueResponse, get_gh_issue_impl};

#[ic_cdk::update]
async fn get_gh_issue(github_token: String) -> IssueResponse {
    get_gh_issue_impl(github_token).await
}