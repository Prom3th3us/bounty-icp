use candid::{CandidType, Principal};
use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::bounty::api::state;
use crate::bounty::api::state::{IssueId, PullRequestId};
use crate::provider::github::api::get_fixed_by::FixedByErr;
use crate::provider::github::api::get_is_merged::IsMergedErr;
use crate::provider::github::api::get_issue::IssueErr;
use crate::provider::github::api::get_merged_details::MergeDetailsErr;

use crate::provider::github::api::get_user_exists::UserExistsError;
use crate::provider::github::api::{
    get_issue::IssueResponse, get_merged_details::PrDetailsResponse,
};
use crate::provider::github::client::IGithubClient;

#[derive(Debug, Serialize, Deserialize, CandidType)]
pub enum ClaimError {
    IssueNotFound { github_issue_id: String },
    PRNotAccepted { github_pr_id: String },
    PRNotMerged { github_pr_id: String },
}

// TODO: remove this after finishing draft impl.
#[cfg(test)]
pub struct GithubClientMock {
    principal: Principal,
}

// TODO: remove this after finishing draft impl.
#[cfg(test)]
#[async_trait::async_trait]
impl IGithubClient for GithubClientMock {
    async fn get_issue(&self, issue_nbr: i32) -> Result<IssueResponse, IssueErr> {
        todo!()
    }
    async fn get_fixed_by(&self, issue_nbr: i32) -> Result<String, FixedByErr> {
        todo!()
    }
    async fn get_is_merged(&self, pr_nbr: i32) -> Result<String, IsMergedErr> {
        todo!()
    }
    async fn get_merged_details(&self, pr_nbr: i32) -> Result<PrDetailsResponse, MergeDetailsErr> {
        todo!()
    }
    async fn get_user_exists(&self, user_id: String) -> Result<String, UserExistsError> {
        todo!()
    }
}

// FIXME: remove this annotation after finishing draft impl.
#[cfg(test)]
pub async fn claim_impl(
    github_client: &dyn IGithubClient,
    github_issue_id: IssueId,
    github_pr_id: PullRequestId,
) -> Option<ClaimError> {
    use crate::bounty::api::state::Issue;
    use crate::provider::github::api::get_merged_details::MergeDetailsErr;

    let issue_opt: Option<Issue> = state::with(|state| {
        return state.github_issues.get(&github_issue_id).map(|i| i.clone());
    });

    match issue_opt {
        None => {
            return Some(ClaimError::IssueNotFound {
                github_issue_id: github_issue_id.clone(),
            })
        }
        Some(issue) => match issue.bounty.accepted_prs.get(&github_pr_id) {
            None => Some(ClaimError::PRNotAccepted {
                github_pr_id: github_pr_id.clone(),
            }),
            Some(pull_request) => {
                // TODO: unify GitHub errors
                // FIXME: remove unwraps
                let pr_response: Result<PrDetailsResponse, MergeDetailsErr> = github_client
                    .get_merged_details(extract_pull_number(&github_pr_id).unwrap())
                    .await;
                let issue_response: Result<IssueResponse, IssueErr> = github_client
                    .get_issue(extract_issue_number(&github_issue_id).unwrap())
                    .await;

                todo!()
            }
        },
    }
}

#[cfg(test)]
mod test_claim {
    use crate::bounty::api::accept::accept_impl;
    use crate::bounty::api::init::init_impl;
    use crate::bounty::api::state;
    use candid::Principal;
    use futures::executor::block_on;

    use super::{claim_impl, ClaimError, GithubClientMock};

    fn accept_contributor(principal: &str, github_issue_id: &str, github_pr_id: &str) {
        let now = 100u64;
        let github_user_id = "prom3th3us";
        accept_impl(
            github_user_id.to_string(),
            github_issue_id.to_string(),
            github_pr_id.to_string(),
            now,
        );
    }

    #[test]
    fn test_accept() {
        let github_issue_id = "input-output-hk/hydra/issues/1370";
        let github_pr_id_1 = "input-output-hk/hydra/pull/1";
        let github_pr_id_2 = "input-output-hk/hydra/pull/2";

        let time = 100u64;
        let caller = Principal::anonymous();

        init_impl(time, caller, None);

        accept_contributor(
            "mxzaz-hqaaa-aaaar-qaada-cai",
            github_issue_id,
            github_pr_id_1,
        );
        accept_contributor(
            "n5wcd-faaaa-aaaar-qaaea-cai",
            github_issue_id,
            github_pr_id_2,
        );

        let github_client = GithubClientMock { principal: caller };

        let result = block_on(claim_impl(
            &github_client,
            github_issue_id.to_string(),
            github_pr_id_2.to_string(),
        ));

        match result {
            None => assert!(true),
            Some(claim_error) => match claim_error {
                ClaimError::PRNotAccepted { github_pr_id: _ } => assert!(false),
                ClaimError::PRNotMerged { github_pr_id: _ } => assert!(false),
                ClaimError::IssueNotFound { github_issue_id: _ } => assert!(false),
            },
        }

        state::with(|state| {
            assert_eq!(
                state
                    .github_issues
                    .get(&github_issue_id.to_string())
                    .unwrap()
                    .bounty
                    .winner,
                Some(github_pr_id_2.to_string())
            );
        });
    }
}

#[cfg(test)]
fn extract_regex<T: std::str::FromStr>(regex: &str, str: &str) -> Option<T> {
    match Regex::new(regex) {
        Err(err) => {
            let error_message = format!("Error (regex): {}", err);
            print!("{}", error_message);
            None
        }
        Ok(re) => {
            if let Some(captures) = re.captures(str) {
                if let Some(number) = captures.get(1) {
                    return number.as_str().parse().ok();
                }
            }
            None
        }
    }
}

#[cfg(test)]
fn extract_pull_number(url: &str) -> Option<i32> {
    return extract_regex(r"/pull/(\d+)", url);
}

#[cfg(test)]
fn extract_issue_number(url: &str) -> Option<i32> {
    return extract_regex(r"/issues/(\d+)", url);
}
