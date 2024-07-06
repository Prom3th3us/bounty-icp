use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

use crate::bounty::api::state;
use crate::bounty::api::state::UserId;

#[derive(Debug, Serialize, Deserialize, CandidType, PartialEq)]
pub enum UpsertUserWalletError {
    UserNotFound,
}

pub type UpsertUserWalletReceipt = Result<(), UpsertUserWalletError>;

pub fn upsert_user_wallet_impl(
    github_user_id: UserId,
    wallet: Option<Principal>,
) -> UpsertUserWalletReceipt {
    state::with_mut(|state| {
        state
            .github_known_users_mut()
            .get_mut(&github_user_id)
            .ok_or(UpsertUserWalletError::UserNotFound)
            .map(|github_user| github_user.set_wallet(wallet))
    })
}

#[cfg(test)]
mod test_upsert_user_wallet {
    use super::*;
    use crate::{
        bounty::api::init::init_impl,
        users::api::register_user::{register_user_impl, RegisterUserError},
    };
    use candid::Principal;

    #[test]
    fn test_upsert_user_wallet() {
        let time = 100u64;
        let caller = Principal::anonymous();

        init_impl(time, caller, None);

        let github_user_id = "prom3th3us";

        state::with(|state| {
            assert!(!state.is_user_existed(github_user_id));
        });

        let r: Option<RegisterUserError> = register_user_impl(github_user_id.to_string(), time);
        assert!(r.is_none());

        let wallet = Principal::anonymous();

        state::with(|state| {
            assert!(state
                .github_known_users()
                .get(github_user_id)
                .and_then(|gh_user| gh_user.wallet())
                .is_none());
        });

        let r: UpsertUserWalletReceipt =
            upsert_user_wallet_impl(github_user_id.to_string(), Some(wallet));
        assert!(r.is_ok());

        state::with(|state| {
            assert_eq!(
                state
                    .github_known_users()
                    .get(github_user_id)
                    .and_then(|gh_user| gh_user.wallet())
                    .unwrap(),
                wallet
            );
        });

        let r2: UpsertUserWalletReceipt = upsert_user_wallet_impl(github_user_id.to_string(), None);
        assert!(r2.is_ok());

        state::with(|state| {
            assert!(state
                .github_known_users()
                .get(github_user_id)
                .and_then(|gh_user| gh_user.wallet())
                .is_none());
        });
    }
}
