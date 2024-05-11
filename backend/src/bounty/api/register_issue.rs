use std::collections::HashMap;

use super::state::{IssueId, Time};
use super::state::{Bounty, Contributor, Issue, BOUNTY_STATE};

use candid::Nat;

pub type RegisterIssueError = ();

pub type RegisterIssueReceipt = Option<RegisterIssueError>;

pub fn register_issue_impl(
    contributor: Contributor,
    github_issue_id: IssueId,
    amount: Nat,
    now: Time
) -> RegisterIssueReceipt {
    return BOUNTY_STATE.with(|state| {
        if let Some(ref mut bounty_canister) = *state.borrow_mut() {
            let issue_exists = bounty_canister.github_issues.contains_key(&github_issue_id);
            if !issue_exists {
                let github_issue = Issue {
                    id: github_issue_id.clone(),
                    maintainer: contributor,
                    bounty: Bounty {
                        amount: amount,
                        winner: None,
                        accepted_prs: HashMap::new(),
                    },
                    created_at: now,
                    updated_at: now,
                    
                };
                // TODO: Check contributor it's registered and github_issue_id exists on github
                // TODO check the issue is still open!
                bounty_canister
                    .github_issues
                    .insert(github_issue_id.clone(), github_issue);
            }
            None
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
            register_issue_impl(contributor, github_issue_id.clone(), bounty_amount, 100u64);

        assert!(r.is_none());

        BOUNTY_STATE.with(|state| {
            let bounty_canister = state.borrow();
            if let Some(ref bounty_canister) = *bounty_canister {
                assert!(bounty_canister
                    .github_issues
                    .get(&github_issue_id)
                    .is_some());
            } else {
                panic!("Bounty canister state not initialized");
            }
        });
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

        let now = 100u64;
        let r: Option<RegisterIssueError> = register_issue_impl(
            contributor.clone(),
            github_issue_id.clone(),
            bounty_amount.clone(),
            now
        );

        assert!(r.is_none());

        let r2: Option<RegisterIssueError> = register_issue_impl(
            contributor.clone(),
            github_issue_id.clone(),
            bounty_amount.clone(),
            now
        );

        assert!(r2.is_none());
    }
}
