use candid::CandidType;
use ic_cdk::api::management_canister::http_request::{
    http_request, CanisterHttpRequestArgument, HttpMethod,
};
use serde::{Deserialize, Serialize};

use crate::provider::github::utils::github_api_host;

#[derive(Debug, Serialize, Deserialize, CandidType)]
pub enum IsMergedErr {
    Rejected { error_message: String },
}

pub async fn get_is_merged_impl(
    owner: &str,
    repo: &str,
    pr_nbr: i32,
    cycles: u128,
) -> Result<String, IsMergedErr> {
    // Setup the URL and its query parameters
    let url = format!(
        "https://{}/repos/{}/{}/pulls/{}/merge",
        github_api_host(),
        owner,
        repo,
        pr_nbr
    );

    // Create the request argument
    let request = CanisterHttpRequestArgument {
        url,
        method: HttpMethod::GET,
        body: None,
        max_response_bytes: None,
        transform: None,
        headers: vec![],
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
            IsMergedErr::Rejected { error_message }
        })
}
