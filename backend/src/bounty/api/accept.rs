use super::state::{Contributor, PullRequest, BOUNTY_STATE};

pub fn accept_impl(contributor: Contributor, github_issue_id: String, github_pr_id: String) -> () {
    BOUNTY_STATE.with(|state| {
        if let Some(ref mut bounty_canister) = *state.borrow_mut() {
            let mut issue_exists = false;
            let mut pr_exists = false;

            if let Some(ref mut issue) = bounty_canister.github_issues.get_mut(&github_issue_id) {
                issue_exists = true;
                if !issue.bounty.accepted_prs.contains_key(&github_pr_id) {
                    let pr = PullRequest {
                        id: github_pr_id.clone(),
                        contributor,
                    };
                    issue.bounty.accepted_prs.insert(github_pr_id.clone(), pr);
                    pr_exists = true;
                }
            }

            if !issue_exists {
                // FIXME: change response type to include a proper domain error.
                // The response should be a Result type (Either).
                panic!("Can't accept an issue which does not exist.");
            }

            if !pr_exists {
                // FIXME: change response type to include a proper domain error.
                // The response should be a Result type (Either).
                panic!("Can't accept twice");
            }
        }
    });
}

#[cfg(test)]
mod test_accept {
    use super::*;
    use crate::bounty::api::init::init_impl;
    use candid::Principal;

    #[test]
    fn test_accept() {
        let authority =
            Principal::from_text("t2y5w-qp34w-qixaj-s67wp-syrei-5yqse-xbed6-z5nsd-fszmf-izgt2-lqe")
                .unwrap();
        init_impl(authority);
        let github_issue_id = "input-output-hk/hydra/issues/1370".to_string();
        BOUNTY_STATE.with(|state| {
            let bounty_canister = state.borrow();
            if let Some(ref bounty_canister) = *bounty_canister {
                assert_eq!(
                    bounty_canister
                        .github_issues
                        .get(&github_issue_id)
                        .unwrap()
                        .bounty
                        .accepted_prs
                        .len(),
                    0
                );
            } else {
                panic!("Bounty canister state not initialized");
            }
        });

        let contributor =
            Principal::from_text("t2y5w-qp34w-qixaj-s67wp-syrei-5yqse-xbed6-z5nsd-fszmf-izgt2-lqe")
                .unwrap();
        let github_pr_id = "input-output-hk/hydra/pull/1266".to_string();
        accept_impl(
            Contributor {
                address: contributor,
                crypto_address: "contributor_address".to_string(),
            },
            github_issue_id.clone(),
            github_pr_id.clone(),
        );
        BOUNTY_STATE.with(|state| {
            let bounty_canister = state.borrow();
            if let Some(ref bounty_canister) = *bounty_canister {
                assert_eq!(
                    bounty_canister
                        .github_issues
                        .get(&github_issue_id)
                        .unwrap()
                        .bounty
                        .accepted_prs
                        .len(),
                    1
                );
            } else {
                panic!("Bounty canister state not initialized");
            }
        });
    }
}
