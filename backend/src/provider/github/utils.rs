use ic_cdk::api::management_canister::http_request::HttpHeader;

pub fn github_api_host() -> &'static str {
    "api.github.com"
}

pub fn github_host() -> &'static str {
    "github.com"
}

pub fn mk_request_headers(github_token: &str) -> Vec<HttpHeader> {
    vec![
        HttpHeader {
            name: "Authorization".to_string(),
            value: format!("Bearer {}", github_token),
        },
        HttpHeader {
            name: "Accept".to_string(),
            value: "application/vnd.github+json".to_string(),
        },
        HttpHeader {
            name: "X-GitHub-Api-Version".to_string(),
            value: "2022-11-28".to_string(),
        },
    ]
}
