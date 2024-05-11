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
