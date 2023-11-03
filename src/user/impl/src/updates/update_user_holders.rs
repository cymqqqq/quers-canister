use crate::{mutate_state, replace_state, RuntimeState};
use crate::guards::caller_is_quers_user;
use ic_cdk_macros::update;
use user_canister::update_user_holders::{Args, Response::*,*};

#[update]
fn update_user_holders(args: Args) -> Response {
    mutate_state(|state| update_user_holders_impl(&args, state))
}

fn update_user_holders_impl(args: &Args, state: &mut RuntimeState) -> Response {
    state.data.update_user_holders(&args.owner, &args.holders);
    Response::Success
}