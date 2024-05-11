use super::state::{Contributor, IssueId, PullRequest, PullRequestId, Time, BOUNTY_STATE};

use candid::CandidType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, CandidType)]
pub enum AcceptError {
    IssueNotFound { github_issue_id: String },
}

pub type AcceptReceipt = Option<AcceptError>;

pub fn accept_impl(
    contributor: Contributor,
    github_issue_id: IssueId,
    github_pr_id: PullRequestId,
    now: Time
) -> AcceptReceipt {
    return BOUNTY_STATE.with(|state| {
        if let Some(ref mut bounty_canister) = *state.borrow_mut() {
            if let Some(ref mut issue) = bounty_canister.github_issues.get_mut(&github_issue_id) {
                if !issue.bounty.accepted_prs.contains_key(&github_pr_id) {
                    let pr = PullRequest {
                        id: github_pr_id.clone(),
                        contributor,
                        accepted_at: now,
                        updated_at: now
                    };
                    // TODO: Check contributor it's registered and github_issue_id exists on github
                    issue.bounty.accepted_prs.insert(github_pr_id.clone(), pr);
                }
            }
            
            let issue_exists = bounty_canister.github_issues.contains_key(&github_issue_id);
            if !issue_exists {
                return Some(AcceptError::IssueNotFound { github_issue_id });
            }

            // TODO check the issue is not closed and has no winner already
            None
        } else {
            panic!("Bounty canister state not initialized")
        }
    });
}

#[cfg(test)]
mod test_accept {
    use super::*;
    use crate::bounty::api::{init::init_impl, register_issue::{register_issue_impl, RegisterIssueError}};
    use candid::{Nat, Principal};
    use num_bigint::BigUint;

    #[test]
    fn test_accept() {
        let authority = Principal::anonymous();
        init_impl(authority);
        let github_issue_id = "input-output-hk/hydra/issues/1370".to_string();

        let contributor = Contributor {
            address: Principal::anonymous(),
            crypto_address: "0x1234".to_string(),
        };

        let bounty_amount: Nat = Nat(BigUint::from(100u32));

        let now = 100u64;
        let r: Option<RegisterIssueError> =
            register_issue_impl(contributor, github_issue_id.clone(), bounty_amount, now);

        assert!(r.is_none());
        
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

        let contributor = Principal::anonymous();
        let github_pr_id = "input-output-hk/hydra/pull/1266".to_string();
        let now = 100u64;
        accept_impl(
            Contributor {
                address: contributor,
                crypto_address: "contributor_address".to_string(),
            },
            github_issue_id.clone(),
            github_pr_id.clone(),
            now
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
