use crate::bounty::api::state::{self, GitHubUser};

use crate::bounty::api::state::{Time, UserId};

pub type RegisterUserError = ();

pub type RegisterUserReceipt = Option<RegisterUserError>;

pub fn register_user_impl(github_user_id: UserId, time: Time) -> RegisterUserReceipt {
    return state::with_mut(|state| {
        if !state.is_user_existed(&github_user_id) {
            let github_user = GitHubUser {
                user_id: github_user_id.clone(),
                wallet: None,
                created_at: time,
                updated_at: time,
            };

            // TODO: Check user it's registered on github
            state
                .github_known_users_mut()
                .insert(github_user_id, github_user);
        }
        None
    });
}

#[cfg(test)]
mod test_register_user {
    use super::*;
    use crate::bounty::api::init::init_impl;
    use candid::Principal;

    #[test]
    fn test_register_user() {
        let time = 100u64;
        let caller = Principal::anonymous();

        init_impl(time, caller, None);

        let github_user_id = "prom3th3us".to_string();

        state::with(|state| {
            assert!(!state.is_user_existed(&github_user_id));
        });

        let r: Option<RegisterUserError> = register_user_impl(github_user_id.clone(), time);
        assert!(r.is_none());

        state::with(|state| {
            assert!(state.is_user_existed(&github_user_id));
        });
    }

    #[test]
    fn test_cant_register_user_twice() {
        let time = 100u64;
        let caller = Principal::anonymous();

        init_impl(time, caller, None);

        let github_user_id = "prom3th3us".to_string();

        state::with(|state| {
            assert!(!state.is_user_existed(&github_user_id));
        });

        let r: Option<RegisterUserError> = register_user_impl(github_user_id.clone(), time);
        assert!(r.is_none());

        state::with(|state| {
            assert!(state.is_user_existed(&github_user_id));
        });

        let r2: Option<RegisterUserError> = register_user_impl(github_user_id.clone(), time);
        assert!(r2.is_none());
    }
}
