use std::collections::HashMap;

use candid::Nat;

use crate::bounty::api::state;
use crate::bounty::api::state::{Bounty, Contributor, Issue, IssueMetadata, IssueId, Time};

pub type RegisterIssueError = ();

pub type RegisterIssueReceipt = Option<RegisterIssueError>;

pub fn register_issue_impl(
    contributor: Contributor,
    github_issue_id: IssueId,
    amount: Nat,
    now: Time,
) -> RegisterIssueReceipt {
    return state::with_mut(|state| {
        if !state.is_issue_existed(&github_issue_id) {
            let github_issue = Issue {
                id: github_issue_id.clone(),
                maintainer: contributor.clone(),
                bounty: Bounty {
                    amount: amount,
                    winner: None,
                    accepted_prs: HashMap::new(),
                },
                metadata: IssueMetadata {
                    created_at: now,
                    updated_at: now,
                }
            };
            // TODO: Check contributor it's registered and github_issue_id exists on github
            // TODO check the issue is still open!
            state
                .github_issues
                .insert(github_issue_id.clone(), github_issue);
        }
        None
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
        let time = 100u64;
        let caller = Principal::anonymous();

        init_impl(time, caller, None);

        let github_issue_id = "input-output-hk/hydra/issues/1370".to_string();

        let contributor = Contributor {
            address: Principal::anonymous(),
        };

        let bounty_amount: Nat = Nat(BigUint::from(100u32));

        let r: Option<RegisterIssueError> =
            register_issue_impl(contributor, github_issue_id.clone(), bounty_amount, 100u64);

        assert!(r.is_none());

        state::with(|state| {
            assert!(state.is_issue_existed(&github_issue_id));
        });
    }
    #[test]
    fn test_cant_register_issue_twice() {
        let time = 100u64;
        let caller = Principal::anonymous();

        init_impl(time, caller, None);

        let github_issue_id = "input-output-hk/hydra/issues/1370".to_string();

        let contributor = Contributor {
            address: Principal::anonymous(),
        };

        let bounty_amount: Nat = Nat(BigUint::from(100u32));

        let now = 100u64;
        let r: Option<RegisterIssueError> = register_issue_impl(
            contributor.clone(),
            github_issue_id.clone(),
            bounty_amount.clone(),
            now,
        );

        assert!(r.is_none());

        let r2: Option<RegisterIssueError> = register_issue_impl(
            contributor.clone(),
            github_issue_id.clone(),
            bounty_amount.clone(),
            now,
        );

        assert!(r2.is_none());
    }
}
