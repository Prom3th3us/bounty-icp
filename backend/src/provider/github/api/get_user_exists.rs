use ic_cdk::api::management_canister::http_request::{
    http_request, CanisterHttpRequestArgument, HttpMethod,
};

use super::super::utils::{github_api_host, mk_request_headers};
use candid::CandidType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, CandidType)]
pub enum UserExistsError {
    Rejected { error_message: String },
}

//https://api.github.com/users/daguis
pub async fn get_user_exists_impl(
    owner: String,
    repo: String,
    user_id: String,
    github_token: String,
) -> Result<String, UserExistsError> {
    // Setup the URL and its query parameters
    let url = format!("https://{}/users/{}", github_api_host(), user_id);

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
    let cycles = 2_500_000_000;

    // Make the HTTP request and wait for the response
    match http_request(request, cycles).await {
        Ok((response,)) => Ok(response.status.to_string()),
        Err((rejection_code, message)) => {
            let error_message = format!(
                "The http_request resulted in an error. RejectionCode: {:?}, Error: {}",
                rejection_code, message
            );
            Err(UserExistsError::Rejected { error_message })
        }
    }
}
