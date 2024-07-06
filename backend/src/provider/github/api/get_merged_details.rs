use candid::CandidType;
use ic_cdk::api::management_canister::http_request::{
    http_request, CanisterHttpRequestArgument, HttpMethod, HttpResponse,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::provider::github::utils::{github_api_host, mk_request_headers};

#[derive(Debug, Serialize, Deserialize, CandidType)]
pub struct PrDetailsResponse {
    state: Option<String>,
    closed_at: Option<String>,
    merge_commit_sha: Option<String>,
    merged_at: Option<String>,
    links: Option<Link>,
    merged: Option<bool>,
    merged_by: Option<User>,
}

#[derive(Debug, Serialize, Deserialize, CandidType)]
pub struct Link {
    issue: IssueLink,
}

#[derive(Debug, Serialize, Deserialize, CandidType, Default)]
pub struct IssueLink {
    href: String,
}

#[derive(Debug, Serialize, Deserialize, CandidType)]
pub struct User {
    login: String,
    id: u64,
}

#[derive(Debug, Serialize, Deserialize, CandidType)]
pub enum MergeDetailsErr {
    Rejected { error_message: String },
}

pub async fn get_merge_details_impl(
    owner: &str,
    repo: &str,
    github_token: &str,
    pr_nbr: i32,
    cycles: u128,
) -> Result<PrDetailsResponse, MergeDetailsErr> {
    // Setup the URL and its query parameters
    let url = format!(
        "https://{}/repos/{}/{}/pulls/{}",
        github_api_host(),
        owner,
        repo,
        pr_nbr
    );

    // Prepare headers for the system http_request call
    let request_headers = mk_request_headers(github_token);

    // Create the request argument
    let request = CanisterHttpRequestArgument {
        url,
        method: HttpMethod::GET,
        body: None,
        max_response_bytes: None,
        transform: None,
        headers: request_headers,
    };

    // Make the HTTP request and wait for the response
    http_request(request, cycles)
        .await
        .map_err(|(rejection_code, message)| {
            let error_message = format!(
                "The http_request resulted in an error. RejectionCode: {:?}, Error: {}",
                rejection_code, message
            );
            MergeDetailsErr::Rejected { error_message }
        })
        .and_then(|(response,)| transform_response(response))
}

// Define a function to transform the response body
fn transform_response(raw_response: HttpResponse) -> Result<PrDetailsResponse, MergeDetailsErr> {
    // Deserialize the raw response body into a serde_json::Value
    let parsed_response: Value =
        serde_json::from_slice(&raw_response.body).map_err(|_| MergeDetailsErr::Rejected {
            error_message: "Failed to parse JSON response: {}".to_string(),
        })?;

    // Print the parsed response for debugging
    println!("Parsed response: {:?}", parsed_response);

    let obj = parsed_response
        .as_object()
        .ok_or(MergeDetailsErr::Rejected {
            error_message: "Failed to map parsed JSON response as object: {}".to_string(),
        })?;

    let state = obj
        .get("state")
        .and_then(|value| value.as_str())
        .map(|s| s.to_string());
    let closed_at = obj
        .get("closed_at")
        .and_then(|value| value.as_str())
        .map(|s| s.to_string());
    let merge_commit_sha = obj
        .get("merge_commit_sha")
        .and_then(|value| value.as_str())
        .map(|s| s.to_string());
    let merged_at = obj
        .get("merged_at")
        .and_then(|value| value.as_str())
        .map(|s| s.to_string());
    let links = obj
        .get("_links")
        .and_then(|value| value.get("issue"))
        .and_then(|issue| issue.as_object())
        .and_then(|issue| {
            issue
                .get("href")
                .and_then(|href| href.as_str())
                .map(|s| s.to_string())
                .map(|href| Link {
                    issue: IssueLink { href },
                })
        });
    let merged = obj.get("merged").and_then(|value| value.as_bool());
    let merged_by = obj.get("merged_by").and_then(|value| {
        value.as_object().and_then(|user| {
            let m_login = user
                .get("login")
                .and_then(|login| login.as_str())
                .map(|s| s.to_string());

            let m_id = user.get("id").and_then(|id| id.as_u64());

            m_login.and_then(|login| m_id.map(|id| User { login, id }))
        })
    });

    // Construct the transformed response object
    let transformed_response = PrDetailsResponse {
        state,
        closed_at,
        merge_commit_sha,
        merged_at,
        links,
        merged,
        merged_by,
    };

    // Print the transformed response for debugging
    println!("Transformed response: {:?}", transformed_response);

    Ok(transformed_response)
}
