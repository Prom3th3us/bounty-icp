use std::collections::HashMap;

use candid::{CandidType, Nat};
use serde::{Deserialize, Serialize};

use crate::bounty::api::state;
use crate::bounty::api::state::{Bounty, Issue, IssueId, IssueMetadata, Time};

use super::state::UserId;

#[derive(Debug, Serialize, Deserialize, CandidType)]
pub enum RegisterIssueError {
    UserNotFound { github_user_id: UserId },
}

pub type RegisterIssueReceipt = Option<RegisterIssueError>;

pub fn register_issue_impl(
    github_user_id: UserId,
    github_issue_id: IssueId,
    amount: Nat,
    now: Time,
) -> RegisterIssueReceipt {
    return state::with_mut(|state| {
        // TODO: Check github_issue_id exists on github
        // First if check contributor it's registered
        if !state.is_user_existed(&github_user_id) {
            return Some(RegisterIssueError::UserNotFound { github_user_id });
        }

        if !state.is_issue_existed(&github_issue_id) {
            let github_issue = Issue {
                id: github_issue_id.clone(),
                maintainer: github_user_id.clone(),
                bounty: Bounty {
                    amount: amount,
                    winner: None,
                    accepted_prs: HashMap::new(),
                },
                metadata: IssueMetadata {
                    created_at: now,
                    updated_at: now,
                },
            };
            // TODO check the issue is still open!
            // TODO check the contributor gave allowance >= amount to the canister!
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
    use crate::{
        bounty::api::init::init_impl,
        users::api::register_user::{register_user_impl, RegisterUserError},
    };
    use candid::{Nat, Principal};
    use num_bigint::BigUint;

    #[test]
    fn test_register_issue() {
        let time = 100u64;
        let caller = Principal::anonymous();

        init_impl(time, caller, None);

        let github_issue_id = "input-output-hk/hydra/issues/1370".to_string();

        let github_user_id = "prom3th3us".to_string();

        let bounty_amount: Nat = Nat(BigUint::from(100u32));

        //Register user first, if not registered will fail with UserNotFound
        let ru1: Option<RegisterUserError> = register_user_impl(github_user_id.clone(), time);
        assert!(ru1.is_none());

        let r: Option<RegisterIssueError> = register_issue_impl(
            github_user_id,
            github_issue_id.clone(),
            bounty_amount,
            100u64,
        );

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

        let github_user_id = "prom3th3us".to_string();

        let bounty_amount: Nat = Nat(BigUint::from(100u32));

        //Register user first, if not registered will fail with UserNotFound
        let ru1: Option<RegisterUserError> = register_user_impl(github_user_id.clone(), time);
        assert!(ru1.is_none());

        let now = 100u64;
        let r: Option<RegisterIssueError> = register_issue_impl(
            github_user_id.clone(),
            github_issue_id.clone(),
            bounty_amount.clone(),
            now,
        );

        assert!(r.is_none());

        let r2: Option<RegisterIssueError> = register_issue_impl(
            github_user_id.clone(),
            github_issue_id.clone(),
            bounty_amount.clone(),
            now,
        );

        assert!(r2.is_none());
    }
}
