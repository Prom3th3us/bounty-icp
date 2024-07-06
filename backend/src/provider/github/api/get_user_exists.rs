use candid::CandidType;
use ic_cdk::api::management_canister::http_request::{
    http_request, CanisterHttpRequestArgument, HttpMethod,
};
use serde::{Deserialize, Serialize};

use crate::provider::github::utils::{github_api_host, mk_request_headers};

#[derive(Debug, Serialize, Deserialize, CandidType)]
pub enum UserExistsError {
    Rejected { error_message: String },
}

//https://api.github.com/users/daguis
pub async fn get_user_exists_impl(
    github_token: &str,
    user_id: String,
    cycles: u128,
) -> Result<String, UserExistsError> {
    // Setup the URL and its query parameters
    let url = format!("https://{}/users/{}", github_api_host(), user_id);

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
        .map(|(response,)| response.status.to_string())
        .map_err(|(rejection_code, message)| {
            let error_message = format!(
                "The http_request resulted in an error. RejectionCode: {:?}, Error: {}",
                rejection_code, message
            );
            UserExistsError::Rejected { error_message }
        })
}
