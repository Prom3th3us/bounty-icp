use crate::bounty::api::state::{BOUNTY_STATE, Contributor};
use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

use crate::provider::github::api::{get_issue::IssueResponse, get_merged_details::PrDetailsResponse};
use crate::provider::github::client::IGithubClient;

#[derive(Debug, Serialize, Deserialize, CandidType)]
pub enum ClaimError {
    PRNotAccepted { github_pr_id: i32 },
    PRNotMerged { github_pr_id: i32 },
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
    async fn get_issue(&self, issue_nbr: i32) -> IssueResponse { todo!() }
    async fn get_fixed_by(&self, issue_nbr: i32) -> String { todo!() }
    async fn get_is_merged(&self, pr_nbr: i32) -> String { todo!() }
    async fn get_merged_details(&self, pr_nbr: i32) -> PrDetailsResponse { todo!() }
}

// FIXME: remove this annotation after finishing draft impl.
#[cfg(test)]
pub async fn claim_impl(
    github_client: &dyn IGithubClient,
    github_issue_id: i32,
    github_pr_id: i32,
    github_token: &str,
) -> Option<ClaimError> {
    let contributor_opt: Option<Contributor> = BOUNTY_STATE.with(|state| {
        match state.borrow().as_ref() {
            Some(bounty_state) => {
                // Access the interested_contributors HashMap from the BountyState
                bounty_state.interested_contributors.get(&github_pr_id).cloned()
            }
            None => panic!("Bounty canister state not initialized")
        }
    });
    match contributor_opt {
        None => Some(ClaimError::PRNotAccepted{github_pr_id})
        , Some(contributor) => {
            let issue = github_client.get_issue(github_issue_id).await;
            todo!()
        }
    }
}

#[cfg(test)]
mod test_claim {
    use crate::bounty::api::accept::accept_impl;
    use crate::bounty::api::init::init_impl;
    use crate::bounty::api::state::{Contributor, BOUNTY_STATE};
    use candid::Principal;
    use futures::executor::block_on;

    use super::{claim_impl, ClaimError, GithubClientMock};

    fn accept_contributor(principal: &str, crypto_address: &str, github_pr_id: i32) {
        accept_impl(
            Contributor {
                address: Principal::from_text(principal).unwrap(),
                crypto_address: crypto_address.to_string(),
            },
            github_pr_id,
        );
    }

    #[test]
    fn test_accept() {
        let github_issue_id = 123;

        let authority = Principal::from_text("rdmx6-jaaaa-aaaaa-aaadq-cai").unwrap();

        init_impl(authority, github_issue_id);

        accept_contributor("mxzaz-hqaaa-aaaar-qaada-cai", "contributor_address_1", 1);
        accept_contributor("n5wcd-faaaa-aaaar-qaaea-cai", "contributor_address_2", 2);

        let github_client = GithubClientMock{principal: authority};

        let result = block_on(claim_impl(&github_client, github_issue_id, 2, "GithubToken"));

        match result {
            None => assert!(true),
            Some(claim_error) => match claim_error {
                ClaimError::PRNotAccepted { github_pr_id: _ } => assert!(false),
                ClaimError::PRNotMerged { github_pr_id: _ } => assert!(false),
            },
        }

        BOUNTY_STATE.with(|state| {
            let bounty_canister = state.borrow();
            if let Some(ref bounty_canister) = *bounty_canister {
                assert_eq!(bounty_canister.winner.unwrap(), 2);
            } else {
                panic!("Bounty canister state not initialized");
            }
        });
    }
}
