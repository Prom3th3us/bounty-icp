use crate::api::state::BOUNTY_STATE;
use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, CandidType)]
pub enum ClaimError {
    PRNotAccepted { github_pr_id: i32 },
    PRNotMerged { github_pr_id: i32 },
}

// TODO: remove this after finishing draft impl.
#[cfg(test)]
pub struct GithubClient {
    principal: Principal,
}

// TODO: remove this after finishing draft impl.
#[cfg(test)]
impl GithubClient {
    pub fn new(principal: Principal) -> Self {
        GithubClient { principal }
    }

    pub async fn get_issue(&self, issue_nbr: i32) -> String {
        todo!()
    }
    pub async fn get_fixed_by(&self, issue_nbr: i32) -> String {
        todo!()
    }
    pub async fn get_is_merged(&self, pr_nbr: i32) -> String {
        todo!()
    }
    pub async fn get_merged_details(&self, pr_nbr: i32) -> String {
        todo!()
    }
}

// FIXME: remove this annotation after finishing draft impl.
#[cfg(test)]
pub async fn claim_impl(
    github_client: GithubClient,
    github_issue_id: i32,
    github_token: &str,
) -> Option<ClaimError> {
    todo!()
}

#[cfg(test)]
mod test_claim {
    use crate::api::accept::accept_impl;
    use crate::api::init::init_impl;
    use crate::api::state::{Contributor, BOUNTY_STATE};
    use candid::Principal;
    use futures::executor::block_on;

    use super::{claim_impl, ClaimError, GithubClient};

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

        let github_client = GithubClient::new(authority);

        let result = block_on(claim_impl(github_client, 2, "GithubToken"));

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
