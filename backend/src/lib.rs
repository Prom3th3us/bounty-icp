mod github {
    pub mod api {
        pub mod get_issue;
        pub mod get_fixed_by;
    }
    pub mod utils;
    pub mod client;
}

use github::utils::IssueResponse;
use github::client::GithubClient;

#[ic_cdk::update]
async fn get_gh_issue(github_token: String) -> IssueResponse {
    let owner = "input-output-hk".to_string();
    let repo = "hydra".to_string();
    let issue_nbr = 1404;
    let client = GithubClient{owner, repo, github_token};
    return client.get_issue(issue_nbr).await;
}

#[ic_cdk::update]
async fn get_gh_fixed_by(github_token: String) -> String {
    let owner = "input-output-hk".to_string();
    let repo = "hydra".to_string();
    let issue_nbr = 1370;
    let client = GithubClient{owner, repo, github_token};
    return client.get_fixed_by(issue_nbr).await;
}