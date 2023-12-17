use crate::{mutate_state, RuntimeState};
// use crate::guards::caller_is_quers_user;
use ic_cdk_macros::update;
use user_canister::down_vote::*;

#[update]
fn down_vote(args: Args) -> Response {
    mutate_state(|state| down_vote_impl(&args, state))
}

fn down_vote_impl(args: &Args, state: &mut RuntimeState) -> Response {
    state.data.down_vote(&args.question_id);
    Response::Success
}