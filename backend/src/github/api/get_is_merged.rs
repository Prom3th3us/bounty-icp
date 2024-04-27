use ic_cdk::api::management_canister::http_request::{
    http_request, CanisterHttpRequestArgument, HttpMethod,
};

use super::super::utils::github_api_host;

//https://api.github.com/repos/input-output-hk/hydra/pulls/1266/merge
pub async fn get_is_merged_impl(owner: String, repo: String, pr_nbr: i32) -> String {
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

    // Make the HTTP request and wait for the response
    match http_request(request).await {
        Ok((response,)) => {
            //We need to decode that Vec<u8> that is the body into readable text.
            //To do this, we:
            //  1. Call `String::from_utf8()` on response.body
            //  3. We use a switch to explicitly call out both cases of decoding the Blob into ?Text
            let str_body = String::from_utf8(response.body)
                .expect("Transformed response is not UTF-8 encoded.");

            return str_body;
        }
        Err((rejection_code, message)) => {
            panic!(
                "The http_request resulted in an error. RejectionCode: {:?}, Error: {}",
                rejection_code, message
            );
        }
    }
}


