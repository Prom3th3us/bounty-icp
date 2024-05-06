use std::collections::HashMap;

use super::state::{BountyState, BOUNTY_STATE};
use candid::Principal;

pub fn init_impl(authority: Principal) -> () {
    BOUNTY_STATE.with(|state| {
        *state.borrow_mut() = Some(BountyState {
            authority,
            github_issues: HashMap::new(),
        });
    });
}

#[cfg(test)]
mod test_init {
    use super::*;

    #[test]
    fn test_init() {
        BOUNTY_STATE.with(|state| {
            let bounty_canister = state.borrow();
            assert!(bounty_canister.is_none());
        });

        let authority =
            Principal::from_text("t2y5w-qp34w-qixaj-s67wp-syrei-5yqse-xbed6-z5nsd-fszmf-izgt2-lqe")
                .unwrap();

        init_impl(authority);
        BOUNTY_STATE.with(|state| {
            let bounty_canister = state.borrow();
            assert!(bounty_canister.is_some());
        });
    }
}
