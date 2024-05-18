use candid::CandidType;
use serde::{Deserialize, Serialize};

use crate::bounty::api::state;
use crate::bounty::api::state::{
    IssueId, PullRequest, PullRequestId, PullRequestMetadata, Time, UserId,
};
use crate::users::api::register_user::RegisterUserError;

#[derive(Debug, Serialize, Deserialize, CandidType)]
pub enum DeclineError {
    IssueNotFound { github_issue_id: IssueId },
    UserNotFound { github_user_id: UserId },
}

pub type DeclineReceipt = Option<DeclineError>;

pub fn decline_impl(
    github_user_id_owner: UserId,
    github_issue_id: IssueId,
    github_pr_id: PullRequestId,
) -> DeclineReceipt {
    return state::with_mut(|state| {
        // First if check owner it's registered
        if !state.is_user_existed(&github_user_id_owner) {
            return Some(DeclineError::UserNotFound {
                github_user_id: github_user_id_owner,
            });
        }
        let issue_exists = state.is_issue_existed(&github_issue_id);
        if !issue_exists {
            return Some(DeclineError::IssueNotFound { github_issue_id });
        }
        if let Some(ref mut issue) = state.github_issues.get_mut(&github_issue_id) {
            if issue
                .bounty
                .accepted_prs
                .contains_key(&github_pr_id.clone())
            {
                // TODO check the issue is not claimed and still open!
                issue.bounty.accepted_prs.remove(&github_pr_id);
            }
        }

        // TODO check the issue is not closed and has no winner already
        None
    });
}

#[cfg(test)]
mod test_decline {
    use super::*;
    use crate::{
        bounty::api::{
            accept::{accept_impl, AcceptError},
            init::init_impl,
            register_issue::{register_issue_impl, RegisterIssueError},
        },
        users::api::register_user::register_user_impl,
    };
    use candid::{Nat, Principal};
    use num_bigint::BigUint;

    #[test]
    fn test_decline() {
        let time = 100u64;
        let caller = Principal::anonymous();

        init_impl(time, caller, None);

        let github_issue_id = "input-output-hk/hydra/issues/1370".to_string();

        let github_user_id_owner = "prom3th3us".to_string();

        let github_user_id_contributor = "cryptoBrew".to_string();

        let github_pr_id = "input-output-hk/hydra/pull/1266".to_string();

        let bounty_amount: Nat = Nat(BigUint::from(100u32));

        let now = 100u64;

        //Register github_user_id_owner first, if not registered will fail with UserNotFound
        let ru1: Option<RegisterUserError> = register_user_impl(github_user_id_owner.clone(), time);
        assert!(ru1.is_none());

        //Register github_user_id_contributor first, if not registered will fail with UserNotFound
        let ru2: Option<RegisterUserError> =
            register_user_impl(github_user_id_contributor.clone(), time);
        assert!(ru2.is_none());

        let r: Option<RegisterIssueError> = register_issue_impl(
            github_user_id_owner.clone(),
            github_issue_id.clone(),
            bounty_amount,
            now,
        );
        assert!(r.is_none());

        let r2: Option<AcceptError> = accept_impl(
            github_user_id_contributor,
            github_issue_id.clone(),
            github_pr_id.clone(),
            now,
        );
        assert!(r2.is_none());

        let r3: Option<DeclineError> = decline_impl(
            github_user_id_owner.clone(),
            github_issue_id.clone(),
            github_pr_id.clone(),
        );
        assert!(r3.is_none());

        state::with(|state| {
            assert_eq!(
                state
                    .github_issues
                    .get(&github_issue_id)
                    .unwrap()
                    .bounty
                    .accepted_prs
                    .contains_key(&github_pr_id),
                false,
            );
        });
    }
    #[test]
    fn test_decline_twice() {
        let time = 100u64;
        let caller = Principal::anonymous();

        init_impl(time, caller, None);

        let github_issue_id = "input-output-hk/hydra/issues/1370".to_string();

        let github_user_id_owner = "prom3th3us".to_string();

        let github_user_id_contributor = "cryptoBrew".to_string();

        let github_pr_id = "input-output-hk/hydra/pull/1266".to_string();

        let bounty_amount: Nat = Nat(BigUint::from(100u32));

        let now = 100u64;

        //Register github_user_id_owner first, if not registered will fail with UserNotFound
        let ru1: Option<RegisterUserError> = register_user_impl(github_user_id_owner.clone(), time);
        assert!(ru1.is_none());

        //Register github_user_id_contributor first, if not registered will fail with UserNotFound
        let ru2: Option<RegisterUserError> =
            register_user_impl(github_user_id_contributor.clone(), time);
        assert!(ru2.is_none());

        let r: Option<RegisterIssueError> = register_issue_impl(
            github_user_id_owner.clone(),
            github_issue_id.clone(),
            bounty_amount,
            now,
        );
        assert!(r.is_none());

        let r2: Option<AcceptError> = accept_impl(
            github_user_id_contributor,
            github_issue_id.clone(),
            github_pr_id.clone(),
            now,
        );
        assert!(r2.is_none());

        let r3: Option<DeclineError> = decline_impl(
            github_user_id_owner.clone(),
            github_issue_id.clone(),
            github_pr_id.clone(),
        );
        assert!(r3.is_none());

        let r4: Option<DeclineError> = decline_impl(
            github_user_id_owner.clone(),
            github_issue_id.clone(),
            github_pr_id.clone(),
        );
        assert!(r4.is_none());
    }
}
