use ic_cdk::api::management_canister::http_request::HttpHeader;
use serde::{Deserialize, Serialize};
use candid::CandidType;

// Define the IssueResponse struct to represent the transformed response
#[derive(Debug, Serialize, Deserialize, CandidType)]
pub struct IssueResponse {
    pub state: Option<String>,
    pub login: Option<String>,
    pub id: Option<String>,
    pub milestone_state: Option<String>,
    pub closed_at: Option<String>,
}

pub fn github_api_host() -> String {
  return "api.github.com".to_string();   
}

pub fn mk_request_headers(github_token: String) -> Vec<HttpHeader> {
    return vec![
        HttpHeader {
            name: "Authorization".to_string(),
            value: format!("Bearer {}", github_token),
        },
        HttpHeader {
            name: "Accept".to_string(),
            value: "application/vnd.github+json".to_string(),
        },
        HttpHeader {
            name: "X-GitHub-Api-Version".to_string(),
            value: "2022-11-28".to_string(),
        },
    ];
}
