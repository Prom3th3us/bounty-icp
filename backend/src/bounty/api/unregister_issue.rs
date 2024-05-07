use super::state::{Bounty, Contributor, Issue, PullRequest, BOUNTY_STATE};

use crate::bounty::api::register_issue::RegisterIssueError;
use crate::register_issue_impl;
use candid::CandidType;
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize, CandidType)]
pub enum UnRegisterIssueError {
    IssueNotFound,
}

pub type UnRegisterIssueReceipt = Option<UnRegisterIssueError>;

pub fn unregister_issue_impl(github_issue: String) -> UnRegisterIssueReceipt {
    print!("Unregistering issue: {:?}", github_issue);
    return BOUNTY_STATE.with(|state| {
        if let Some(ref mut bounty_canister) = *state.borrow_mut() {
            let mut issue_exists = false;

            if let Some(ref mut issue) = bounty_canister.github_issues.get_mut(&github_issue) {
                issue_exists = true;
            }

            if issue_exists {
                bounty_canister.github_issues.remove(&github_issue);
                None
            } else {
                Some(UnRegisterIssueError::IssueNotFound)
            }
        } else {
            panic!("Bounty canister state not initialized")
        }
    });
}

#[cfg(test)]
mod test_unregister_issue {
    use super::*;
    use crate::bounty::api::init::init_impl;
    use candid::Principal;

    #[test]
    fn test_unregister_issue() {
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
        let r2: Option<UnRegisterIssueError> = unregister_issue_impl(github_issue_id.clone());
        assert!(r2.is_none());
    }
    #[test]
    fn test_cant_unregister_a_non_existent_issue() {
        let authority =
            Principal::from_text("t2y5w-qp34w-qixaj-s67wp-syrei-5yqse-xbed6-z5nsd-fszmf-izgt2-lqe")
                .unwrap();
        init_impl(authority);
        let github_issue_id = "input-output-hk/hydra/issues/1370".to_string();
        let r: Option<UnRegisterIssueError> = unregister_issue_impl(github_issue_id.clone());
        assert!(r.is_some());
        assert!(matches!(r, Some(UnRegisterIssueError::IssueNotFound)));
    }
}
