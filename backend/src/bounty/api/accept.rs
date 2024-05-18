use candid::CandidType;
use serde::{Deserialize, Serialize};

use crate::bounty::api::state;
use crate::bounty::api::state::{
    IssueId, PullRequest, PullRequestId, PullRequestMetadata, Time, UserId,
};

use crate::register_user_impl;
use crate::users::api::register_user::RegisterUserError;

#[derive(Debug, Serialize, Deserialize, CandidType)]
pub enum AcceptError {
    IssueNotFound { github_issue_id: IssueId },
    UserNotFound { github_user_id: UserId },
}

pub type AcceptReceipt = Option<AcceptError>;

pub fn accept_impl(
    github_user_id: UserId,
    github_issue_id: IssueId,
    github_pr_id: PullRequestId,
    now: Time,
) -> AcceptReceipt {
    return state::with_mut(|state| {
        // First if check contributor it's registered
        if !state.is_user_existed(&github_user_id) {
            return Some(AcceptError::UserNotFound { github_user_id });
        }

        if let Some(ref mut issue) = state.github_issues.get_mut(&github_issue_id) {
            if !issue.bounty.accepted_prs.contains_key(&github_pr_id) {
                let pr = PullRequest {
                    id: github_pr_id.clone(),
                    contributor: github_user_id.clone(),
                    metadata: PullRequestMetadata {
                        accepted_at: now,
                        updated_at: now,
                    },
                };

                // TODO check the issue is not claimed and still open!
                issue.bounty.accepted_prs.insert(github_pr_id.clone(), pr);
            }
        }

        let issue_exists = state.is_issue_existed(&github_issue_id);
        if !issue_exists {
            return Some(AcceptError::IssueNotFound { github_issue_id });
        }

        // TODO check the issue is not closed and has no winner already
        None
    });
}

#[cfg(test)]
mod test_accept {
    use super::*;
    use crate::bounty::api::{
        init::init_impl,
        register_issue::{register_issue_impl, RegisterIssueError},
    };
    use candid::{Nat, Principal};
    use num_bigint::BigUint;

    #[test]
    fn test_accept() {
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
        let r: Option<RegisterIssueError> =
            register_issue_impl(github_user_id, github_issue_id.clone(), bounty_amount, now);

        assert!(r.is_none());

        state::with(|state| {
            assert_eq!(
                state
                    .github_issues
                    .get(&github_issue_id)
                    .unwrap()
                    .bounty
                    .accepted_prs
                    .len(),
                0
            );
        });

        let github_user_id = "prom3th3us".to_string();
        let github_pr_id = "input-output-hk/hydra/pull/1266".to_string();
        let now = 100u64;
        accept_impl(
            github_user_id,
            github_issue_id.clone(),
            github_pr_id.clone(),
            now,
        );
        state::with(|state| {
            assert_eq!(
                state
                    .github_issues
                    .get(&github_issue_id)
                    .unwrap()
                    .bounty
                    .accepted_prs
                    .len(),
                1
            );
        });
    }
}
