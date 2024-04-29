use ic_cdk::api::management_canister::http_request::{
    http_request, CanisterHttpRequestArgument, HttpMethod, HttpResponse,
};
use serde_json::Value;

use candid::CandidType;
use serde::{Deserialize, Serialize};

use super::super::utils::{github_api_host, mk_request_headers};

// Define the IssueResponse struct to represent the transformed response
#[derive(Debug, Serialize, Deserialize, CandidType)]
pub struct IssueResponse {
    pub state: Option<String>,
    pub login: Option<String>,
    pub id: Option<String>,
    pub milestone_state: Option<String>,
    pub closed_at: Option<String>,
    pub reason: Option<String>,
}

pub async fn get_issue_impl(
    owner: String,
    repo: String,
    issue_nbr: i32,
    github_token: String,
) -> IssueResponse {
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
        url: url.to_string(),
        method: HttpMethod::GET,
        body: None,
        max_response_bytes: None,
        transform: None,
        headers: request_headers,
    };

    // FIXME
    let cycles = 10;

    // Make the HTTP request and wait for the response
    match http_request(request, cycles).await {
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
fn transform_response(raw_response: HttpResponse) -> IssueResponse {
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
            let login = obj
                .get("closed_by")
                .and_then(|closed_by| closed_by.get("login"))
                .and_then(|value| value.as_str().map(|s| s.to_string()));
            let id = obj
                .get("closed_by")
                .and_then(|closed_by| closed_by.get("id"))
                .and_then(|value| value.as_i64().map(|s| s.to_string()));
            //TODO: check why are we trying to catch milestone_state,
            //may "state_reason": "completed" fullfill our needs?
            let milestone_state = obj
                .get("milestone")
                .and_then(|milestone| milestone.get("state"))
                .and_then(|value| value.as_str().map(|s| s.to_string()));
            let closed_at = obj
                .get("closed_at")
                .map(|value| value.as_str().map(|s| s.to_string()))
                .flatten();
            let reason = obj
                .get("state_reason")
                .map(|value| value.as_str().map(|s| s.to_string()))
                .flatten();

            // Construct the transformed response object
            Some(IssueResponse {
                state,
                login,
                id,
                milestone_state,
                closed_at,
                reason
            })
        })
        .unwrap_or_else(|| panic!("Failed to extract fields from parsed response"));

    // Print the transformed response for debugging
    println!("Transformed response: {:?}", transformed_response);

    transformed_response
}
