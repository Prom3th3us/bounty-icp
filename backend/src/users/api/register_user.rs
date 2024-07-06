use crate::bounty::api::state::{self, GitHubUser, Time, UserId};

pub type RegisterUserError = ();

pub type RegisterUserReceipt = Option<RegisterUserError>;

pub fn register_user_impl(github_user_id: UserId, time: Time) -> RegisterUserReceipt {
    state::with_mut(|state| {
        if !state.is_user_existed(&github_user_id) {
            let github_user = GitHubUser::new(&github_user_id, None, time, time);
            // TODO: Check user it's registered on github
            state.insert_github_user(github_user_id, github_user);
        }
        None
    })
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

        let github_user_id = "prom3th3us";

        state::with(|state| {
            assert!(!state.is_user_existed(github_user_id));
        });

        let r: Option<RegisterUserError> = register_user_impl(github_user_id.to_string(), time);
        assert!(r.is_none());

        state::with(|state| {
            assert!(state.is_user_existed(github_user_id));
        });
    }

    #[test]
    fn test_cant_register_user_twice() {
        let time = 100u64;
        let caller = Principal::anonymous();

        init_impl(time, caller, None);

        let github_user_id = "prom3th3us";

        state::with(|state| {
            assert!(!state.is_user_existed(github_user_id));
        });

        let r: Option<RegisterUserError> = register_user_impl(github_user_id.to_string(), time);
        assert!(r.is_none());

        state::with(|state| {
            assert!(state.is_user_existed(github_user_id));
        });

        let r2: Option<RegisterUserError> = register_user_impl(github_user_id.to_string(), time);
        assert!(r2.is_none());
    }
}
