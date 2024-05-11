use super::state::BOUNTY_STATE;
use crate::IssueId;
pub type UnRegisterIssueError = ();

pub type UnRegisterIssueReceipt = Option<UnRegisterIssueError>;

pub fn unregister_issue_impl(github_issue_id: IssueId) -> UnRegisterIssueReceipt {
    return BOUNTY_STATE.with(|state| {
        if let Some(ref mut bounty_canister) = *state.borrow_mut() {
            let issue_exists = bounty_canister.github_issues.contains_key(&github_issue_id);

            if issue_exists {
                // TODO: Check contributor it's registered and github_issue_id exists on github
                bounty_canister.github_issues.remove(&github_issue_id);
            }
            None
        } else {
            panic!("Bounty canister state not initialized")
        }
    });
}

#[cfg(test)]
mod test_unregister_issue {
    use super::*;
    use crate::bounty::api::init::init_impl;
    use crate::bounty::api::register_issue::RegisterIssueError;
    use crate::bounty::api::state::{Contributor, BOUNTY_STATE};
    use crate::register_issue_impl;
    use candid::{Nat, Principal};
    use num_bigint::BigUint;

    #[test]
    fn test_unregister_issue() {
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
        let r2: Option<UnRegisterIssueError> = unregister_issue_impl(github_issue_id.clone());
        assert!(r2.is_none());

        BOUNTY_STATE.with(|state| {
            let bounty_canister = state.borrow();
            if let Some(ref bounty_canister) = *bounty_canister {
                assert!(bounty_canister
                    .github_issues
                    .get(&github_issue_id)
                    .is_none());
            } else {
                panic!("Bounty canister state not initialized");
            }
        });
    }

    #[test]
    fn test_unregister_issue_twice() {
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
        let r2: Option<UnRegisterIssueError> = unregister_issue_impl(github_issue_id.clone());
        assert!(r2.is_none());

        let r3: Option<UnRegisterIssueError> = unregister_issue_impl(github_issue_id.clone());
        assert!(r3.is_none());
    }
}
