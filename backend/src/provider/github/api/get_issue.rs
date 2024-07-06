use candid::CandidType;
use ic_cdk::api::management_canister::http_request::{
    http_request, CanisterHttpRequestArgument, HttpMethod, HttpResponse,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::provider::github;
use github::utils::{github_api_host, mk_request_headers};

#[derive(Debug, Serialize, Deserialize, CandidType)]
pub enum IssueErr {
    Rejected { error_message: String },
}

#[derive(Debug, Serialize, Deserialize, CandidType)]
pub struct IssueResponse {
    state: Option<String>,
    login: Option<String>,
    id: Option<String>,
    milestone_state: Option<String>,
    closed_at: Option<String>,
    reason: Option<String>,
}

pub async fn get_issue_impl(
    owner: &str,
    repo: &str,
    github_token: &str,
    issue_nbr: i32,
    cycles: u128,
) -> Result<IssueResponse, IssueErr> {
    // Setup the URL and its query parameters
    let url = format!(
        "https://{}/repos/{}/{}/issues/{}",
        github_api_host(),
        owner,
        repo,
        issue_nbr
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
            IssueErr::Rejected { error_message }
        })
        .and_then(|(response,)| transform_response(response))
}

// Define a function to transform the response body
fn transform_response(raw_response: HttpResponse) -> Result<IssueResponse, IssueErr> {
    // Deserialize the raw response body into a serde_json::Value
    let parsed_response: Value =
        serde_json::from_slice(&raw_response.body).map_err(|_| IssueErr::Rejected {
            error_message: "Failed to parse JSON response: {}".to_string(),
        })?;

    // Print the parsed response for debugging
    println!("Parsed response: {:?}", parsed_response);

    let obj = parsed_response.as_object().ok_or(IssueErr::Rejected {
        error_message: "Failed to map parsed JSON response as object: {}".to_string(),
    })?;

    // Extract fields from the parsed object
    // and construct a new object with only the desired fields
    let state = obj
        .get("state")
        .and_then(|value| value.as_str())
        .map(|s| s.to_string());
    let login = obj
        .get("closed_by")
        .and_then(|closed_by| closed_by.get("login"))
        .and_then(|value| value.as_str())
        .map(|s| s.to_string());
    let id = obj
        .get("closed_by")
        .and_then(|closed_by| closed_by.get("id"))
        .and_then(|value| value.as_i64())
        .map(|s| s.to_string());
    //TODO: check why are we trying to catch milestone_state,
    //may "state_reason": "completed" fullfill our needs?
    let milestone_state = obj
        .get("milestone")
        .and_then(|milestone| milestone.get("state"))
        .and_then(|value| value.as_str())
        .map(|s| s.to_string());
    let closed_at = obj
        .get("closed_at")
        .and_then(|value| value.as_str())
        .map(|s| s.to_string());
    let reason = obj
        .get("state_reason")
        .and_then(|value| value.as_str())
        .map(|s| s.to_string());

    // Construct the transformed response object
    let transformed_response = IssueResponse {
        state,
        login,
        id,
        milestone_state,
        closed_at,
        reason,
    };

    // Print the transformed response for debugging
    println!("Transformed response: {:?}", transformed_response);

    Ok(transformed_response)
}
