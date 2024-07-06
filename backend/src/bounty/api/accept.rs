use candid::CandidType;
use serde::{Deserialize, Serialize};

use crate::bounty::api::state;
use state::{IssueId, PullRequest, PullRequestId, PullRequestMetadata, Time, UserId};

#[derive(Debug, Serialize, Deserialize, CandidType)]
pub enum AcceptError {
    IssueNotFound { github_issue_id: String },
}

pub type AcceptReceipt = Option<AcceptError>;

pub fn accept_impl(
    github_user_id: UserId,
    github_issue_id: IssueId,
    github_pr_id: PullRequestId,
    now: Time,
) -> AcceptReceipt {
    state::with_mut(|state| {
        if let Some(issue) = state.github_issues_mut().get_mut(&github_issue_id) {
            if !issue.bounty().accepted_prs().contains_key(&github_pr_id) {
                let pr = PullRequest::new(
                    &github_pr_id,
                    &github_user_id,
                    PullRequestMetadata::new(now, now),
                );
                // TODO: Check contributor it's registered
                // TODO check the issue is not claimed and still open!
                issue.bounty_mut().insert_pull_request(github_pr_id, pr);
            }
        }

        let issue_exists = state.is_issue_existed(&github_issue_id);
        if !issue_exists {
            return Some(AcceptError::IssueNotFound { github_issue_id });
        }

        // TODO check the issue is not closed and has no winner already
        None
    })
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

        let github_issue_id = "input-output-hk/hydra/issues/1370";

        let github_user_id = "prom3th3us";

        let bounty_amount: Nat = Nat(BigUint::from(100u32));

        let now = 100u64;
        let r: Option<RegisterIssueError> = register_issue_impl(
            github_user_id.to_string(),
            github_issue_id.to_string(),
            bounty_amount,
            now,
        );

        assert!(r.is_none());

        state::with(|state| {
            assert_eq!(
                state
                    .github_issues()
                    .get(github_issue_id)
                    .map(|issue| issue.bounty())
                    .map(|bounty| bounty.accepted_prs())
                    .map(|prs| prs.len())
                    .unwrap(),
                0
            );
        });

        let github_user_id = "prom3th3us";
        let github_pr_id = "input-output-hk/hydra/pull/1266";
        let now = 100u64;
        accept_impl(
            github_user_id.to_string(),
            github_issue_id.to_string(),
            github_pr_id.to_string(),
            now,
        );
        state::with(|state| {
            assert_eq!(
                state
                    .github_issues()
                    .get(github_issue_id)
                    .map(|issue| issue.bounty())
                    .map(|bounty| bounty.accepted_prs())
                    .map(|prs| prs.len())
                    .unwrap(),
                1
            );
        });
    }
}
