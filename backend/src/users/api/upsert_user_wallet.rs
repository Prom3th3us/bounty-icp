use candid::Principal;

use crate::bounty::api::state;

use crate::bounty::api::state::UserId;

pub type UpsertUserWalletError = ();

pub type UpsertUserWalletReceipt = Option<UpsertUserWalletError>;

pub fn upsert_user_wallet_impl(
    github_user_id: UserId,
    wallet: Option<Principal>,
) -> UpsertUserWalletReceipt {
    return state::with_mut(|state| {
        // TODO: return user not found error
        if let Some(github_user) = state.github_known_users_mut().get_mut(&github_user_id) {
            github_user.wallet = wallet;
        }
        None
    });
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

        let github_user_id = "prom3th3us".to_string();

        state::with(|state| {
            assert!(!state.is_user_existed(&github_user_id));
        });

        let r: Option<RegisterUserError> = register_user_impl(github_user_id.clone(), time);
        assert!(r.is_none());

        let wallet = Principal::anonymous();

        state::with(|state| {
            assert!(state
                .github_known_users
                .get(&github_user_id)
                .unwrap()
                .wallet
                .is_none());
        });

        let r: Option<UpsertUserWalletError> =
            upsert_user_wallet_impl(github_user_id.clone(), Some(wallet));
        assert!(r.is_none());

        state::with(|state| {
            assert_eq!(
                state
                    .github_known_users
                    .get(&github_user_id)
                    .unwrap()
                    .wallet
                    .unwrap(),
                wallet
            );
        });

        let r2: Option<UpsertUserWalletError> =
            upsert_user_wallet_impl(github_user_id.clone(), None);
        assert!(r2.is_none());

        state::with(|state| {
            assert!(state
                .github_known_users
                .get(&github_user_id)
                .unwrap()
                .wallet
                .is_none());
        });
    }
}
