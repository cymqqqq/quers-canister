use crate::{mutate_state, RuntimeState};
// use crate::guards::caller_is_quers_user;
use ic_cdk_macros::update;
use user_canister::un_follow_operation::*;

#[update]
fn un_follow_operation(args: Args) -> Response {
    mutate_state(|state| un_follow_operation_impl(&args, state))
}

fn un_follow_operation_impl(args: &Args, state: &mut RuntimeState) -> Response {
    state.data.un_follow_operation(&args.to_follow, &args.owner);
    Response::Success
}