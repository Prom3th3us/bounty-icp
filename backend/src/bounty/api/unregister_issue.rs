use crate::bounty::api::state;
use state::{IssueId, UserId};

pub type UnRegisterIssueError = ();

pub type UnRegisterIssueReceipt = Option<UnRegisterIssueError>;

pub fn unregister_issue_impl(
    github_user_id: UserId,
    github_issue_id: IssueId,
) -> UnRegisterIssueReceipt {
    state::with_mut(|state| {
        if state.is_issue_existed(&github_issue_id) {
            // TODO: Check contributor it's registered
            // TODO check the issue is claimed, return error if not!
            state.remove_github_issue(&github_issue_id);
        }
        None
    })
}

#[cfg(test)]
mod test_unregister_issue {
    use super::*;
    use crate::bounty::api::init::init_impl;
    use crate::bounty::api::register_issue::RegisterIssueError;
    use crate::register_issue_impl;
    use candid::{Nat, Principal};
    use num_bigint::BigUint;

    #[test]
    fn test_unregister_issue() {
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
        let r2: Option<UnRegisterIssueError> =
            unregister_issue_impl(github_user_id.to_string(), github_issue_id.to_string());
        assert!(r2.is_none());

        state::with(|state| {
            assert!(!state.is_issue_existed(github_issue_id));
        });
    }

    #[test]
    fn test_unregister_issue_twice() {
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
        let r2: Option<UnRegisterIssueError> =
            unregister_issue_impl(github_user_id.to_string(), github_issue_id.to_string());
        assert!(r2.is_none());

        let r3: Option<UnRegisterIssueError> =
            unregister_issue_impl(github_user_id.to_string(), github_issue_id.to_string());
        assert!(r3.is_none());
    }
}
