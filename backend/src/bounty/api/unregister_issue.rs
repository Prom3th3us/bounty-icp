use super::state::{Bounty, Contributor, Issue, PullRequest, BOUNTY_STATE};

use crate::bounty::api::register_issue::RegisterIssueError;
use crate::register_issue_impl;
use candid::CandidType;
use candid::Nat;
use num_bigint::BigUint;
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
        let authority = Principal::anonymous();

        init_impl(authority);

        let github_issue_id = "input-output-hk/hydra/issues/1370".to_string();

        let contributor = Contributor {
            address: Principal::anonymous(),
            crypto_address: "0x1234".to_string(),
        };

        let bounty_amount: Nat = Nat(BigUint::from(100u32));

        let r: Option<RegisterIssueError> = register_issue_impl(
            contributor.clone(),
            github_issue_id.clone(),
            bounty_amount.clone(),
        );

        assert!(r.is_none());
        let r2: Option<UnRegisterIssueError> = unregister_issue_impl(github_issue_id.clone());
        assert!(r2.is_none());

        BOUNTY_STATE.with(|state| {
            let bounty_canister = state.borrow();
            if let Some(ref bounty_canister) = *bounty_canister {
                assert!(bounty_canister
                    .github_issues
                    .get(&github_issue_id)
                    .is_none());
            } else {
                panic!("Bounty canister state not initialized");
            }
        });
    }
    #[test]
    fn test_cant_unregister_a_non_existent_issue() {
        let authority = Principal::anonymous();
        init_impl(authority);
        let github_issue_id = "input-output-hk/hydra/issues/1370".to_string();
        let r: Option<UnRegisterIssueError> = unregister_issue_impl(github_issue_id.clone());
        assert!(r.is_some());
        assert!(matches!(r, Some(UnRegisterIssueError::IssueNotFound)));
    }

    #[test]
    fn test_unregister_issue_twice() {
        let authority = Principal::anonymous();

        init_impl(authority);

        let github_issue_id = "input-output-hk/hydra/issues/1370".to_string();

        let contributor = Contributor {
            address: Principal::anonymous(),
            crypto_address: "0x1234".to_string(),
        };

        let bounty_amount: Nat = Nat(BigUint::from(100u32));

        let r: Option<RegisterIssueError> = register_issue_impl(
            contributor.clone(),
            github_issue_id.clone(),
            bounty_amount.clone(),
        );

        assert!(r.is_none());
        let r2: Option<UnRegisterIssueError> = unregister_issue_impl(github_issue_id.clone());
        assert!(r2.is_none());

        let r3: Option<UnRegisterIssueError> = unregister_issue_impl(github_issue_id.clone());
        assert!(r3.is_none());
    }
}
