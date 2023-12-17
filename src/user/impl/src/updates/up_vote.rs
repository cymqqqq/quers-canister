use crate::{mutate_state, RuntimeState};
// use crate::guards::caller_is_quers_user;
use ic_cdk_macros::update;
use user_canister::up_vote::*;

#[update]
fn up_vote(args: Args) -> Response {
    mutate_state(|state| up_vote_impl(&args, state))
}

fn up_vote_impl(args: &Args, state: &mut RuntimeState) -> Response {
    state.data.up_vote(&args.question_id);
    Response::Success
}