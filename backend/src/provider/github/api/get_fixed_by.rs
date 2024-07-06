use std::collections::HashSet;

use candid::CandidType;
use ic_cdk::api::management_canister::http_request::{
    http_request, CanisterHttpRequestArgument, HttpMethod,
};
use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::provider::github::utils::github_host;

#[derive(Debug, Serialize, Deserialize, CandidType)]
pub enum FixedByErr {
    IssueNotFound { github_issue_id: String },
    Rejected { error_message: String },
}

pub async fn get_fixed_by_impl(
    owner: &str,
    repo: &str,
    issue_nbr: i32,
    cycles: u128,
) -> Result<String, FixedByErr> {
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
        .map_err(|(rejection_code, message)| {
            let error_message = format!(
                "The http_request resulted in an error. RejectionCode: {:?}, Error: {}",
                rejection_code, message
            );
            FixedByErr::Rejected { error_message }
        })
        .and_then(|(response,)| {
            // We need to decode that Vec<u8> that is the body into readable text
            let str_body = String::from_utf8(response.body).map_err(|_| FixedByErr::Rejected {
                error_message: "Transformed response is not UTF-8 encoded.".to_string(),
            })?;

            let fixed_by_lines = str_body.lines().fold(HashSet::new(), |mut set, line| {
                if line.contains("Fixed by") {
                    set.insert(line);
                }
                set
            });

            let result = fixed_by_lines.into_iter().collect::<Vec<&str>>().join(", ");

            extract_pull_request(&result)
                .map(remove_github_prefix)
                .ok_or({
                    let issue_not_found = format!(
                        "https://{}/{}/{}/issue/{}",
                        github_host(),
                        owner,
                        repo,
                        issue_nbr
                    );
                    FixedByErr::IssueNotFound {
                        github_issue_id: issue_not_found,
                    }
                })
        })
}

fn remove_github_prefix(url: &str) -> String {
    url.replace("https://github.com/", "")
}

// TODO: use extract_regex
fn extract_pull_request(html: &str) -> Option<&str> {
    // Define a regular expression pattern to match the href attribute
    Regex::new(r#"<a\s+[^>]*?href="(.*?)"[^>]*?>"#)
        .ok()
        .and_then(|re| re.captures(html))
        // Extract the href attribute from the HTML string
        .and_then(|captures| captures.get(1))
        .map(|href| href.as_str())
}
