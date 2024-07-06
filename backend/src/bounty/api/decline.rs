use candid::CandidType;
use serde::{Deserialize, Serialize};

use crate::bounty::api::state;
use crate::bounty::api::state::{IssueId, PullRequestId, UserId};

#[derive(Debug, Serialize, Deserialize, CandidType)]
pub enum DeclineError {
    IssueNotFound { github_issue_id: String },
}

pub type DeclineReceipt = Option<DeclineError>;

pub fn decline_impl(
    github_user_id_owner: UserId,
    github_issue_id: IssueId,
    github_pr_id: PullRequestId,
) -> DeclineReceipt {
    state::with_mut(|state| {
        let issue_exists = state.is_issue_existed(&github_issue_id);
        if !issue_exists {
            return Some(DeclineError::IssueNotFound { github_issue_id });
        }
        if let Some(issue) = state.github_issues_mut().get_mut(&github_issue_id) {
            if issue.bounty().accepted_prs().contains_key(&github_pr_id) {
                // TODO: Check contributor it's registered
                // TODO check the issue is not claimed and still open!
                issue.bounty_mut().remove_pull_request(&github_pr_id);
            }
        }

        // TODO check the issue is not closed and has no winner already
        None
    })
}

#[cfg(test)]
mod test_decline {
    use super::*;
    use crate::bounty::api::{
        accept::{accept_impl, AcceptError},
        init::init_impl,
        register_issue::{register_issue_impl, RegisterIssueError},
    };
    use candid::{Nat, Principal};
    use num_bigint::BigUint;

    #[test]
    fn test_decline() {
        let time = 100u64;
        let caller = Principal::anonymous();

        init_impl(time, caller, None);

        let github_issue_id = "input-output-hk/hydra/issues/1370";

        let github_user_id_owner = "prom3th3us";

        let github_user_id_contributor = "cryptoBrew";

        let github_pr_id = "input-output-hk/hydra/pull/1266";

        let bounty_amount: Nat = Nat(BigUint::from(100u32));

        let now = 100u64;

        let r: Option<RegisterIssueError> = register_issue_impl(
            github_user_id_owner.to_string(),
            github_issue_id.to_string(),
            bounty_amount,
            now,
        );
        assert!(r.is_none());

        let r2: Option<AcceptError> = accept_impl(
            github_user_id_contributor.to_string(),
            github_issue_id.to_string(),
            github_pr_id.to_string(),
            now,
        );
        assert!(r2.is_none());

        let r3: Option<DeclineError> = decline_impl(
            github_user_id_owner.to_string(),
            github_issue_id.to_string(),
            github_pr_id.to_string(),
        );
        assert!(r3.is_none());

        state::with(|state| {
            assert!(!state
                .github_issues()
                .get(github_issue_id)
                .map(|issue| issue.bounty())
                .map(|bounty| bounty.accepted_prs())
                .map(|prs| prs.contains_key(github_pr_id))
                .unwrap());
        });
    }
    #[test]
    fn test_decline_twice() {
        let time = 100u64;
        let caller = Principal::anonymous();

        init_impl(time, caller, None);

        let github_issue_id = "input-output-hk/hydra/issues/1370";

        let github_user_id_owner = "prom3th3us";

        let github_user_id_contributor = "cryptoBrew";

        let github_pr_id = "input-output-hk/hydra/pull/1266";

        let bounty_amount: Nat = Nat(BigUint::from(100u32));

        let now = 100u64;

        let r: Option<RegisterIssueError> = register_issue_impl(
            github_user_id_owner.to_string(),
            github_issue_id.to_string(),
            bounty_amount,
            now,
        );
        assert!(r.is_none());

        let r2: Option<AcceptError> = accept_impl(
            github_user_id_contributor.to_string(),
            github_issue_id.to_string(),
            github_pr_id.to_string(),
            now,
        );
        assert!(r2.is_none());

        let r3: Option<DeclineError> = decline_impl(
            github_user_id_owner.to_string(),
            github_issue_id.to_string(),
            github_pr_id.to_string(),
        );
        assert!(r3.is_none());

        let r4: Option<DeclineError> = decline_impl(
            github_user_id_owner.to_string(),
            github_issue_id.to_string(),
            github_pr_id.to_string(),
        );
        assert!(r4.is_none());
    }
}
