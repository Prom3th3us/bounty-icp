use ic_cdk::api::management_canister::http_request::{
    http_request, CanisterHttpRequestArgument, HttpMethod, HttpResponse,
};
use serde_json::Value;

use candid::CandidType;
use serde::{Deserialize, Serialize};

use super::super::utils::{github_api_host, mk_request_headers};

// Define the Closing PR details struct to represent the transformed response
#[derive(Debug, Serialize, Deserialize, CandidType)]
pub struct PrDetailsResponse {
    pub state: Option<String>,
    pub closed_at: Option<String>,
    pub merge_commit_sha: Option<String>,
    pub merged_at: Option<String>,
    pub links: Option<Link>,
    pub merged: Option<bool>,
    pub merged_by: Option<User>,
}

#[derive(Debug, Serialize, Deserialize, CandidType)]
pub struct Link {
    pub issue: IssueLink,
}

#[derive(Debug, Serialize, Deserialize, CandidType, Default)]
pub struct IssueLink {
    pub href: String,
}

#[derive(Debug, Serialize, Deserialize, CandidType)]
pub struct User {
    pub login: String,
    pub id: u64,
}

pub async fn get_merge_details_impl(
    owner: String,
    repo: String,
    pr_nbr: i32,
    github_token: String,
) -> PrDetailsResponse {
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
        url: url.to_string(),
        method: HttpMethod::GET,
        body: None,
        max_response_bytes: None,
        transform: None,
        headers: request_headers,
    };

    // Make the HTTP request and wait for the response
    match http_request(request).await {
        Ok((response,)) => {
            // Parse the response body using the transform function
            let transformed_response = transform_response(response.clone());

            // Print the transformed response for debugging
            println!("Transformed response: {:?}", transformed_response);

            // Return the transformed response
            transformed_response
        }
        Err((rejection_code, message)) => {
            panic!(
                "The http_request resulted in an error. RejectionCode: {:?}, Error: {}",
                rejection_code, message
            );
        }
    }
}

// Define a function to transform the response body
fn transform_response(raw_response: HttpResponse) -> PrDetailsResponse {
    // Deserialize the raw response body into a serde_json::Value
    let parsed_response: Value = serde_json::from_slice(&raw_response.body)
        .unwrap_or_else(|e| panic!("Failed to parse JSON response: {}", e));

    // Print the parsed response for debugging
    println!("Parsed response: {:?}", parsed_response);

    // Extract only the desired fields from the parsed response
    let transformed_response = parsed_response
        .as_object()
        .and_then(|obj| {
            // Extract fields from the object and construct a new object with only the desired fields
            let state = obj
                .get("state")
                .map(|value| value.as_str().map(|s| s.to_string()))
                .flatten();
            let closed_at = obj
                .get("closed_at")
                .map(|value| value.as_str().map(|s| s.to_string()))
                .flatten();
            let merge_commit_sha = obj
                .get("merge_commit_sha")
                .map(|value| value.as_str().map(|s| s.to_string()))
                .flatten();
            let merged_at = obj
                .get("merged_at")
                .map(|value| value.as_str().map(|s| s.to_string()))
                .flatten();
            let links = obj
                .get("_links")
                .and_then(|value| value.get("issue"))
                .and_then(|issue| issue.as_object())
                .map(|issue| Link {
                    issue: IssueLink {
                        href: issue
                            .get("href")
                            .and_then(|href| href.as_str())
                            .unwrap_or_default()
                            .to_string(),
                    },
                })
                .map(Some)
                .unwrap_or_default();
            let merged = obj.get("merged").map(|value| value.as_bool()).flatten();
            let merged_by = obj
                .get("merged_by")
                .map(|value| {
                    value.as_object().map(|user| User {
                        login: user
                            .get("login")
                            .and_then(|login| login.as_str())
                            .unwrap_or_default()
                            .to_string(),
                        id: user
                            .get("id")
                            .and_then(|id| id.as_u64())
                            .unwrap_or_default(),
                    })
                })
                .flatten();

            // Construct the transformed response object
            Some(PrDetailsResponse {
                state,
                closed_at,
                merge_commit_sha,
                merged_at,
                links,
                merged,
                merged_by,
            })
        })
        .unwrap_or_else(|| panic!("Failed to extract fields from parsed response"));

    // Print the transformed response for debugging
    println!("Transformed response: {:?}", transformed_response);

    transformed_response
}
