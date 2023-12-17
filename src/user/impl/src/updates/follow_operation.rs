use crate::{mutate_state, RuntimeState};
// use crate::guards::caller_is_quers_user;
use ic_cdk_macros::update;
use user_canister::follow_operation::*;

#[update]
fn follow_operation(args: Args) -> Response {
    mutate_state(|state| follow_operation_impl(&args, state))
}

fn follow_operation_impl(args: &Args, state: &mut RuntimeState) -> Response {
    state.data.follow_operation(&args.owner, &args.to_follow);
    Response::Success
}