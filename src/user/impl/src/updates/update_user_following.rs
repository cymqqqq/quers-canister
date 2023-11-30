use crate::{mutate_state, replace_state, RuntimeState};
use crate::guards::caller_is_quers_user;
use ic_cdk_macros::update;
use user_canister::update_user_following::{Args, Response::*,*};

#[update]
fn update_user_following(args: Args) -> Response {
    mutate_state(|state| update_user_following_impl(&args, state))
}

fn update_user_following_impl(args: &Args, state: &mut RuntimeState) -> Response {
    state.data.update_user_following(&args.owner, &args.following);
    Response::Success
}