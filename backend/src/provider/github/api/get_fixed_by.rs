use ic_cdk::api::management_canister::http_request::{
    http_request, CanisterHttpRequestArgument, HttpMethod,
};

use crate::provider::github::utils::github_host;
use candid::CandidType;
use serde::{Deserialize, Serialize};

use std::collections::HashSet;

use regex::Regex;

#[derive(Debug, Serialize, Deserialize, CandidType)]
pub enum GetFixedByError {
    IssueNotFound { issue_nbr: i32 },
}

// curl https://github.com/input-output-hk/hydra/issues/1370
pub async fn get_fixed_by_impl(owner: String, repo: String, issue_nbr: i32) -> Result<String, GetFixedByError> {
    // Setup the URL and its query parameters
    let url = format!(
        "https://{}/{}/{}/issues/{}",
        github_host(),
        owner,
        repo,
        issue_nbr
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
            //We need to decode that Vec<u8> that is the body into readable text.
            //To do this, we:
            //  1. Call `String::from_utf8()` on response.body
            //  3. We use a switch to explicitly call out both cases of decoding the Blob into ?Text
            let str_body = String::from_utf8(response.body)
                .expect("Transformed response is not UTF-8 encoded.");

            let fixed_by_lines = str_body.lines().fold(HashSet::new(), |mut set, line| {
                if line.contains("Fixed by") {
                    set.insert(line.to_string());
                }
                set
            });

            let result = fixed_by_lines
                .into_iter()
                .collect::<Vec<String>>()
                .join(", ");

            if let Some(pull_request) = extract_pull_request(&result) {
                return Ok(remove_github_prefix(&pull_request));
            }
            return Err(GetFixedByError::IssueNotFound{issue_nbr});
            
        }
        Err((rejection_code, message)) => {
            panic!(
                "The http_request resulted in an error. RejectionCode: {:?}, Error: {}",
                rejection_code, message
            );
        }
    }
}

fn remove_github_prefix(url: &str) -> String {
    url.replace("https://github.com/", "")
}

fn extract_pull_request(html: &str) -> Option<String> {
    // Define a regular expression pattern to match the href attribute
    let re = Regex::new(r#"<a\s+[^>]*?href="(.*?)"[^>]*?>"#).unwrap();

    // Extract the href attribute from the HTML string
    if let Some(captures) = re.captures(html) {
        if let Some(href) = captures.get(1) {
            return Some(href.as_str().to_string());
        }
    }
    None
}
