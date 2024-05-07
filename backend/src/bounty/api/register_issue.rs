use super::state::{Bounty, Contributor, Issue, PullRequest, BOUNTY_STATE};

use candid::CandidType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, CandidType)]
pub enum RegisterIssueError {
    CantRegisterIssueTwice,
}

pub type RegisterIssueReceipt = Option<RegisterIssueError>;

pub fn register_issue_impl(github_issue: Issue) -> RegisterIssueReceipt {
    print!("Registering issue: {:?}", github_issue);
    return BOUNTY_STATE.with(|state| {
        if let Some(ref mut bounty_canister) = *state.borrow_mut() {
            let mut issue_exists = false;

            if let Some(ref mut issue) = bounty_canister.github_issues.get_mut(&github_issue.id) {
                issue_exists = true;
            }

            if issue_exists {
                Some(RegisterIssueError::CantRegisterIssueTwice)
            } else {
                bounty_canister
                    .github_issues
                    .insert(github_issue.id.clone(), github_issue);
                None
            }
        } else {
            panic!("Bounty canister state not initialized")
        }
    });
}

#[cfg(test)]
mod test_register_issue {
    use super::*;
    use crate::bounty::api::init::init_impl;
    use candid::Principal;

    #[test]
    fn test_register_issue() {
        let authority =
            Principal::from_text("t2y5w-qp34w-qixaj-s67wp-syrei-5yqse-xbed6-z5nsd-fszmf-izgt2-lqe")
                .unwrap();
        init_impl(authority);
        let github_issue_id = "input-output-hk/hydra/issues/1370".to_string();
        let r: Option<RegisterIssueError> = register_issue_impl(Issue {
            id: github_issue_id,
            maintainer: Contributor {
                address: Principal::anonymous(),
                crypto_address: "0x1234".to_string(),
            },
            bounty: Bounty {
                amount: 100,
                winner: None,
                accepted_prs: Default::default(),
            },
        });
        assert!(r.is_none());
    }
    #[test]
    fn test_cant_register_issue_twice() {
        let authority =
            Principal::from_text("t2y5w-qp34w-qixaj-s67wp-syrei-5yqse-xbed6-z5nsd-fszmf-izgt2-lqe")
                .unwrap();
        init_impl(authority);
        let github_issue_id = "input-output-hk/hydra/issues/1370".to_string();
        let r1: Option<RegisterIssueError> = register_issue_impl(Issue {
            id: github_issue_id.clone(),
            maintainer: Contributor {
                address: Principal::anonymous(),
                crypto_address: "0x1234".to_string(),
            },
            bounty: Bounty {
                amount: 100,
                winner: None,
                accepted_prs: Default::default(),
            },
        });
        assert!(r1.is_none());
        let r2: Option<RegisterIssueError> = register_issue_impl(Issue {
            id: github_issue_id.clone(),
            maintainer: Contributor {
                address: Principal::anonymous(),
                crypto_address: "0x1234".to_string(),
            },
            bounty: Bounty {
                amount: 100,
                winner: None,
                //not sure if next line  is correct
                accepted_prs: Default::default(),
            },
        });
        assert!(r2.is_some());
        assert!(matches!(
            r2,
            Some(RegisterIssueError::CantRegisterIssueTwice)
        ));
    }
}
