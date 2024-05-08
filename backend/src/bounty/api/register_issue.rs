use super::state::{Bounty, Contributor, Issue, PullRequest, BOUNTY_STATE};

use candid::CandidType;
use candid::Nat;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, CandidType)]
pub enum RegisterIssueError {
    CantRegisterIssueTwice,
}

pub type RegisterIssueReceipt = Option<RegisterIssueError>;

pub fn register_issue_impl(
    contributor: Contributor,
    github_issue_id: String,
    amount: Nat,
) -> RegisterIssueReceipt {
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
    use candid::{Nat, Principal};
    use num_bigint::BigUint;

    #[test]
    fn test_register_issue() {
        let authority = Principal::anonymous();

        init_impl(authority);

        let github_issue_id = "input-output-hk/hydra/issues/1370".to_string();

        let contributor = Contributor {
            address: Principal::anonymous(),
            crypto_address: "0x1234".to_string(),
        };

        let bounty_amount: Nat = Nat(BigUint::from(100u32));

        let r: Option<RegisterIssueError> =
            register_issue_impl(contributor, github_issue_id.clone(), bounty_amount);

        assert!(r.is_none());
    }
    #[test]
    fn test_cant_register_issue_twice() {
        let authority = Principal::anonymous();

        init_impl(authority);

        let github_issue_id = "input-output-hk/hydra/issues/1370".to_string();

        let contributor = Contributor {
            address: Principal::anonymous(),
            crypto_address: "0x1234".to_string(),
        };

        let bounty_amount: Nat = Nat(BigUint::from(100u32));

        let r: Option<RegisterIssueError> =
            register_issue_impl(contributor, github_issue_id.clone(), bounty_amount);

        assert!(r.is_none());

        let r2: Option<RegisterIssueError> =
            register_issue_impl(contributor, github_issue_id.clone(), bounty_amount);

        assert!(r2.is_none());
    }
}
