use crate::{mutate_state, RuntimeState};
// use crate::guards::caller_is_quers_user;
use ic_cdk_macros::update;
use user_canister::update_name::*;

#[update]
fn update_name(args: Args) -> Response {
    mutate_state(|state| update_name_impl(&args, state))
}

fn update_name_impl(args: &Args, state: &mut RuntimeState) -> Response {
    state.data.update_name(&args.owner, &args.name);
    Response::Success
}