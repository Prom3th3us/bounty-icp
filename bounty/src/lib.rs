use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, CandidType)]
pub struct Contributor {
    pub address: Principal,
    pub crypto_address: String,
}

#[derive(Debug, Serialize, Deserialize, CandidType)]
struct BountyState {
    authority: Principal,
    github_issue_id: i32,
    interested_contributors: Vec<Contributor>,
    claimed: bool,
}

// Define thread-local storage for the bounty canister state
thread_local! {
    static BOUNTY_STATE: std::cell::RefCell<Option<BountyState>> = std::cell::RefCell::new(None);
}

#[ic_cdk::init]
fn init(authority: Principal, github_issue_id: i32) -> () {
    BOUNTY_STATE.with(|state| {
        *state.borrow_mut() = Some(BountyState {
            authority,
            github_issue_id,
            interested_contributors: Vec::new(),
            claimed: false,
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

        init(authority, 123);
        BOUNTY_STATE.with(|state| {
            let bounty_canister = state.borrow();
            assert!(bounty_canister.is_some());
        });
    }
}

#[ic_cdk::update]
fn accept(contributor: Contributor) -> () {
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
    #[test]
    fn test_accept() {
        let authority =
            Principal::from_text("t2y5w-qp34w-qixaj-s67wp-syrei-5yqse-xbed6-z5nsd-fszmf-izgt2-lqe")
                .unwrap();
        init(authority, 123);
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
        accept(Contributor {
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

#[ic_cdk::update]
async fn healthcheck() -> String {
    return "OK".to_string();
}
