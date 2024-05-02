use super::state::{Contributor, BOUNTY_STATE};

pub fn accept_impl(contributor: Contributor) -> () {
    BOUNTY_STATE.with(|state| {
        if let Some(ref mut bounty_canister) = *state.borrow_mut() {
            // Add the contributor to the interested contributors list
            bounty_canister.interested_contributors.push(contributor);
        }
    });
}

#[cfg(test)]
mod test_accept {
    use super::*;
    use crate::api::init::init_impl;
    use candid::Principal;

    #[test]
    fn test_accept() {
        let authority =
            Principal::from_text("t2y5w-qp34w-qixaj-s67wp-syrei-5yqse-xbed6-z5nsd-fszmf-izgt2-lqe")
                .unwrap();
        init_impl(authority, 123);
        BOUNTY_STATE.with(|state| {
            let bounty_canister = state.borrow();
            if let Some(ref bounty_canister) = *bounty_canister {
                assert_eq!(bounty_canister.interested_contributors.len(), 0);
            } else {
                panic!("Bounty canister state not initialized");
            }
        });
        let contributor =
            Principal::from_text("t2y5w-qp34w-qixaj-s67wp-syrei-5yqse-xbed6-z5nsd-fszmf-izgt2-lqe")
                .unwrap();
        accept_impl(Contributor {
            address: contributor,
            crypto_address: "contributor_address".to_string(),
        });
        BOUNTY_STATE.with(|state| {
            let bounty_canister = state.borrow();
            if let Some(ref bounty_canister) = *bounty_canister {
                assert_eq!(bounty_canister.interested_contributors.len(), 1);
            } else {
                panic!("Bounty canister state not initialized");
            }
        });
    }
}
