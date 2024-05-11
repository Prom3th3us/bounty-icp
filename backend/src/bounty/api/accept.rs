use super::state::{Contributor, PullRequest, BOUNTY_STATE};

use ic_cdk::api::time;
use candid::CandidType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, CandidType)]
pub enum AcceptError {
    IssueNotFound { github_issue_id: String },
    CantAcceptedTwice,
}

pub type AcceptReceipt = Option<AcceptError>;

pub fn accept_impl(
    contributor: Contributor,
    github_issue_id: String,
    github_pr_id: String,
) -> AcceptReceipt {
    return BOUNTY_STATE.with(|state| {
        if let Some(ref mut bounty_canister) = *state.borrow_mut() {
            let mut issue_exists = false;
            let mut pr_exists = false;

            if let Some(ref mut issue) = bounty_canister.github_issues.get_mut(&github_issue_id) {
                issue_exists = true;
                if !issue.bounty.accepted_prs.contains_key(&github_pr_id) {
                    let now = time();
                    let pr = PullRequest {
                        id: github_pr_id.clone(),
                        contributor,
                        accepted_at: now,
                        updated_at: now
                    };
                    issue.bounty.accepted_prs.insert(github_pr_id.clone(), pr);
                    pr_exists = true;
                }
            }

            if !issue_exists {
                Some(AcceptError::IssueNotFound { github_issue_id });
            }

            if !pr_exists {
                Some(AcceptError::CantAcceptedTwice);
            }

            None
        } else {
            panic!("Bounty canister state not initialized")
        }
    });
}

#[cfg(test)]
mod test_accept {
    use super::*;
    use crate::bounty::api::init::init_impl;
    use candid::Principal;

    #[test]
    fn test_accept() {
        let authority =
            Principal::from_text("t2y5w-qp34w-qixaj-s67wp-syrei-5yqse-xbed6-z5nsd-fszmf-izgt2-lqe")
                .unwrap();
        init_impl(authority);
        let github_issue_id = "input-output-hk/hydra/issues/1370".to_string();
        BOUNTY_STATE.with(|state| {
            let bounty_canister = state.borrow();
            if let Some(ref bounty_canister) = *bounty_canister {
                assert_eq!(
                    bounty_canister
                        .github_issues
                        .get(&github_issue_id)
                        .unwrap()
                        .bounty
                        .accepted_prs
                        .len(),
                    0
                );
            } else {
                panic!("Bounty canister state not initialized");
            }
        });

        let contributor =
            Principal::from_text("t2y5w-qp34w-qixaj-s67wp-syrei-5yqse-xbed6-z5nsd-fszmf-izgt2-lqe")
                .unwrap();
        let github_pr_id = "input-output-hk/hydra/pull/1266".to_string();
        accept_impl(
            Contributor {
                address: contributor,
                crypto_address: "contributor_address".to_string(),
            },
            github_issue_id.clone(),
            github_pr_id.clone(),
        );
        BOUNTY_STATE.with(|state| {
            let bounty_canister = state.borrow();
            if let Some(ref bounty_canister) = *bounty_canister {
                assert_eq!(
                    bounty_canister
                        .github_issues
                        .get(&github_issue_id)
                        .unwrap()
                        .bounty
                        .accepted_prs
                        .len(),
                    1
                );
            } else {
                panic!("Bounty canister state not initialized");
            }
        });
    }
}
