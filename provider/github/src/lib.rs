mod api {
    pub mod get_fixed_by;
    pub mod get_is_merged;
    pub mod get_issue;
    pub mod get_merged_details;
}
mod client;
mod utils;

use api::get_issue::IssueResponse;
use api::get_merged_details::PrDetailsResponse;

use client::GithubClient;

#[ic_cdk::update]
async fn get_issue(github_token: String) -> IssueResponse {
    let owner = "input-output-hk".to_string();
    let repo = "hydra".to_string();
    let issue_nbr = 1218;
    let client = GithubClient {
        owner,
        repo,
        github_token,
    };
    return client.get_issue(issue_nbr).await;
}

#[ic_cdk::update]
async fn get_fixed_by(github_token: String) -> String {
    let owner = "input-output-hk".to_string();
    let repo = "hydra".to_string();
    let issue_nbr = 1370;
    let client = GithubClient {
        owner,
        repo,
        github_token,
    };
    return client.get_fixed_by(issue_nbr).await;
}

#[ic_cdk::update]
async fn get_is_merged(github_token: String) -> String {
    let owner = "input-output-hk".to_string();
    let repo = "hydra".to_string();
    let pr_nbr = 1266;
    let client = GithubClient {
        owner,
        repo,
        github_token,
    };
    return client.get_is_merged(pr_nbr).await;
}

#[ic_cdk::update]
async fn get_merged_details(github_token: String) -> PrDetailsResponse {
    let owner = "input-output-hk".to_string();
    let repo = "hydra".to_string();
    let pr_nbr = 1266;
    let client = GithubClient {
        owner,
        repo,
        github_token,
    };
    return client.get_merged_details(pr_nbr).await;
}
