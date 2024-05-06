use ic_cdk::api::management_canister::http_request::{
    http_request, CanisterHttpRequestArgument, HttpMethod,
};

use super::super::utils::github_api_host;
use candid::CandidType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, CandidType)]
pub enum IsMergedErr {
    Rejected { error_message: String },
}

//https://api.github.com/repos/input-output-hk/hydra/pulls/1266/merge
pub async fn get_is_merged_impl(owner: String, repo: String, pr_nbr: i32) -> Result<String, IsMergedErr> {
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
        url: url.to_string(),
        method: HttpMethod::GET,
        body: None,
        max_response_bytes: None,
        transform: None,
        headers: vec![],
    };

    // FIXME
    let cycles = 2_500_000_000;

    // Make the HTTP request and wait for the response
    match http_request(request, cycles).await {
        Ok((response,)) => {
            Ok(response.status.to_string())
        }
        Err((rejection_code, message)) => {
            let error_message = format!(
                "The http_request resulted in an error. RejectionCode: {:?}, Error: {}",
                rejection_code, message
            );
            Err(IsMergedErr::Rejected{error_message})
        }
    }
}
