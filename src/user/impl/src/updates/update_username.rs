use crate::{mutate_state, RuntimeState};
// use crate::guards::caller_is_quers_user;
use ic_cdk_macros::update;
use user_canister::update_username::*;

#[update]
fn update_username(args: Args) -> Response {
    mutate_state(|state| update_username_impl(&args, state))
}

fn update_username_impl(args: &Args, state: &mut RuntimeState) -> Response {
    state.data.update_username(&args.owner, &args.username);
    Response::Success
}