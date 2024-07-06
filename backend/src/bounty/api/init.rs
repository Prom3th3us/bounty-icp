use candid::Principal;

use crate::bounty::api::state;
use crate::bounty::api::state::{InitArgs, Time};

pub fn init_impl(time: Time, caller: Principal, args: Option<InitArgs>) {
    state::with_mut(|state| state.init(time, caller, args));
}

#[cfg(test)]
mod test_init {
    use super::*;

    #[test]
    fn test_init() {
        let time = 100u64;
        let caller = Principal::anonymous();

        state::with(|state| assert!(state.is_canister_custodian(caller).is_err()));

        init_impl(time, caller, None);

        state::with(|state| assert!(state.is_canister_custodian(caller).is_ok()));
    }
}
