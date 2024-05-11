use crate::IssueId;

use crate::bounty::api::state;

pub type UnRegisterIssueError = ();

pub type UnRegisterIssueReceipt = Option<UnRegisterIssueError>;

pub fn unregister_issue_impl(github_issue_id: IssueId) -> UnRegisterIssueReceipt {
    return state::with_mut(|state| {
        if state.is_issue_existed(&github_issue_id) {
            // TODO: Check contributor it's registered and github_issue_id exists on github
            // TODO check the issue is claimed, return error if not!
            state.github_issues.remove(&github_issue_id);
        }
        None
    });
}

#[cfg(test)]
mod test_unregister_issue {
    use super::*;
    use crate::bounty::api::init::init_impl;
    use crate::bounty::api::register_issue::RegisterIssueError;
    use crate::bounty::api::state::Contributor;
    use crate::register_issue_impl;
    use candid::{Nat, Principal};
    use num_bigint::BigUint;

    #[test]
    fn test_unregister_issue() {
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
        let r2: Option<UnRegisterIssueError> = unregister_issue_impl(github_issue_id.clone());
        assert!(r2.is_none());

        state::with(|state| {
            assert!(!state.is_issue_existed(&github_issue_id));
        });
    }

    #[test]
    fn test_unregister_issue_twice() {
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
        let r2: Option<UnRegisterIssueError> = unregister_issue_impl(github_issue_id.clone());
        assert!(r2.is_none());

        let r3: Option<UnRegisterIssueError> = unregister_issue_impl(github_issue_id.clone());
        assert!(r3.is_none());
    }
}
